
$ rustc +nightly --crate-type=lib test.rs --target=arm-linux-androideabi
error[E0570]: The ABI `"stdcall"` is not supported for the current target
 --> test.rs:1:1
  |
1 | / pub extern "stdcall" fn sup(_: isize) -> isize {
2 | |     0
3 | | }
  | |_^

error: aborting due to previous error
