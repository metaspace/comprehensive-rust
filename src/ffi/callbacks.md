# Calling Rust from C

To register a callback with an external C library, we must specify the calling convention of the callback as "C" (remember "C" is the default for `extern`):

```rust,ignore
extern fn callback(a: i32) {
    println!("I'm called from C with value {0}", a);
}
```

We then pass the function as a function pointer to the external library:

```rust,ignore
#[link(name = "extlib")]
extern {
   fn register_callback(cb: extern fn(i32)) -> i32;
   fn trigger_callback();
}

fn main() {
    unsafe {
        register_callback(callback);
        trigger_callback(); // Triggers the callback.
    }
}
```

The C equivalent might look something like this:

```c
typedef void (*rust_callback)(int32_t);
rust_callback cb;

int32_t register_callback(rust_callback callback) {
    cb = callback;
    return 1;
}

void trigger_callback() {
  cb(7); // Will call callback(7) in Rust.
}
```

