# Pointer from Shared Reference

Consider the following Rust program:

```rust
extern "C" { fn foo(p: *const u64) -> u64; }

pub extern "C" fn bar(x: &u64) -> u64 {
    let y = *x;
    let z = unsafe { foo(x) };
    z+y+*x
}
```

* Bar takes a shared reference to `u64`, thus it cannot mutate.
* We read the referee into `y`, then call `foo()` and deref `x` again.
* The compiler is free to reuse the value of `y` in the return statement for `*x`

This is what the program compiles to:

```asm
example::bar:
        pushq   %rbx
        movq    (%rdi), %rbx
        callq   *foo@GOTPCREL(%rip)
        leaq    (%rax,%rbx,2), %rax
        popq    %rbx
        retq
```

The instruction `leaq (%rax,%rbx,2), %rax` adds 2x *x as it was loaded before
the call to the return value of the call.

We can opt out of the immutability guarantee by wrapping our value in an
[`UnsafeCell`](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html#):


```rust
extern crate core;
use core::cell::UnsafeCell;

extern "C" { fn foo(p: *const u64) -> u64; }

pub extern "C" fn bar(x: &UnsafeCell<u64>) -> u64 {
    let y = unsafe{*x.get()};
    let z = unsafe { foo(x.get()) };
    z+y+unsafe{*x.get()}
}
```

The API for `UnsafeCell` does not allow us to obtain a shared reference to the
contents, but we can obtain a raw pointer with `get()`.

```asm
example::bar:
        pushq   %r14
        pushq   %rbx
        pushq   %rax
        movq    %rdi, %rbx
        movq    (%rdi), %r14
        callq   *foo@GOTPCREL(%rip)
        addq    %r14, %rax
        addq    (%rbx), %rax
        addq    $8, %rsp
        popq    %rbx
        popq    %r14
        retq
```

In the generated machine code we see that the reference is fetched from memroy twice:

* `movq    (%rdi), %r14`
* `addq    (%rbx), %rax`

You must still apply proper synchronization to avoid data races if the referee
is accessed concurrently!
