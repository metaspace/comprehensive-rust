# Pin

Links
 - [Pin](https://doc.rust-lang.org/core/pin/struct.Pin.html)
 - [Future](https://doc.rust-lang.org/core/future/trait.Future.html)
 - [Stack Overflow](https://stackoverflow.com/a/73178712) answer that inspired this section

## What is pinning?

From the docs:

>\[`Pin<P>`\] is a wrapper around a kind of pointer which makes that pointer “pin” its value in place, preventing the value referenced by that pointer from being moved unless it implements Unpin.

```rust,ignore
impl<P: Deref<Target: Unpin>> Pin<P> {
  pub fn into_inner(pin: Pin<P>) -> P {..}
}
```

```rust,ignore
pub fn get_mut(self) -> &'a mut T
where
    T: Unpin,
```

In safe code, pinned pointers provide address stability for `!Unpin` types.

## Why do we sometimes need to pin futures?

Futures may be self referential, and the `Future::poll()` function requires the `self` argument to be pinned:

```rust,ignore
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

By the Pin API contract, `self` cannot move after poll has been called if `Self` is `!Unpin`. If we await a future by value, the future is consumed and the contract is upheld:

```rust,editable
pub async fn foo() {
    let fut = async {};
    fut.await;
    //drop(fut); // error[E0382]: use of moved value: `fut`
}
```

But if were to await the future by reference, we could violate the contract. The following code will _not_ compile:

```rust,compile_fail
pub async fn foo() {
    let fut = async {};
    let fut2 = async {};
    (&mut fut).await;
    std::mem::swap(&mut fut, &mut fut2);
    (&mut fut).await;
}
```

To achieve this, `&mut Future` is not a future unless the referee is `Unpin`:

```rust,ignore
impl<F> Future for &mut F
where
    F: Future + Unpin + ?Sized,
```

The following example illustrates how futures returned by `async` blocks are `!Unpin`:

```rust,editable,compile_fail
#[tokio::main]
async fn main() {
    let mut f1 = core::future::pending();
    let f2 = async {};
    //tokio::pin!(f2);

    loop {
        tokio::select! {
            _ = &mut f1 => { println!("f1"); }
            _ = &mut f2 => { println!("f2"); }
        }
    }
}
```

Uncomment line 5 to make the example compile. Notice how `core::future::pending()` is `Unpin` and does not require pinning.

Notice that if we only wanted to `await` `f2` once, we could do so without taking a reference to the future:

```rust,ignore,editable
#[tokio::main]
async fn main() {
    let mut f1 = core::future::pending();
    let f2 = async {};

    tokio::select! {
        _ = &mut f1 => { println!("f1"); }
        _ = f2 => { println!("f2"); }
    }
}
```

## Why are `async` blocks `!Unpin`?

From the [async-book](https://github.com/rust-lang/async-book/blob/ed022fc51a1c45e08be12bab65bc1cfd39d32a0d/src/04_pinning/01_chapter.md?plain=1#L70):

> However, what happens if we have an `async` block that uses references?
> For example:
> 
> ```rust,edition2018,ignore
> async {
>     let mut x = [0; 128];
>     let read_into_buf_fut = read_into_buf(&mut x);
>     read_into_buf_fut.await;
>     println!("{:?}", x);
> }
> ```
> 
> What struct does this compile down to?
> 
> ```rust,ignore
> struct ReadIntoBuf<'a> {
>     buf: &'a mut [u8], // points to `x` below
> }
> 
> struct AsyncFuture {
>     x: [u8; 128],
>     read_into_buf_fut: ReadIntoBuf<'what_lifetime?>,
> }
> ```
> 
> Here, the `ReadIntoBuf` future holds a reference into the other field of our
> structure, `x`. However, if `AsyncFuture` is moved, the location of `x` will
> move as well, invalidating the pointer stored in `read_into_buf_fut.buf`.
> 
> Pinning futures to a particular spot in memory prevents this problem, making
> it safe to create references to values inside an `async` block.

Hand written futures may be `!Unpin` because they are self referential as well,
because they are part of intrusive lists, or for any other reason that requires
address stability of the future across successive calls to poll.
