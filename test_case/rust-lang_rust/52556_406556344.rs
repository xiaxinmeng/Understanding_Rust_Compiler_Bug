plain
[00:04:08]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:04:14] error: This node does not have a stability attribute
[00:04:14]    --> libcore/time.rs:517:1
[00:04:14]     |
[00:04:14] 517 | / impl Mul<Duration> for u32 {
[00:04:14] 518 | |     type Output = Duration;
[00:04:14] 519 | |
[00:04:14] 520 | |     fn mul(self, rhs: Duration) -> Duration {
[00:04:14] 521 | |         rhs.checked_mul(self).expect("overflow when multiplying scalar by duration")
[00:04:14] 523 | | }
[00:04:14]     | |_^
[00:04:14] 
[00:04:17] error: build failed
[00:04:17] error: build failed
[00:04:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:17] expected success, got: exit code: 101
[00:04:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:04:17] travis_fold:end:stage0-std

[00:04:17] travis_time:end:stage0-std:start=1532081533014968808,finish=1532081558199578374,duration=25184609566


[00:04:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:17] Build completed unsuccessfully in 0:00:26
[00:04:17] make: *** [all] Error 1
[00:04:17] Makefile:28: recipe for target 'all' failed
