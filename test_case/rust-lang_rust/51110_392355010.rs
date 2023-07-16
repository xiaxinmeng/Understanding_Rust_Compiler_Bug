plain
[00:21:06]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:21:48] error: constant evaluation error
[00:21:48]    --> libcore/num/f32.rs:239:9
[00:21:48]     |
[00:21:48] 239 |         self.classify() == FpCategory::Normal
[00:21:48]     |         ^^^^^^^^^^^^^^^ calling non-const fn `f32::<impl f32>::classify`
[00:21:48]     |
[00:21:48]     = note: #[deny(const_err)] on by default
[00:21:48] error: constant evaluation error
[00:21:48]    --> libcore/num/f32.rs:239:9
[00:21:48]     |
[00:21:48]     |
[00:21:48] 239 |         self.classify() == FpCategory::Normal
[00:21:48]     |         ^^^^^^^^^^^^^^^ calling non-const fn `f32::<impl f32>::classify`
[00:21:48] error: constant evaluation error
[00:21:48]    --> libcore/num/f64.rs:239:9
[00:21:48]     |
[00:21:48]     |
[00:21:48] 239 |         self.classify() == FpCategory::Normal
[00:21:48]     |         ^^^^^^^^^^^^^^^ calling non-const fn `f64::<impl f64>::classify`
[00:21:48] 
[00:21:48] thread 'main' panicked at 'index out of bounds: the len is 8 but the index is 150', /checkout/src/libcore/slice/mod.rs:2057:14
[00:21:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:21:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:49] expected success, got: exit code: 101
[00:21:49] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:21:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:49] Build completed unsuccessfully in 0:17:06
[00:21:49] Build completed unsuccessfully in 0:17:06
[00:21:49] Makefile:28: recipe for target 'all' failed
[00:21:49] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27fddacf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
