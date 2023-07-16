rust
//! #![feature(async_await)]
//!
//! # fn main() {
//! fn foo() {
//!     drop(async move {});
//! }
//! # foo();
//! # }
