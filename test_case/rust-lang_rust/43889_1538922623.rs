sh
warning: use of deprecated method `futures::Future::boxed`: removed without replacement, recommended to use a local extension trait or function if needed, more details in https://github.com/rust-lang-nursery/futures-rs/issues/228
  --> src/main.rs:19:64
   |
19 |     let http_fut: Box<Future<Item=(),Error=()> + Send> = get().boxed();
   |                                                                ^^^^^
   |
   = note: `#[warn(deprecated)]` on by default
