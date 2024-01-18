# Example

Consider this Rust type intended to interface the Linux kernel `struct bio` (block IO descriptor):

```rust,ignore
/// A block device IO descriptor (`struct bio`)
///
/// # Invariants
///
/// Instances of this type is always reference counted. A call to
/// `bindings::bio_get()` ensures that the instance is valid for read at least
/// until a matching call to `bindings :bio_put()`.
#[repr(transparent)]
pub struct Bio(UnsafeCell<MaybeUninit<bindings::bio>>);
```

We wrap the `bio` type in `MaybeUninit` and `UnsafeCell` to ensure that when we
create references to this type, we are not encountering UB if the value is
mutated, or if some fields of the value contain invalid state.

We write the following method to create a reference to our type from a pointer
to a `struct bio`:

```rust,ignore
    /// Create an instance of `Bio` from a raw pointer.
    ///
    /// # Safety
    ///
    /// If `ptr` is not null, caller must ensure positive refcount for the
    /// pointee and immutability for the duration of the returned lifetime.
    #[inline(always)]
    pub(crate) unsafe fn from_raw<'a>(ptr: *mut bindings::bio) -> Option<&'a Self> {
        Some(
            // SAFETY: by the safety requirement of this funciton, `ptr` is
            // valid for read for the duration of the returned lifetime
            unsafe { &*NonNull::new(ptr)?.as_ptr().cast::<Bio>() },
        )
    }
```

The Linux kernel `struct request`(block layer request) may have a `struct bio`
associated with it. In the wrapper type for `struct request` we add an accessor
method to obtain a reference to this `Bio` if it is present:

```rust,ignore
    /// Get a wrapper for the first Bio in this request
    #[inline(always)]
    pub fn bio(&self) -> Option<&Bio> {
        let ptr = unsafe { (*self.ptr).bio };
        // SAFETY: By C API contract, if `bio` is not null it will have a
        // positive refcount at least for the duration of the lifetime of
        // `&self`.
        unsafe { Bio::from_raw(ptr) }
    }

```

Note that we tie the lifetime of the returned reference to the lifetime of the
request that the method is called on (lifetime of return value is elided to
lifetime of `&self`).
