rustc
$ RUST_BACKTRACE=1 rustc -Z treat-err-as-bug=500 ./main.rs 
!! master-installed (default)
!! Executing '/home/user/.cargo/bin/rustc' in pwd='/tmp/blah/src' with args(3): '-Z treat-err-as-bug=500 ./main.rs'
error[E0507]: cannot move out of `*key` which is behind a shared reference
  --> ./main.rs:15:35
   |
15 |                 String::from_utf8(*key).unwrap()
   |                                   ^^^^ move occurs because `*key` has type `std::vec::Vec<u8>`, which does not implement the `Copy` trait

error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
