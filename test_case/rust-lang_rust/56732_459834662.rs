plain
travis_time:end:2179d1de:start=1549046851619518989,finish=1549046856011209993,duration=4391691004
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:23:42]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:24:42] error: unused `std::result::Result` that must be used
[00:24:42]    --> src/librustc_driver/lib.rs:303:17
[00:24:42]     |
[00:24:42] 303 |                 tcx.analysis(LOCAL_CRATE);
[00:24:42]     |
[00:24:42]     = note: `-D unused-must-use` implied by `-D warnings`
[00:24:42]     = note: `-D unused-must-use` implied by `-D warnings`
[00:24:42]     = note: this `Result` may be an `Err` variant, which should be handled
[00:24:42] error: aborting due to previous error
[00:24:42] 
[00:24:42] error: Could not compile `rustc_driver`.
[00:24:42] 
[00:24:42] 
[00:24:42] To learn more, run the command again with --verbose.
[00:24:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:24:42] expected success, got: exit code: 101
[00:24:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:24:42] Build completed unsuccessfully in 0:19:11
[00:24:42] make: *** [all] Error 1
[00:24:42] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11366740
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  1 19:12:31 UTC 2019
---
travis_time:end:01bbd920:start=1549048352795024363,finish=1549048352800268935,duration=5244572
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04e883da
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a15b190
travis_time:start:0a15b190
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:067984d4
$ dmesg | grep -i kill
