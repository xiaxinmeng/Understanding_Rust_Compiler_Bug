plain
travis_time:end:00bf4372:start=1549895446682520464,finish=1549895524783535846,duration=78101015382
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:16]    Compiling cc v1.0.28
[00:03:16]    Compiling libc v0.2.46
[00:03:16]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:16]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:17] error: couldn't read src/libcore/core_arch_docs.md: No such file or directory (os error 2)
[00:03:17]   |
[00:03:17]   |
[00:03:17] 8 | #[doc(include = "core_arch_docs.md")]
[00:03:17]   |                 ^^^^^^^^^^^^^^^^^^^ couldn't read file
[00:03:17]   |
[00:03:17]   = help: external doc paths are relative to the crate root
[00:03:18]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:22]    Compiling compiler_builtins v0.1.5
[00:03:22]    Compiling cmake v0.1.33
[00:03:22]    Compiling backtrace-sys v0.1.27
---
[00:03:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:36] expected success, got: exit code: 101
[00:03:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:36] Build completed unsuccessfully in 0:00:21
[00:03:36] make: *** [all] Error 1
[00:03:36] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02708f85
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 14:35:50 UTC 2019
---
travis_time:end:23fc2560:start=1549895751458688360,finish=1549895751464181466,duration=5493106
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23853f54
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16b390c0
travis_time:start:16b390c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:18a9efdb
$ dmesg | grep -i kill
