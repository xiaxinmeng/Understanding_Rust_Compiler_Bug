plain
........i.......................i.......................i............................... 3784/3847
...............................................................
failures:

---- src/future/into_future.rs - future::into_future::IntoFuture::into_future (line 123) stdout ----
error[E0658]: use of unstable library feature 'into_future'
  |
  |
8 | let mut fut = v.into_future();
  |
  = note: see issue #67644 <https://github.com/rust-lang/rust/issues/67644> for more information
  = help: add `#![feature(into_future)]` to the crate attributes to enable

