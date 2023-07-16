
Compiling panic_abort v0.0.0 (/home/ding/.rustup/toolchains/nightly-2018-12-30-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libpanic_abort)
error[E0428]: the name `abort` is defined multiple times
  --> /home/ding/.rustup/toolchains/nightly-2018-12-30-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libpanic_abort/lib.rs:61:5
   |
48 |     unsafe fn abort() -> ! {
   |     ---------------------- previous definition of the value `abort` here
...
61 |     unsafe fn abort() -> ! {
   |     ^^^^^^^^^^^^^^^^^^^^^^ `abort` redefined here
   |
   = note: `abort` must be defined only once in the value namespace of this block

error: aborting due to previous error

For more information about this error, try `rustc --explain E0428`.
error: Could not compile `panic_abort`.

To learn more, run the command again with --verbose.
error: `"cargo" "build" "--release" "--manifest-path" "/tmp/xargo.Zwy6GtlBtmjL/Cargo.toml" "--target" "x86_64-unknown-linux-sgx" "-p" "panic_abort"` failed with exit code: Some(101)
