
[02:13:48] -- Build files have been written to: /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/lsan/build
[02:13:48] running: "cmake" "--build" "." "--target" "lsan" "--config" "Release" "--" "-j2"
[02:13:48] 
[02:13:48] --- stderr
[02:13:48] make[1]: warning: -jN forced in submake: disabling jobserver mode.
[02:13:48] make[1]: *** No rule to make target `lsan'.  Stop.
[02:13:48] thread 'main' panicked at '
[02:13:48] command did not execute successfully, got: exit code: 2
[02:13:48] 
[02:13:48] build script failed, must exit now', /Users/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.24/src/lib.rs:593:4
[02:13:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[02:13:48] 
[02:13:48] make: *** [check] Error 1
