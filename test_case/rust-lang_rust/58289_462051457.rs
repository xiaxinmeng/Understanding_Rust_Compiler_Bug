plain
travis_time:end:0331c84e:start=1549724228611119600,finish=1549724301514411354,duration=72903291754
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:49]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:03:49]    Compiling rustc-demangle v0.1.10
[00:03:49]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:03:54]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:00] error: type does not implement `fmt::Debug`; consider adding #[derive(Debug)] or a manual implementation
[00:04:00]     |
[00:04:00] 808 | / pub struct ErrorIter<'a> {
[00:04:00] 808 | / pub struct ErrorIter<'a> {
[00:04:00] 809 | |     current: Option<&'a (dyn Error + 'static)>,
[00:04:00]     | |_^
[00:04:00]     |
[00:04:00] note: lint level defined here
[00:04:00]    --> src/libstd/lib.rs:210:9
---
[00:04:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:00] expected success, got: exit code: 101
[00:04:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:00] Build completed unsuccessfully in 0:00:47
[00:04:00] make: *** [all] Error 1
[00:04:00] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1779ed36
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  9 15:02:31 UTC 2019
---
travis_time:end:06db1de0:start=1549724552186209773,finish=1549724552191759236,duration=5549463
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0001b30f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:180c11b3
travis_time:start:180c11b3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_fa
