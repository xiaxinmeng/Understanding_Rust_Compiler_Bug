
$ rustc +stage1 x.rs --edition=2018       
[...]
error[E0573]: expected type, found variant `Err`
 --> x.rs:3:25
  |
3 |     fn into_future() -> Err {}
  |                         ^^^ not a type
  |
help: try using the variant's enum
  |
3 |     fn into_future() -> core::result::Result {}
  |                         ^^^^^^^^^^^^^^^^^^^^
3 |     fn into_future() -> crate::result::Result {}
  |                         ^^^^^^^^^^^^^^^^^^^^^
3 |     fn into_future() -> std::result::Result {}
  |                         ^^^^^^^^^^^^^^^^^^^
