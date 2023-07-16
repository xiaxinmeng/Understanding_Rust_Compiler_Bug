plain
travis_time:end:13b2350a:start=1558090773250171555,finish=1558090774033395656,duration=783224101
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:49] 
[01:23:49] running 143 tests
[01:23:52] i..iii.....i.ii.iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:23:54] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:23:54] 
[01:23:54]  finished in 4.794
[01:23:54] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:56] 
[01:23:56] running 9 tests
[01:23:56] iiiiiiiii
[01:23:56] 
[01:23:56]  finished in 0.155
[01:23:56] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:12] 
[01:24:12] running 122 tests
[01:24:38] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:24:43] .i.i......iii.i.....ii
[01:24:43] 
[01:24:43]  finished in 31.171
[01:24:43] travis_fold:end:test_debuginfo

---
[01:40:53]      |
[01:40:53] 1923 | / pub fn build_session_options_and_crate_config(
[01:40:53] 1924 | |     driver_symbols: &[&str],
[01:40:53] 1925 | |     matches: &getopts::Matches,
[01:40:53] 1926 | | ) -> (Options, FxHashSet<(String, Option<String>)>) {
[01:40:53] 2439 | |     )
[01:40:53] 2440 | | }
[01:40:53]      | |_- defined here
[01:40:53] ...
[01:40:53] ...
[01:40:53] 2748 |               let (sessopts, cfg) = build_session_options_and_crate_config(matches);
[01:40:53] 
[01:40:53] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[01:40:53]     --> src/librustc/session/config.rs:2767:35
[01:40:53]      |
[01:40:53]      |
[01:40:53] 1923 | / pub fn build_session_options_and_crate_config(
[01:40:53] 1924 | |     driver_symbols: &[&str],
[01:40:53] 1925 | |     matches: &getopts::Matches,
[01:40:53] 1926 | | ) -> (Options, FxHashSet<(String, Option<String>)>) {
[01:40:53] 2439 | |     )
[01:40:53] 2440 | | }
[01:40:53]      | |_- defined here
[01:40:53] ...
[01:40:53] ...
[01:40:53] 2767 |               let (sessopts, cfg) = build_session_options_and_crate_config(matches);
[01:40:53] 
[01:40:53] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[01:40:53]     --> src/librustc/session/config.rs:2781:33
[01:40:53]      |
[01:40:53]      |
[01:40:53] 1923 | / pub fn build_session_options_and_crate_config(
[01:40:53] 1924 | |     driver_symbols: &[&str],
[01:40:53] 1925 | |     matches: &getopts::Matches,
[01:40:53] 1926 | | ) -> (Options, FxHashSet<(String, Option<String>)>) {
[01:40:53] 2439 | |     )
[01:40:53] 2440 | | }
[01:40:53]      | |_- defined here
[01:40:53] ...
[01:40:53] ...
[01:40:53] 2781 |               let (sessopts, _) = build_session_options_and_crate_config(&matches);
[01:40:53] 
[01:40:53] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[01:40:53]     --> src/librustc/session/config.rs:2791:33
[01:40:53]      |
[01:40:53]      |
[01:40:53] 1923 | / pub fn build_session_options_and_crate_config(
[01:40:53] 1924 | |     driver_symbols: &[&str],
[01:40:53] 1925 | |     matches: &getopts::Matches,
[01:40:53] 1926 | | ) -> (Options, FxHashSet<(String, Option<String>)>) {
[01:40:53] 2439 | |     )
[01:40:53] 2440 | | }
[01:40:53]      | |_- defined here
[01:40:53] ...
[01:40:53] ...
[01:40:53] 2791 |               let (sessopts, _) = build_session_options_and_crate_config(&matches);
[01:40:53] 
[01:40:53] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[01:40:53]     --> src/librustc/session/config.rs:2799:33
[01:40:53]      |
[01:40:53]      |
[01:40:53] 1923 | / pub fn build_session_options_and_crate_config(
[01:40:53] 1924 | |     driver_symbols: &[&str],
[01:40:53] 1925 | |     matches: &getopts::Matches,
[01:40:53] 1926 | | ) -> (Options, FxHashSet<(String, Option<String>)>) {
[01:40:53] 2439 | |     )
[01:40:53] 2440 | | }
[01:40:53]      | |_- defined here
[01:40:53] ...
[01:40:53] ...
[01:40:53] 2799 |               let (sessopts, _) = build_session_options_and_crate_config(&matches);
[01:40:53] 
[01:40:53] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[01:40:53]     --> src/librustc/session/config.rs:3397:29
[01:40:53]      |
[01:40:53]      |
[01:40:53] 1923 | / pub fn build_session_options_and_crate_config(
[01:40:53] 1924 | |     driver_symbols: &[&str],
[01:40:53] 1925 | |     matches: &getopts::Matches,
[01:40:53] 1926 | | ) -> (Options, FxHashSet<(String, Option<String>)>) {
[01:40:53] 2439 | |     )
[01:40:53] 2440 | | }
[01:40:53]      | |_- defined here
[01:40:53] ...
[01:40:53] ...
[01:40:53] 3397 |           let (sessopts, _) = build_session_options_and_crate_config(&matches);
[01:40:53] 
[01:41:02] error: aborting due to 6 previous errors
[01:41:02] 
[01:41:02] For more information about this error, try `rustc --explain E0061`.
[01:41:02] For more information about this error, try `rustc --explain E0061`.
[01:41:03] error: Could not compile `rustc`.
[01:41:03] 
[01:41:03] To learn more, run the command again with --verbose.
[01:41:03] 
[01:41:03] 
[01:41:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:41:03] 
[01:41:03] 
[01:41:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:03] Build completed unsuccessfully in 0:29:29
[01:41:03] Build completed unsuccessfully in 0:29:29
[01:41:03] make: *** [check] Error 1
[01:41:03] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:095585c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 12:40:48 UTC 2019
---
travis_time:end:12ae583c:start=1558096850226829366,finish=1558096850232936731,duration=6107365
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03735b10
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2300c3a1
travis_time:start:2300c3a1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f139e80
$ dmesg | grep -i kill
