# References and Pointers

## References

There are two kinds of reference:

* Shared reference: `&`
* Mutable reference: `&mut`

Which obey the following rules:

* A reference cannot outlive its referent
* A mutable reference cannot be aliased

That's it. That's the whole model references follow.

Unfortunately, Rust hasn't actually defined its aliasing model. 🙀

If we will be use the broadest possible definition of aliasing, we are good.
Rust's definition will probably be more restricted to factor in mutations and
liveness.

Use the following definition: variables and pointers alias if they refer to
overlapping regions of memory.
