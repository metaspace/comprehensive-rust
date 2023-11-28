# Deep Dive

In this section we look at how the compiler desugars the function `multiple()` in the example below.

```rust,editable
use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

struct Counter {
    count: u32,
}

type CounterRef = Rc<RefCell<Counter>>;

struct CounterFuture {
    counter: CounterRef,
    target: u32,
}

impl Future for CounterFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _ctx: &mut Context) -> Poll<Self::Output> {
        let count = self.counter.borrow().count;
        if count == self.target {
            Poll::Ready(())
        } else {
            let count = &mut self.counter.borrow_mut().count;
            if *count > self.target {
                *count -= 1;
            } else {
                *count += 1;
            }
            Poll::Pending
        }
    }
}

fn count(counter: CounterRef, target: u32) -> impl Future<Output = ()> {
    CounterFuture { counter, target }
}

async fn multiple(counter: CounterRef, targets: [u32;2]) {
    loop {
        count(counter.clone(), targets[0]).await;
        count(counter.clone(), targets[1]).await;
    }
}
```

The compiler will implement a state machine for the function `multiple()` similar to the following code:

```rust,compile_fail
// State record implemented by compiler
enum MultipleFuture {
    Start(CounterRef, [u32; 2]),
    Await0(CounterRef, [u32; 2], impl Future<Output = ()>),
    Await1(CounterRef, [u32; 2], impl Future<Output = ()>),
    Done,
}

// Future trait implementation for `multiple()` implemented by compiler
impl Future for MultipleFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        loop {
            match *self {
                MultipleFuture::Start(counter, targets) => {
                    let next_future = count(counter.clone(), targets[0]);
                    *self = MultipleFuture::Await0(counter, targets, next_future);
                }
                MultipleFuture::Await0(counter, targets, ref mut active_future) => {
                    // We promise that we will not move the future from now
                    // until it is dropped
                    match unsafe { Pin::new_unchecked(active_future) }.poll(cx) {
                        Poll::Ready(()) => {
                            let next_future = count(counter.clone(), targets[1]);
                            *self = MultipleFuture::Await1(counter, targets, next_future)
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
                // We promise that we will not move the future from now
                // until it is dropped
                MultipleFuture::Await1(counter, targets, ref mut active_future) => {
                    match unsafe { Pin::new_unchecked(active_future) }.poll(cx) {
                        Poll::Ready(()) => *self = MultipleFuture::Start(counter.clone(), targets),
                        Poll::Pending => return Poll::Pending,
                    }
                }
                // Unreachable
                MultipleFuture::Done => return Poll::Ready(()),
            }
        }
    }
}
```
