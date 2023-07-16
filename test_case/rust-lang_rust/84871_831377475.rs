shell
$ ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc     src/test/run-make-fulldeps/coverage/no_cov_func.rs 
error[E0554]: `#[feature(no_coverage)]` may not be used on the dev release channel
 --> src/test/run-make-fulldeps/coverage/no_cov_func.rs:3:1
  |
3 | #[feature(no_coverage)]
  | ^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#[feature(no_coverage)]` may not be used on the dev release channel
  --> src/test/run-make-fulldeps/coverage/no_cov_func.rs:10:1
   |
10 | #[feature(no_coverage)]
   | ^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0554`.
