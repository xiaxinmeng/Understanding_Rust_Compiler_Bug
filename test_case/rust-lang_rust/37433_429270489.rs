
$ rustc +nightly 37433.rs
error[E0669]: invalid value for constraint in inline assembly
 --> 37433.rs:5:9
  |
5 |         asm!("" :: "r"(""));
  |         ^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0669`.
$ rustc +nightly 37433-2.rs 
error[E0669]: invalid value for constraint in inline assembly
 --> 37433-2.rs:9:9
  |
9 |         asm!( "ret" : : "{rdi}"(target) );
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0669`.
$ rustc +nightly -vV
rustc 1.31.0-nightly (77af31408 2018-10-11)
binary: rustc
commit-hash: 77af314083e5acabf9ba5335e47271f35eef2e99
commit-date: 2018-10-11
host: x86_64-unknown-linux-gnu
release: 1.31.0-nightly
LLVM version: 8.0
