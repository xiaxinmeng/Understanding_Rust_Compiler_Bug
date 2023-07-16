
$ RUST_BACKTRACE=1 rustc test.rs -Z ast-json
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugr
eport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'called `Result::unwrap()` on an `Err` value: OS Error 8:
 Not enough storage is available to process this command.
', C:\bot\slave\nightly-win\build\src\libcore\result.rs:548

stack backtrace:
   1: 0x6a181f84
   2: 0x61f88a1c
   3: 0x61f8866d
   4: 0x61f8849e
   5: 0x61fd2658
   6: 0x7130b59c
   7: 0x7170c5c4
   8: 0x717b6613
   9: 0x717b3b0d
  10: 0x717c603e
  11: 0x717c5f5c
  12: 0x70c33cf8
  13: 0x61f882c5
  14: 0x61f85d9a
  15: 0x70c33b97
  16: 0x61f87e1f
  17: 0x77099ef2
