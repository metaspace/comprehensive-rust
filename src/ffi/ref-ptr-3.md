# Reference from Pointer

When we obtain a pointer from an external function, we often want to turn it
into a reference to model the lifetime of the pointer. We have two options:

* [`ptr::as_ref<'a>() -> Option<'a
  T>`](https://doc.rust-lang.org/core/primitive.pointer.html#method.as_ref) -
  does null check
* `&*ptr` - deref `ptr` and take a reference of that [place
  expression](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)

`ptr::as_ref()` is implemented [^1] by use of `&*ptr`.

The reference we create from the pointer must satisfy the invariants for
references, otherwise we risk undefined behavior in our program. The following
must be satisfied:

* The pointee must be properly aligned.
* The pointer must be dereferenceable [^2].
* The resulting reference must not break the aliasing rules.
* The pointee must be live for at least the lifetime of the constructed reference.
* The pointee must be initialized and contain a valid bit pattern.

See the documentation for [ptr], and [reference], the [nomicon on aliasing] and
the [Rust book on UB] for further documentation.

Note that when we create references from pointers in this manner, the lifetime
of the resulting reference is [unbounded], meaning that it will outlive any
other lifetime.

We can opt out of some of these requirements:

 * By wrapping our value in an [`UnsafeCell`], we can opt out of the
   immutability invariant.
 * By wrapping our value in [`MaybeUninit`], we can opt out of the requirement
   that the pointee must be initialized.
 * By making the type `!Unpin`, we may opt out of the uniqueness requirement for
   mutable references [^3].

[ptr]: https://doc.rust-lang.org/core/ptr/index.html#safety
[reference]: https://doc.rust-lang.org/core/primitive.reference.html 
[nomicon on aliasing]: https://doc.rust-lang.org/nomicon/aliasing.html
[Rust book on UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[unbounded]: https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html
[`UnsafeCell`]: https://doc.rust-lang.org/core/cell/struct.UnsafeCell.html
[`MaybeUninit`]: https://doc.rust-lang.org/core/mem/union.MaybeUninit.html

[^1]: The `as_ref()` implementation:
```rust,ignore
    pub const unsafe fn as_ref<'a>(self) -> Option<&'a T> {
        // SAFETY: the caller must guarantee that `self` is valid
        // for a reference if it isn't null.
        if self.is_null() { None } else { unsafe { Some(&*self) } }
    }
```

[^2]: Dereferencable: the memory range of the given size starting at the pointer
must all be within the bounds of a single allocated object. Note that in
Rust, every (stack-allocated) variable is considered a separate allocated
object.

[^3]: This is a hack. There is [an
    effort](https://github.com/rust-lang/rfcs/pull/3467) to introduce
    `UnsafePinned` to opt out of the uniqueness guarantee.
