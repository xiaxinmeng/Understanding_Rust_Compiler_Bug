plain
travis_time:end:01d41984:start=1550232874282701886,finish=1550232875131673338,duration=848971452
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
[01:10:41] 
[01:10:41] running 119 tests
[01:11:05] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:10] i......iii.i.....ii
[01:11:10] 
[01:11:10]  finished in 28.498
[01:11:10] travis_fold:end:test_debuginfo

---
[01:29:56] 
[01:30:17] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:30:17]     --> src/librustc/session/config.rs:2619:24
[01:30:17]      |
[01:30:17] 2619 |               let sess = build_session(sessopts, None, registry);
[01:30:17]      | 
[01:30:17]     ::: src/librustc/session/mod.rs:1007:1
[01:30:17]      |
[01:30:17] 1007 | / pub fn build_session(
[01:30:17] 1007 | / pub fn build_session(
[01:30:17] 1008 | |     sopts: config::Options,
[01:30:17] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:30:17] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:30:17] 1022 | |     )
[01:30:17] 1023 | | }
[01:30:17]      | |_- defined here
[01:30:17] 
[01:30:17] 
[01:30:17] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:30:17]     --> src/librustc/session/config.rs:2637:24
[01:30:17]      |
[01:30:17] 2637 |               let sess = build_session(sessopts, None, registry);
[01:30:17]      | 
[01:30:17]     ::: src/librustc/session/mod.rs:1007:1
[01:30:17]      |
[01:30:17] 1007 | / pub fn build_session(
[01:30:17] 1007 | / pub fn build_session(
[01:30:17] 1008 | |     sopts: config::Options,
[01:30:17] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:30:17] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:30:17] 1022 | |     )
[01:30:17] 1023 | | }
[01:30:17]      | |_- defined here
[01:30:17] 
[01:30:17] 
[01:30:17] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:30:17]     --> src/librustc/session/config.rs:2651:24
[01:30:17]      |
[01:30:17] 2651 |               let sess = build_session(sessopts, None, registry);
[01:30:17]      | 
[01:30:17]     ::: src/librustc/session/mod.rs:1007:1
[01:30:17]      |
[01:30:17] 1007 | / pub fn build_session(
[01:30:17] 1007 | / pub fn build_session(
[01:30:17] 1008 | |     sopts: config::Options,
[01:30:17] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:30:17] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:30:17] 1022 | |     )
[01:30:17] 1023 | | }
[01:30:17]      | |_- defined here
[01:30:17] 
[01:30:17] 
[01:30:17] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:30:17]     --> src/librustc/session/config.rs:2661:24
[01:30:17]      |
[01:30:17] 2661 |               let sess = build_session(sessopts, None, registry);
[01:30:17]      | 
[01:30:17]     ::: src/librustc/session/mod.rs:1007:1
[01:30:17]      |
[01:30:17] 1007 | / pub fn build_session(
[01:30:17] 1007 | / pub fn build_session(
[01:30:17] 1008 | |     sopts: config::Options,
[01:30:17] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:30:17] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:30:17] 1022 | |     )
[01:30:17] 1023 | | }
[01:30:17]      | |_- defined here
[01:30:17] 
[01:30:17] 
[01:30:17] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:30:17]     --> src/librustc/session/config.rs:2669:24
[01:30:17]      |
[01:30:17] 2669 |               let sess = build_session(sessopts, None, registry);
[01:30:17]      | 
[01:30:17]     ::: src/librustc/session/mod.rs:1007:1
[01:30:17]      |
[01:30:17] 1007 | / pub fn build_session(
[01:30:17] 1007 | / pub fn build_session(
[01:30:17] 1008 | |     sopts: config::Options,
[01:30:17] 1009 | |     self_profiler: Option<Arc<Mutex<SelfProfiler>>>,
[01:30:17] 1010 | |     local_crate_source_file: Option<PathBuf>,
[01:30:17] 1022 | |     )
[01:30:17] 1023 | | }
[01:30:17]      | |_- defined here
[01:30:17] 
---
[01:30:25] 
[01:30:25] To learn more, run the command again with --verbose.
[01:30:25] 
[01:30:25] 
[01:30:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:30:25] 
[01:30:25] 
[01:30:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:25] Build completed unsuccessfully in 0:31:05
[01:30:25] Build completed unsuccessfully in 0:31:05
[01:30:25] make: *** [check] Error 1
[01:30:25] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e3ef98a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 15 13:45:11 UTC 2019
