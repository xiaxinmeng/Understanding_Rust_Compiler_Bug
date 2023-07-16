rust
#![feature(pin_macro)]
use std::task::{self, Poll};
use std::pin::Pin;

// using a function to get the underlying `Context` object:
let pinned_fut = pin!(future::ready(()));
let cx = task::context().await; // this method is made up, it just returns the underlying `Context`
assert_eq!(pinned_fut.poll(cx), Poll::Ready(()));
