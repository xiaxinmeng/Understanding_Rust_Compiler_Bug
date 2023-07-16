
error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait return
 --> src/lib.rs:4:33
  |
4 | fn master() -> dyn FnOnce() -> (impl Future<Output = ()> + 'static) {
  |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0562`.
error: could not compile `issue-96270` due to previous error
