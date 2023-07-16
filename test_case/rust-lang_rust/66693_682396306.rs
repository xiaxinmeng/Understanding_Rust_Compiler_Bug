
error[E0658]: panicking in constants is unstable
 --> src/lib.rs:1:17
  |
1 | struct Bug([u8; panic!(1)]);
  |                 ^^^^^^^^^
  |
  = note: see issue #51999 <https://github.com/rust-lang/rust/issues/51999> for more information
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at '`ref_to_mplace` called on non-ptr type', src/librustc_mir/interpret/place.rs:292:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic
