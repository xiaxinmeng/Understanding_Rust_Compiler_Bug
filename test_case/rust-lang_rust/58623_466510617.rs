plain
travis_time:end:27256134:start=1550855847061037770,finish=1550855986444044834,duration=139383007064
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:52] 
######################################################################## 100.0%
[00:03:53] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:03:53]     Updating crates.io index
[00:04:05]     Updating git repository `https://github.com/Amanieu/hashbrown`
[00:04:07]   Downloaded toml v0.4.10
[00:04:07]   Downloaded lazy_static v0.2.11
[00:04:07]   Downloaded petgraph v0.4.13
[00:04:07]   Downloaded cc v1.0.28
---
[00:06:52]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:06:52]    Compiling rustc-demangle v0.1.10
[00:06:57]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:06:57]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:06:57]    Compiling hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[00:07:15] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:07:15] travis_fold:end:stage0-std

[00:07:15] travis_time:end:stage0-std:start=1550856370233384723,finish=1550856431085034599,duration=60851649876
---
[00:31:29]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:31:29]    Compiling rustc-demangle v0.1.10
[00:31:35]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:31:35]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:31:35]    Compiling hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[00:31:58] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:31:58] travis_fold:end:stage1-std

[00:31:58] travis_time:end:stage1-std:start=1550857839309315208,finish=1550857914259204906,duration=74949889698
---
[01:10:11]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[01:10:11]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:10:11]     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[01:10:11]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[01:10:11]     Checking hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[01:10:32]     Finished release [optimized] target(s) in 27.93s
[01:10:33] Documenting stage2 test (x86_64-unknown-linux-gnu)
[01:10:33]     Checking term v0.0.0 (/checkout/src/libterm)
[01:10:33]     Checking getopts v0.2.17
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:27:21] 
[01:27:21] running 119 tests
[01:27:50] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:27:55] i......iii.i.....ii
[01:27:55] 
[01:27:55]  finished in 33.860
[01:27:55] travis_fold:end:test_debuginfo

---
[01:47:11] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:47:11]   left: `1`,
[01:47:11]  right: `2`', src/libstd/collections/hash/map.rs:3152:9
[01:47:11] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:47:11] ..........F......................................................................................... 100/763
[01:47:11] thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/buffered.rs:1290:17
[01:47:11] thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/buffered.rs:1277:9
[01:47:11] thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:793:13
[01:47:11] .................................................................................................... 300/763
---
[01:47:23] 
[01:47:23] error: test failed, to rerun pass '--lib'
[01:47:23] 
[01:47:23] 
[01:47:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:47:23] 
[01:47:23] 
[01:47:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:47:23] Build completed unsuccessfully in 0:33:11
[01:47:23] Build completed unsuccessfully in 0:33:11
[01:47:23] make: *** [check] Error 1
[01:47:23] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10327b84
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 22 19:07:20 UTC 2019
---
travis_time:end:189cfc80:start=1550862442226217131,finish=1550862442232160897,duration=5943766
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dafe3d7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05e73a4c
travis_time:start:05e73a4c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
