
[00:02:28] error[E0230]: there is no type parameter Try on trait Try
[00:02:28]   --> /checkout/src/libcore/ops/try.rs:18:1
[00:02:28]    |
[00:02:28] 18 | / #[rustc_on_unimplemented = "the `?` operator can only be used in a function that returns `Result` \
[00:02:28] 19 | |                             (or another type that implements `{Try}`)"]
[00:02:28]    | |_______________________________________________________________________^
