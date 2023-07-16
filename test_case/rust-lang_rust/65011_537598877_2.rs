rust
$ RUST_BACKTRACE=0 /home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc -Z treat-err-as-bug=500 ./main.rs 
error[E0507]: cannot move out of `*key` which is behind a shared reference
  --> ./main.rs:15:35
   |
15 |                 String::from_utf8(*key).unwrap()
   |                                   ^^^^ move occurs because `*key` has type `std::vec::Vec<u8>`, which does not implement the `Copy` trait

error: internal compiler error: src/librustc_mir/borrow_check/mod.rs:1949: Accessing `(*_4)` with the kind `Write(Move)` shouldn't be possible
  --> ./main.rs:15:35
   |
15 |                 String::from_utf8(*key).unwrap()
   |                                   ^^^^

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:871:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (22bc9e1d9 2019-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug=500

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0507`.
