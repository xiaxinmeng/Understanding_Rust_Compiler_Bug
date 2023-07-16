
$ rustc +nightly foo.rs -C opt-level=1 -C codegen-units=200 -Z thinlto=no
$ ./foo
$ rustc +nightly foo.rs -C opt-level=2 -C codegen-units=200 -Z thinlto=no
$ ./foo
thread 'main' panicked at 'index 5 out of range for slice of length 1', libcore/slice/mod.rs:785:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
