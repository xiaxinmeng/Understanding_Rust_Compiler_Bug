
$ rustc +nightly -vV
rustc 1.31.0-nightly (77af31408 2018-10-11)
binary: rustc
commit-hash: 77af314083e5acabf9ba5335e47271f35eef2e99
commit-date: 2018-10-11
host: x86_64-unknown-linux-gnu
release: 1.31.0-nightly
LLVM version: 8.0
$ rustc +nightly 37437.rs 
error[E0669]: invalid value for constraint in inline assembly
 --> 37437.rs:4:14
  |
4 |     unsafe { asm!("" : : "i"(hello)) };
  |              ^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0669`.
