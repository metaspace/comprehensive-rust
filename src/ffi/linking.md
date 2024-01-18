# Linking


* We use the [snappy](https://github.com/google/snappy) compression library as an example C library
* Snappy includes a C interface (documented in
[`snappy-c.h`](https://github.com/google/snappy/blob/master/snappy-c.h))


We can declare external symbols with an `extern` block:

```rust,ignore
use libc::size_t;

#[link(name = "snappy", kind="dylib")]
extern "C" {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

fn main() {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);
}
```

* It is unsafe to call external functions
* By using an unsafe block we promise to the compiler that the call is safe
* Specifying the arguments of external functions incorrect may lead to type confusion bugs

After build we can see that snappy was linked:

```shell
> ldd target/debug/one
        linux-vdso.so.1 (0x00007ffdf29ce000)
        libsnappy.so.1 => /usr/lib/libsnappy.so.1 (0x00007f8c3525a000)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007f8c35235000)
        libc.so.6 => /usr/lib/libc.so.6 (0x00007f8c35053000)
        /lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007f8c352ff000)
        libstdc++.so.6 => /usr/lib/libstdc++.so.6 (0x00007f8c34c00000)
        libm.so.6 => /usr/lib/libm.so.6 (0x00007f8c34f66000)
```

<details>

* The "C" ABI is the default C ABI for the platform
* Multiple ABIs are available: [Supported ABIs](https://doc.rust-lang.org/reference/items/external-blocks.html#abi)
* `kind` can be "dylib", "static" and [others](https://doc.rust-lang.org/reference/items/external-blocks.html#the-link-attribute)
* `rustc` will link with the library given in the `link` attribute and the symbols will only be resolved in that library
* `rustc` also has a `-l` switch: `RUSTFLAGS="-lsnappy" cargo build`
* You can also build the rust program as a static or shared C library and
  include that in a C program.
  * You might want to use the `#[no_mangle]` attribute on exported symbols.

</details>
