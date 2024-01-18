
# Writing safe APIs

* It is unsafe to call external functions
* When we build idiomatic Rust APIs, we strive to provide safe interfaces to libraries
* We have to ensure that our APIs are sound - they must not invoke UB when used from safe code

The C definition of `validate_compressed_buffer()` is:

```c
snappy_status snappy_validate_compressed_buffer(const char* compressed,
                                                size_t compressed_length);
```

Bindgen has generated the following prototype for us:

```rust,ignore
extern "C" {
    pub fn snappy_validate_compressed_buffer(
        compressed: *const ::std::os::raw::c_char,
        compressed_length: usize,
    ) -> snappy_status;
}
```

`rustc` cannot know if the external function will uphold Rust safety invariants.
When we use the `unsafe` keyword, we tell the compiler that we make sure safety
invariants are upheld.

Some projects require each `unsafe` block to be annotated with a `SAFETY:`
comment, describing why the code inside the unsafe region is UB free:

```rust,ignore
pub fn validate_compressed_buffer(src: &[u8]) -> bool {
    // SAFETY: `snappy_validate_compressed_buffer()` reads from the memory
    // pointed to by the first argument. The function never reads more than the
    // number of bytes given by the second argument and does not use the pointer
    // after the function returns. The pointee is guaranteed to be immutable for
    // the duration of the call by the existence of a shared reference.
    unsafe {
        snappy_validate_compressed_buffer(
            src.as_ptr().cast::<core::ffi::c_char>(),
            src.len(),
        ) == 0
    }
}
```
