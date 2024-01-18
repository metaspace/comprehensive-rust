# Small unsafe blocks

You should try to split up your unsafe blocks into atomic units that can be described more clearly. Consider the following example:

```rust,ignore
pub fn compress(src: &[u8]) -> Vec<u8> {
    unsafe {
        let srclen = src.len();
        let psrc = src.as_ptr().cast::<core::ffi::c_char>();

        let mut dstlen = snappy_max_compressed_length(srclen);
        let mut dst: Vec<u8> = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr().cast::<i8>();

        snappy_compress(psrc, srclen, pdst, &mut dstlen);
        dst.set_len(dstlen as usize);
        dst
    }
}
```

It can be quite complex to reason about this entire block. In addition, it is
not clear which operations inside the unsafe block are actually unsafe. If we
split it up, it becomes more clear:

```rust,ignore
pub fn compress2(src: &[u8]) -> Vec<u8> {
    let srclen = src.len();
    let psrc = src.as_ptr().cast::<core::ffi::c_char>();

    // SAFETY: By C API contract, this function does not read or write any
    // memory used by Rust and has no side effects.
    let mut dstlen = unsafe { snappy_max_compressed_length(srclen) };

    let mut dst: Vec<u8> = Vec::with_capacity(dstlen as usize);
    let pdst = dst.as_mut_ptr().cast::<i8>();


    // SAFETY: This function reads `srclen` bytes pointed to by `psrc`. Because
    // of the existenc of a shared reference to `src`, these are valid for read.
    // The function writes up to `dstlen` bytes pointed to by `pdst` and also
    // writes `dstlen`. `dstlen` is valid for write due to existence of a mutable
    // reference to the allocation. The allocation pointed to by `pdst` is valid
    // for write for `dstlen` bytes as we own the allcoation.
    unsafe { snappy_compress(psrc, srclen, pdst, &mut dstlen) };

    // SAFETY: By C API contract dstlen is not greater than `dst` capacity. The
    // elements up to `dstlen` are initialized by the call to `snappy_compress`
    // above.
    unsafe { dst.set_len(dstlen as usize) };

    dst
}
```

When we call unsafe Rust functions, we must adhere to their safety requirements. This is the documentation from `Vec::set_len()`:

> Forces the length of the vector to `new_len`.
>
> This is a low-level operation that maintains none of the normal
> invariants of the type. Normally changing the length of a vector
> is done using one of the safe operations instead, such as
> [`truncate`], [`resize`], [`extend`], or [`clear`].
>
> [`truncate`]: Vec::truncate
> [`resize`]: Vec::resize
> [`extend`]: Extend::extend
> [`clear`]: Vec::clear
>
> # Safety
>
> - `new_len` must be less than or equal to [`capacity()`].
> - The elements at `old_len..new_len` must be initialized.
>
> [`capacity()`]: Vec::capacity

<details>

* Notice how `&mut usize` is automatically coerced into `*mut usize`

</details>
