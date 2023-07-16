plain
travis_time:end:0128cb10:start=1550449253697689067,finish=1550449328373580793,duration=74675891726
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:51] 
[01:13:51] running 119 tests
[01:14:16] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:14:20] i......iii.i.....ii
[01:14:20] 
[01:14:20]  finished in 29.335
[01:14:20] travis_fold:end:test_debuginfo

---
[01:34:13] 
[01:34:34] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:34:34]     --> src/librustc/session/config.rs:2617:24
[01:34:34]      |
[01:34:34] 2617 |               let sess = build_session(sessopts, None, registry);
[01:34:34]      | 
[01:34:34]     ::: src/librustc/session/mod.rs:1007:1
[01:34:34]      |
[01:34:34] 1007 | / pub fn build_session(
[01:34:34] 1007 | / pub fn build_session(
[01:34:34] 1008 | |     sopts: config::Options,
[01:34:34] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:34:34] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:34:34] 1022 | |     )
[01:34:34] 1023 | | }
[01:34:34]      | |_- defined here
[01:34:34] 
[01:34:34] 
[01:34:34] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:34:34]     --> src/librustc/session/config.rs:2635:24
[01:34:34]      |
[01:34:34] 2635 |               let sess = build_session(sessopts, None, registry);
[01:34:34]      | 
[01:34:34]     ::: src/librustc/session/mod.rs:1007:1
[01:34:34]      |
[01:34:34] 1007 | / pub fn build_session(
[01:34:34] 1007 | / pub fn build_session(
[01:34:34] 1008 | |     sopts: config::Options,
[01:34:34] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:34:34] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:34:34] 1022 | |     )
[01:34:34] 1023 | | }
[01:34:34]      | |_- defined here
[01:34:34] 
[01:34:34] 
[01:34:34] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:34:34]     --> src/librustc/session/config.rs:2649:24
[01:34:34]      |
[01:34:34] 2649 |               let sess = build_session(sessopts, None, registry);
[01:34:34]      | 
[01:34:34]     ::: src/librustc/session/mod.rs:1007:1
[01:34:34]      |
[01:34:34] 1007 | / pub fn build_session(
[01:34:34] 1007 | / pub fn build_session(
[01:34:34] 1008 | |     sopts: config::Options,
[01:34:34] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:34:34] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:34:34] 1022 | |     )
[01:34:34] 1023 | | }
[01:34:34]      | |_- defined here
[01:34:34] 
[01:34:34] 
[01:34:34] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:34:34]     --> src/librustc/session/config.rs:2659:24
[01:34:34]      |
[01:34:34] 2659 |               let sess = build_session(sessopts, None, registry);
[01:34:34]      | 
[01:34:34]     ::: src/librustc/session/mod.rs:1007:1
[01:34:34]      |
[01:34:34] 1007 | / pub fn build_session(
[01:34:34] 1007 | / pub fn build_session(
[01:34:34] 1008 | |     sopts: config::Options,
[01:34:34] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:34:34] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:34:34] 1022 | |     )
[01:34:34] 1023 | | }
[01:34:34]      | |_- defined here
[01:34:34] 
[01:34:34] 
[01:34:34] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:34:34]     --> src/librustc/session/config.rs:2667:24
[01:34:34]      |
[01:34:34] 2667 |               let sess = build_session(sessopts, None, registry);
[01:34:34]      | 
[01:34:34]     ::: src/librustc/session/mod.rs:1007:1
[01:34:34]      |
[01:34:34] 1007 | / pub fn build_session(
[01:34:34] 1007 | / pub fn build_session(
[01:34:34] 1008 | |     sopts: config::Options,
[01:34:34] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:34:34] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:34:34] 1022 | |     )
[01:34:34] 1023 | | }
[01:34:34]      | |_- defined here
[01:34:34] 
---
[01:34:43] 
[01:34:43] To learn more, run the command again with --verbose.
[01:34:43] 
[01:34:43] 
[01:34:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:34:43] 
[01:34:43] 
[01:34:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:43] Build completed unsuccessfully in 0:32:44
[01:34:43] Build completed unsuccessfully in 0:32:44
[01:34:43] make: *** [check] Error 1
[01:34:43] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16362ab2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 18 01:57:01 UTC 2019
---
travis_time:end:07c998e1:start=1550455022870337492,finish=1550455022876994055,duration=6656563
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00ae26d3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06eb0568
travis_time:start:06eb0568
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a2895e0
$ dmesg | grep -i kill
