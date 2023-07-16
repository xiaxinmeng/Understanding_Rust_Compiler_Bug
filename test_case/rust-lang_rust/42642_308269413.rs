bash
[00:54:08] error[E0277]: the trait bound `<Self as std::ops::Deref>::Target: std::marker::Sized` is not satisfied
[00:54:08]   --> /checkout/src/test/run-pass/associated-types-sugar-path.rs:19:29
[00:54:08]    |
[00:54:08] 19 |     fn baz(_: Self::Target) where Self: Deref {}
[00:54:08]    |                             ^ `<Self as std::ops::Deref>::Target` does not have a constant size known at compile-time
[00:54:08]    |
[00:54:08]    = help: the trait `std::marker::Sized` is not implemented for `<Self as std::ops::Deref>::Target`
[00:54:08]    = help: consider adding a `where <Self as std::ops::Deref>::Target: std::marker::Sized` bound
[00:54:08] 
[00:54:08] error: aborting due to previous error(s)
