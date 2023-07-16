plain
travis_time:end:184f015b:start=1543446509142086832,finish=1543446511684429596,duration=2542342764
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:38]    Compiling chalk-engine v0.8.0
[00:04:38]    Compiling env_logger v0.5.12
[00:04:39]    Compiling tempfile v3.0.3
[00:04:40]    Compiling parking_lot_core v0.3.0
[00:04:40]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:04:43]    Compiling crossbeam-epoch v0.3.1
[00:04:43]    Compiling log_settings v0.1.2
[00:04:43]    Compiling parking_lot v0.6.4
[00:04:43]    Compiling flate2 v1.0.3
---
[00:20:21]    Compiling polonius-engine v0.5.0
[00:20:21]    Compiling chalk-engine v0.8.0
[00:20:23]    Compiling env_logger v0.5.12
[00:20:23]    Compiling parking_lot_core v0.3.0
[00:20:25]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:20:28]    Compiling parking_lot v0.6.4
[00:20:28]    Compiling crossbeam-epoch v0.3.1
[00:20:28]    Compiling log_settings v0.1.2
[00:20:30]    Compiling flate2 v1.0.3
---
[00:43:50]     Checking num_cpus v1.8.0
[00:43:50]     Checking rand v0.5.5
[00:43:50]     Checking atty v0.2.11
[00:43:50]     Checking crossbeam-epoch v0.3.1
[00:43:50]     Checking rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:43:52]     Checking rustc-rayon-core v0.1.1
[00:43:52]     Checking rustc-rayon v0.1.1
[00:43:53]     Checking parking_lot_core v0.3.0
[00:43:54]     Checking parking_lot v0.6.4
---
[00:48:14] .................................................................................................... 100/5065
[00:48:17] .................................................................................................... 200/5065
[00:48:19] .............................ii............................................ii...................ii.. 300/5065
[00:48:22] ..............................................................................................iii... 400/5065
[00:48:25] .....iiiiiiii.iii............................iii...........................................i........ 500/5065
[00:48:31] .................................................................................................... 700/5065
[00:48:37] ................................................................................................i... 800/5065
[00:48:40] ........i........................................................................................... 900/5065
[00:48:43] ...............iiiii..................ii.iiii....................................................... 1000/5065
---
[00:49:21] .................................................................................................... 2300/5065
[00:49:25] .................................................................................................... 2400/5065
[00:49:29] .................................................................................................... 2500/5065
[00:49:32] .................................................................................................... 2600/5065
[00:49:36] ........iiiiiiiii................................................................................... 2700/5065
[00:49:42] .................................................................................................... 2900/5065
[00:49:45] .................................................................................................... 3000/5065
[00:49:48] .......................................................................i............................ 3100/5065
[00:49:51] .................................................................................................... 3200/5065
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:12] 
[01:03:12] running 117 tests
[01:03:15] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:03:15] i.i.....iiii.....
[01:03:15] 
[01:03:15]  finished in 3.341
[01:03:15] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:29] 
[01:03:29] running 118 tests
[01:03:52] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:03:56] ......iii.i.....ii
[01:03:56] 
[01:03:56]  finished in 27.225
[01:03:56] travis_fold:end:test_debuginfo

---
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:56] 
[01:03:56] running 47 tests
[01:04:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:04:37] ...F...........................................
[01:04:37] 
[01:04:37] ---- [ui] ui-fulldeps/deprecated-derive.rs stdout ----
[01:04:37] 
[01:04:37] 
[01:04:37] error: test compilation failed although it shouldn't!
[01:04:37] status: exit code: 1
[01:04:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/deprecated-derive.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deprecated-derive/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deprecated-derive/auxiliary" "-A" "unused"
[01:04:37] ------------------------------------------
[01:04:37] 
[01:04:37] ------------------------------------------
[01:04:37] stderr:
[01:04:37] stderr:
[01:04:37] -----------------------------------shMap<u32, u32> = HashMap::new(); // So it can be used!\n