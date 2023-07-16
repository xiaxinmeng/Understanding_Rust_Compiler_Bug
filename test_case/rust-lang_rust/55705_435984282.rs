plain
travis_time:end:0c299c43:start=1541442553312489885,finish=1541442554430759872,duration=1118269987
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:27] 
[00:04:27] error: This node does not have a stability attribute
[00:04:27]     --> libcore/num/mod.rs:4777:5
[00:04:27]      |
[00:04:27] 4777 |     InvalidDigit,
[00:04:27] 
[00:04:27] error: This node does not have a stability attribute
[00:04:27]     --> libcore/num/mod.rs:4778:5
[00:04:27]      |
---
[00:04:27]      |
[00:04:27] 4779 |     Underflow,
[00:04:27]      |     ^^^^^^^^^
[00:04:27] 
[00:04:27] error: This node does not have arive(Debug, Clone, PartialEq, Eq)]
[00:04:27] 
[00:04:28] error: missing documentation for a struct field
[00:04:28]     --> libcore/num/mod.rs:4771:5
[00:04:28]      |
---
[00:04:28] 
[00:04:28] error: missing documentation for an enum
[00:04:28]     --> libcore/num/mod.rs:4775:1
[00:04:28]      |
[00:04:28] 4775 | pub enum IntErrorKind {
[00:04:28] 
[00:04:28] error: missing documentation for a variant
[00:04:28]     --> libcore/num/mod.rs:4776:5
[00:04:[00:04:28] error: aborting due to 16 previous errors
[00:04:[00:04:28] error: aborting due to 16 previous errors
[00:04:28] 
[00:04:28] error: Could not compile `core`.
[00:04:28] 
[00:04:28] To learn more, run the command again with --verbose.
[00:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:28] expected success, got: exit code: 101
[00:04:28] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:04:28] travis_fold:end:stage0-std

[00:04:28] travis_time:end:stage0-std:start=1541442812513021635,finish=1541442832870690834,duration=20357669199


[00:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:28] Build completed unsuccessfully in 0:00:21
[00:04:28] Makefile:28: recipe for target 'all' failed
[00:04:28] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01ff4fb6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:06a5fc34:start=1541442833541286653,finish=1541442833547343721,duration=6057068
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:041371b9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:353eb540
travis_time:start:353eb540
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23017a18
$ dmesg | grep -i kil
