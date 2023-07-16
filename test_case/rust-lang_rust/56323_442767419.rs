plain
travis_time:end:0adef38c:start=1543479626676268344,finish=1543479683323522522,duration=56647254178
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:01]    Compiling polonius-engine v0.5.0
[00:05:02]    Compiling lock_api v0.1.3
[00:05:02]    Compiling chalk-engine v0.8.0
[00:05:02]    Compiling env_logger v0.5.12
[00:05:02]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:05:06]    Compiling parking_lot_core v0.3.0
[00:05:06]    Compiling tempfile v3.0.3
[00:05:08]    Compiling parking_lot v0.6.4
[00:05:08]    Compiling crossbeam-epoch v0.3.1
---
[00:21:56]    Compiling polonius-engine v0.5.0
[00:21:58]    Compiling chalk-engine v0.8.0
[00:21:58]    Compiling env_logger v0.5.12
[00:21:58]    Compiling parking_lot_core v0.3.0
[00:21:58]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:22:04]    Compiling parking_lot v0.6.4
[00:22:04]    Compiling crossbeam-epoch v0.3.1
[00:22:04]    Compiling log_settings v0.1.2
[00:22:06]    Compiling flate2 v1.0.3
---
[00:46:46]     Checking num_cpus v1.8.0
[00:46:46]     Checking rand v0.4.3
[00:46:46]     Checking atty v0.2.11
[00:46:46]     Checking crossbeam-epoch v0.3.1
[00:46:46]     Checking rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:46:49]     Checking rustc-rayon-core v0.1.1
[00:46:49]     Checking parking_lot_core v0.3.0
[00:46:49]     Checking parking_lot v0.6.4
[00:46:50]     Checking rustc-rayon v0.1.1
---
[00:51:33] .................................................................................................... 100/5065
[00:51:37] .................................................................................................... 200/5065
[00:51:39] .............................ii............................................ii...................ii.. 300/5065
[00:51:42] ..............................................................................................iii... 400/5065
[00:51:45] .....iiiiiiii.iii............................iii...........................................i........ 500/5065
[00:51:52] .................................................................................................... 700/5065
[00:51:58] ................................................................................................i... 800/5065
[00:52:02] ........i........................................................................................... 900/5065
[00:52:05] ...............iiiii..................ii.iiii....................................................... 1000/5065
---
[00:52:46] .................................................................................................... 2300/5065
[00:52:50] .................................................................................................... 2400/5065
[00:52:54] .................................................................................................... 2500/5065
[00:52:58] .................................................................................................... 2600/5065
[00:53:02] ........iiiiiiiii................................................................................... 2700/5065
[00:53:08] .................................................................................................... 2900/5065
[00:53:12] .................................................................................................... 3000/5065
[00:53:15] .......................................................................i............................ 3100/5065
[00:53:19] .................................................................................................... 3200/5065
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:52] 
[01:07:52] running 119 tests
[01:07:56] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[01:07:56] .ii.i.....iiii.....
[01:07:56] 
[01:07:56]  finished in 3.671
[01:07:56] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:11] 
[01:08:11] running 118 tests
[01:08:38] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:08:42] ......iii.i.....ii
[01:08:42] 
[01:08:42]  finished in 31.062
[01:08:42] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:25] 
[01:09:25] running 97 tests
[01:11:27] ....FFFF...F.F.........FFFF............................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:14:20] failures:
[01:14:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:14:20] 
[01:14:20] ---- [run-pass] run-pass-fulldeps/deriving-encodable-decodable-box.rs stdout ----
[01:14:20] ---- [run-pass] run-pass-fulldeps/deriving-encodable-decodable-box.rs stdout ----
[01:14:20] 
[01:14:20] error: test compilation failed although it shouldn't!
[01:14:20] status: exit code: 1
[01:14:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-box.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-encodable-decodable-box/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-encodable-decodable-box/auxiliary"
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] stderr:
[01:14:20] stderr:
[01:14:20] ------------------------------------------
[01:14:20] {"message":"the name `rustc_ezilaires` is defined multiple times","code":{"code":"E0254","explanation":"\nAttempt was made to import an item whereas an extern crate with this name has\nalready been imported.\n\nErroneous code example:\n\n