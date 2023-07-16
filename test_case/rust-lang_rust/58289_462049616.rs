plain
travis_time:end:225af542:start=1549722635420175979,finish=1549722708982251417,duration=73562075438
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:37]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:43] error: This node does not have a stability attribute
[00:04:43]    --> src/libstd/error.rs:807:1
[00:04:43]     |
[00:04:43] 807 | / pub struct ErrorIter<'a> {
[00:04:43] 808 | |     current: Option<&'a (dyn Error + 'static)>,
[00:04:43]     | |_^
[00:04:43] 
[00:04:43] error: This node does not have a stability attribute
[00:04:43]    --> src/libstd/error.rs:811:1
[00:04:43]    --> src/libstd/error.rs:811:1
[00:04:43]     |
[00:04:43] 811 | / impl<'a> Iterator for ErrorIter<'a> {
[00:04:43] 812 | |     type Item = &'a (dyn Error + 'static);
[00:04:43] 813 | |
[00:04:43] 814 | |     fn next(&mut self) -> Option<Self::Item> {
[00:04:43] 818 | |     }
[00:04:43] 819 | | }
[00:04:43]     | |_^
[00:04:43] 
[00:04:43] 
[00:04:43] error: type does not implement `fmt::Debug`; consider adding #[derive(Debug)] or a manual implementation
[00:04:43]     |
[00:04:43] 807 | / pub struct ErrorIter<'a> {
[00:04:43] 807 | / pub struct ErrorIter<'a> {
[00:04:43] 808 | |     current: Option<&'a (dyn Error + 'static)>,
[00:04:43]     | |_^
[00:04:43]     |
[00:04:43] note: lint level defined here
[00:04:43]    --> src/libstd/lib.rs:210:9
---
[00:04:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:44] expected success, got: exit code: 101
[00:04:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:44] Build completed unsuccessfully in 0:00:47
[00:04:44] make: *** [all] Error 1
[00:04:44] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09d9d2e3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  9 14:36:41 UTC 2019
---
travis_time:end:064404e8:start=1549723002574247264,finish=1549723002580168139,duration=5920875
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f1e3944
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ace9852
travis_time:start:0ace9852
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:028f4634
$ dmesg | grep -i kill
