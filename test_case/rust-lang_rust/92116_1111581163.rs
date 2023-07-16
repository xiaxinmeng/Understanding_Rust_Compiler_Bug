rust
#![feature(pin_macro)]
use std::task::Poll;
use std::pin::Pin;

// current proposed API:
let pinned_fut = pin!(future::ready(()));
assert_eq!(pinned_fut.as_mut().poll_once(), Poll::Ready(()));

// using a pinned `Self`-type in the signature:
let pinned_fut = pin!(future::ready(()));
assert_eq!(pinned_fut.poll_once(), Poll::Ready(())); // note the omission of `as_mut` here.
