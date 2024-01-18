# Synchronization

* The [Rust memory model] is not fully defined but for now follows C++11.
* This memory model might not be compatible with external libraries (this is the
  case for Rust code in the Linux kernel).
* It is not sound to combine Rust synchronization primitives with
  synchronization primitives from external libraries
  * That is, synchronizing on a C atomic with `core::sync::atomic::*` may yield UB

If you need to synchronize with non-Rust code, do so entirely en Rust or
entirely external. For instance, call C function provided by the external
library to do the synchronization.

## References

* [What about: volatile, concurrency, and interaction with untrusted threads](https://github.com/rust-lang/unsafe-code-guidelines/issues/152#issuecomment-506027424)
* [Support for Linux kernel memory model](https://github.com/rust-lang/unsafe-code-guidelines/issues/348)
* [Document the current recommendation when Rust is used to communicate with a different memory ordering model](https://github.com/rust-lang/unsafe-code-guidelines/issues/476)

[Rust memory model]: https://doc.rust-lang.org/reference/memory-model.html
