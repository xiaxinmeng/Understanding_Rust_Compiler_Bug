plain
travis_time:end:1b71f8e0:start=1549959208698294557,finish=1549959209599693803,duration=901399246
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:52]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:03:52]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:03:53]    Compiling rustc-demangle v0.1.10
[00:03:57]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:03:59] error[E0425]: cannot find value `ealier` in this scope
[00:03:59]    --> src/libstd/time.rs:238:20
[00:03:59]     |
[00:03:59] 238 |         if self >= ealier {
[00:03:59]     |                    ^^^^^^ help: a local variable with a similar name exists: `earlier`
[00:04:01] error: aborting due to previous error
[00:04:01] 
[00:04:01] For more information about this error, try `rustc --explain E0425`.
[00:04:01] error: Could not compile `std`.
[00:04:01] error: Could not compile `std`.
[00:04:01] 
[00:04:01] To learn more, run the command again with --verbose.
[00:04:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:01] expected success, got: exit code: 101
[00:04:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:01] Build completed unsuccessfully in 0:00:44
[00:04:01] make: *** [all] Error 1
[00:04:01] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03c79879
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 08:17:42 UTC 2019
---
travis_time:end:04e458b6:start=1549959463295586956,finish=1549959463300065827,duration=4478871
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19c60068
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a7346db
travis_time:start:0a7346db
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02cd0ac2
$ dmesg | grep -i kill
