plain
travis_time:end:099bf600:start=1550271127176465394,finish=1550271129346161345,duration=2169695951
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:19:05]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:19:05] error: hidden lifetime parameters in types are deprecated
[00:19:05]    --> src/librustc_lint/nonstandard_style.rs:459:44
[00:19:05]     |
[00:19:05] 459 |     fn check_generic_param(&mut self, cx: &LateContext, param: &hir::GenericParam) {
[00:19:05]     |
[00:19:05] note: lint level defined here
[00:19:05]    --> src/librustc_lint/lib.rs:22:9
[00:19:05]     |
[00:19:05]     |
[00:19:05] 22  | #![deny(rust_2018_idioms)]
[00:19:05]     |         ^^^^^^^^^^^^^^^^
[00:19:05]     = note: #[deny(elided_lifetimes_in_paths)] implied by #[deny(rust_2018_idioms)]
[00:19:06] error: aborting due to previous error
[00:19:06] 
[00:19:06] error: Could not compile `rustc_lint`.
[00:19:06] warning: build failed, waiting for other jobs to finish...
[00:19:06] warning: build failed, waiting for other jobs to finish...
[00:19:49] error: build failed
[00:19:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:49] expected success, got: exit code: 101
[00:19:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:49] Build completed unsuccessfully in 0:15:25
[00:19:49] Makefile:18: recipe for target 'all' failed
[00:19:49] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02da71a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 15 23:12:09 UTC 2019
---
travis_time:end:014f58d2:start=1550272330220711906,finish=1550272330225332581,duration=4620675
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2141a86e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08acdfd4
travis_time:start:08acdfd4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17241fd8
$ dmesg | grep -i kill
