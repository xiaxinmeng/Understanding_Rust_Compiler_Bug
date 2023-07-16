
error[[E0391]](https://doc.rust-lang.org/nightly/error-index.html#E0391): cycle detected when computing type of `BarFuture::{opaque#0}`
  --> src/lib.rs:7:18
   |
7  | type BarFuture = impl Future<Output = usize> + Send + Sync + 'static;
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: ...which requires type-checking `<impl at src/lib.rs:17:1: 17:11>::bar_project`...
  --> src/lib.rs:23:40
   |
23 |         let bar: &mut BarFuture = &mut self.bar;
   |                                        ^^^^
   = note: ...which requires evaluating trait selection obligation `core::pin::Pin<&'a mut State>: core::ops::deref::DerefMut`...
   = note: ...which again requires computing type of `BarFuture::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
  --> src/lib.rs:1:1
   |
1  | / #![feature(type_alias_impl_trait)]
2  | |
3  | | [use std::future::Future;](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=b2a9d6f2493a79743f66e7852aeca81a#)
4  | | [use std::pin::Pin;](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=b2a9d6f2493a79743f66e7852aeca81a#)
...  |
33 | |     }
34 | | }
   | |_^
