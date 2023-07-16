plain
travis_time:end:2708a897:start=1550064370670388663,finish=1550064371914852396,duration=1244463733
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
[01:09:40] 
[01:09:40] running 119 tests
[01:10:05] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:10:09] i......iii.i.....ii
[01:10:09] 
[01:10:09]  finished in 28.683
[01:10:09] travis_fold:end:test_debuginfo

---
[01:28:53] 
[01:29:13] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:29:13]     --> src/librustc/session/config.rs:2617:24
[01:29:13]      |
[01:29:13] 2617 |               let sess = build_session(sessopts, None, registry);
[01:29:13]      | 
[01:29:13]     ::: src/librustc/session/mod.rs:1007:1
[01:29:13]      |
[01:29:13] 1007 | / pub fn build_session(
[01:29:13] 1007 | / pub fn build_session(
[01:29:13] 1008 | |     sopts: config::Options,
[01:29:13] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:29:13] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:29:13] 1022 | |     )
[01:29:13] 1023 | | }
[01:29:13]      | |_- defined here
[01:29:13] 
[01:29:13] 
[01:29:13] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:29:13]     --> src/librustc/session/config.rs:2635:24
[01:29:13]      |
[01:29:13] 2635 |               let sess = build_session(sessopts, None, registry);
[01:29:13]      | 
[01:29:13]     ::: src/librustc/session/mod.rs:1007:1
[01:29:13]      |
[01:29:13] 1007 | / pub fn build_session(
[01:29:13] 1007 | / pub fn build_session(
[01:29:13] 1008 | |     sopts: config::Options,
[01:29:13] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:29:13] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:29:13] 1022 | |     )
[01:29:13] 1023 | | }
[01:29:13]      | |_- defined here
[01:29:13] 
[01:29:13] 
[01:29:13] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:29:13]     --> src/librustc/session/config.rs:2649:24
[01:29:13]      |
[01:29:13] 2649 |               let sess = build_session(sessopts, None, registry);
[01:29:13]      | 
[01:29:13]     ::: src/librustc/session/mod.rs:1007:1
[01:29:13]      |
[01:29:13] 1007 | / pub fn build_session(
[01:29:13] 1007 | / pub fn build_session(
[01:29:13] 1008 | |     sopts: config::Options,
[01:29:13] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:29:13] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:29:13] 1022 | |     )
[01:29:13] 1023 | | }
[01:29:13]      | |_- defined here
[01:29:13] 
[01:29:13] 
[01:29:13] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:29:13]     --> src/librustc/session/config.rs:2659:24
[01:29:13]      |
[01:29:13] 2659 |               let sess = build_session(sessopts, None, registry);
[01:29:13]      | 
[01:29:13]     ::: src/librustc/session/mod.rs:1007:1
[01:29:13]      |
[01:29:13] 1007 | / pub fn build_session(
[01:29:13] 1007 | / pub fn build_session(
[01:29:13] 1008 | |     sopts: config::Options,
[01:29:13] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:29:13] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:29:13] 1022 | |     )
[01:29:13] 1023 | | }
[01:29:13]      | |_- defined here
[01:29:13] 
[01:29:13] 
[01:29:13] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:29:13]     --> src/librustc/session/config.rs:2667:24
[01:29:13]      |
[01:29:13] 2667 |               let sess = build_session(sessopts, None, registry);
[01:29:13]      | 
[01:29:13]     ::: src/librustc/session/mod.rs:1007:1
[01:29:13]      |
[01:29:13] 1007 | / pub fn build_session(
[01:29:13] 1007 | / pub fn build_session(
[01:29:13] 1008 | |     sopts: config::Options,
[01:29:13] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:29:13] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:29:13] 1022 | |     )
[01:29:13] 1023 | | }
[01:29:13]      | |_- defined here
[01:29:13] 
---
[01:29:21] 
[01:29:21] To learn more, run the command again with --verbose.
[01:29:21] 
[01:29:21] 
[01:29:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:29:21] 
[01:29:21] 
[01:29:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:21] Build completed unsuccessfully in 0:30:55
[01:29:21] Build completed unsuccessfully in 0:30:55
[01:29:21] Makefile:48: recipe for target 'check' failed
[01:29:21] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b0eb3d8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 14:55:44 UTC 2019
---
travis_time:end:009d3a76:start=1550069746046369497,finish=1550069746051734721,duration=5365224
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00d05a89
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:044c1d78
travis_time:start:044c1d78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:29060f50
$ dmesg | grep -i kill
