# `Send` and `Sync`

How does Rust know to forbid shared access across threads? The answer is in two traits:

* [`Send`][1]: a type `T` is `Send` if it is safe to move a `T` across a thread
  boundary.
* [`Sync`][2]: a type `T` is `Sync` if it is safe to move a `&T` across a thread
  boundary.

`Send` and `Sync` are [unsafe traits][3]. The compiler will automatically derive them for your types
as long as they only contain `Send` and `Sync` types. You can also implement them manually when you
know it is valid.

Here is another way to look at `Send` and `Sync`. There are three relevant
permissions:

 * Access value mutably from threads other than the one it was created on, with
   synchronization. (Send gives this.)
 * Access value immutably from threads other than the one it was created on,
   with synchronization. (Send and Sync both gives this - you only need one of
   them to have it.)
 * Access value immutably from threads other than the one it was created on,
   without synchronization. (Sync gives this.)

[1]: https://doc.rust-lang.org/std/marker/trait.Send.html
[2]: https://doc.rust-lang.org/std/marker/trait.Sync.html
[3]: ../unsafe/unsafe-traits.md

<details>

* One can think of these traits as markers that the type has certain thread-safety properties.
* They can be used in the generic constraints as normal traits.
  
</details>
