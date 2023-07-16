
error: cannot find attribute `async_trait` in this scope
 --> src/lib.rs:1:3
  |
1 | #[async_trait]
  |   ^^^^^^^^^^^
  |
  = help: consider importing this attribute macro:
          async_trait::async_trait
  = note: `async_trait` is in scope, but it is a crate, not an attribute

error[[E0706]](https://doc.rust-lang.org/nightly/error_codes/E0706.html): functions in traits cannot be declared `async`
 --> src/lib.rs:7:5
  |
7 |     async fn get_thing(&self) -> anyhow::Result<Self::Item>;
  |     -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     `async` because of this
  |
  = note: `async` trait functions are not currently supported
  = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
  = note: [see issue #91611 <https://github.com/rust-lang/rust/issues/91611>](https://github.com/rust-lang/rust/issues/91611) for more information
  = help: [add `#![feature(async_fn_in_trait)]`](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=c4034bf4f00df0a1b23a57ad19bbd30e#) to the crate attributes to enable

For more information about this error, try `rustc --explain E0706`.
error: could not compile `playground` (lib) due to 2 previous errors
