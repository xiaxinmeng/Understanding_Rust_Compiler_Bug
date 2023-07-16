
$ rustc +stage1 x.rs --edition=2015
[...]
error[E0573]: expected type, found variant `Err`
 --> x.rs:3:25
  |
3 |     fn into_future() -> Err {}
  |                         ^^^
  |                         |
  |                         not a type
  |                         help: try using the variant's enum: `std::prelude::rust_2021`
