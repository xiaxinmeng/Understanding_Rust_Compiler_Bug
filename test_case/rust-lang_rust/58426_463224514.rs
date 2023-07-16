plain
travis_time:end:006e59b7:start=1550067918292459754,finish=1550068179125410797,duration=260832951043
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:12:25]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:12:55] error[E0308]: mismatched types
[00:12:55]    --> src/librustc/traits/error_reporting.rs:651:74
[00:12:55]     |
[00:12:55] 651 |                         let parent_node = self.tcx.hir().get_parent_node(obligation.cause.body_id);
[00:12:55]     |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `syntax::ast::NodeId`, found struct `hir::HirId`
[00:12:55]     = note: expected type `syntax::ast::NodeId`
[00:12:55]                found type `hir::HirId`
[00:12:55] 
[00:13:03] error: aborting due to previous error
---
[00:13:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:13:03] expected success, got: exit code: 101
[00:13:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:13:03] Build completed unsuccessfully in 0:04:15
[00:13:03] make: *** [all] Error 1
[00:13:03] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24171912
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 14:42:51 UTC 2019
---
travis_time:end:06a5d5bb:start=1550068973113858684,finish=1550068973118859580,duration=5000896
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:33940de6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0124f486
travis_time:start:0124f486
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b0fbeca
$ dmesg | grep -i kill
