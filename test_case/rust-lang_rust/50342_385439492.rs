plain
[01:19:29]    Doc-tests std
[01:19:33] 
[01:19:33] running 925 tests
[01:20:07] ii..................................................................................................
[01:20:26] ............................................F.....................................F............iii..
[01:20:48] ....................................................................................................
[01:21:06] .................iiii........ii.....................................................................
[01:21:14] ....................................................................................................
[01:21:31] .................................................................i..................................
[01:21:31] .................................................................i..................................
[01:21:56] ....................................................................................................
[01:22:08] ..........................................................................iiii......................
[01:22:11] .........................
[01:22:11] failures:
[01:22:11] 
[01:22:11] ---- f32.rs - f32::f32::mod_euc (line 252) stdout ----
[01:22:11]  error: incorrect close delimiter: `)`
[01:22:11]   --> f32.rs:261:48
[01:22:11]    |
[01:22:11] 12 | assert!(-std::f32::EPSILON).mod_euc(3.0) != 0.0);
[01:22:11]    |
[01:22:11] note: unclosed delimiter
[01:22:11]   --> f32.rs:253:11
[01:22:11]    |
[01:22:11]    |
[01:22:11] 4  | fn main() {
[01:22:11]    |           ^
[01:22:11] 
[01:22:11] error: unexpected close delimiter: `}`
[01:22:11]   --> f32.rs:262:1
[01:22:11] 13 | }
[01:22:11]    | ^
[01:22:11] 
[01:22:11] thread 'f32.rs - f32::f32::mod_euc (line 252)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:22:11] thread 'f32.rs - f32::f32::mod_euc (line 252)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:22:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:22:11] 
[01:22:11] ---- f64.rs - f64::f64::mod_euc (line 228) stdout ----
[01:22:11]  error: incorrect close delimiter: `)`
[01:22:11]   --> f64.rs:237:48
[01:22:11]    |
[01:22:11] 12 | assert!(-std::f64::EPSILON).mod_euc(3.0) != 0.0);
[01:22:11]    |
[01:22:11] note: unclosed delimiter
[01:22:11]   --> f64.rs:229:11
[01:22:11]    |
[01:22:11]    |
[01:22:11] 4  | fn main() {
[01:22:11]    |           ^
[01:22:11] 
[01:22:11] error: unexpected close delimiter: `}`
[01:22:11]   --> f64.rs:238:1
[01:22:11] 13 | }
[01:22:11]    | ^
[01:22:11] 
[01:22:11] thread 'f64.rs - f64::f64::mod_euc (line 228)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
---
[01:22:11] 
[01:22:11] error: test failed, to rerun pass '--doc'
[01:22:11] 
[01:22:11] 
[01:22:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:22:11] 
[01:22:11] 
[01:22:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:11] Build completed unsuccessfully in 0:39:41
[01:22:11] Build completed unsuccessfully in 0:39:41
[01:22:11] make: *** [check] Error 1
[01:22:11] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:083cd531
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
