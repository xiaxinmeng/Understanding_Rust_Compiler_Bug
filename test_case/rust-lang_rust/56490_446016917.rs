plain
travis_time:end:0549c3a3:start=1544484952341807474,finish=1544484953363552999,duration=1021745525
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:31]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:04:31]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:32]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:37]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:40] error[E0599]: no method named `checked_sub_duration` found for type `sys::unix::time::inner::Instant` in the current scope
[00:04:40]    --> src/libstd/time.rs:225:16
[00:04:40]     |
[00:04:40] 225 |         self.0.checked_sub_duration(&duration).map(|t| Instant(t))
[00:04:40]     | 
[00:04:40]    ::: src/libstd/sys/unix/time.rs:261:5
[00:04:40]     |
[00:04:40] 261 |     pub struct Instant {
[00:04:40] 261 |     pub struct Instant {
[00:04:40]     |     ------------------ method `checked_sub_duration` not found for this
[00:04:40]     = help: did you mean `checked_add_duration`?
[00:04:40] 
[00:04:40] 
[00:04:40] error[E0599]: no method named `checked_sub_duration` found for type `sys::unix::time::inner::SystemTime` in the current scope
[00:04:40]    --> src/libstd/time.rs:392:16
[00:04:40]     |
[00:04:40] 392 |         self.0.checked_sub_duration(&duration).map(|t| SystemTime(t))
[00:04:40]     | 
[00:04:40]    ::: src/libstd/sys/unix/time.rs:266:5
[00:04:40]     |
[00:04:40] 266 |     pub struct SystemTime {
[00:04:40] 266 |     pub struct SystemTime {
[00:04:40]     |     --------------------- method `checked_sub_duration` not found for this
[00:04:40]     = help: did you mean `checked_add_duration`?
[00:04:40] 
[00:04:40] 
[00:04:40] error[E0599]: no method named `sub_duration` found for type `sys::unix::time::Timespec` in the current scope
[00:04:40]     |
[00:04:40] 22  | struct Timespec {
[00:04:40] 22  | struct Timespec {
[00:04:40]     | --------------- method `sub_duration` not found for this
[00:04:40] ...
[00:04:40] 295 |             Instant { t: self.t.sub_duration(other) }
[00:04:40] 
[00:04:40] 
[00:04:40] error[E0599]: no method named `sub_duration` found for type `sys::unix::time::Timespec` in the current scope
[00:04:40]     |
[00:04:40] 22  | struct Timespec {
[00:04:40] 22  | struct Timespec {
[00:04:40]     | --------------- method `sub_duration` not found for this
[00:04:40] ...
[00:04:40] 323 |             SystemTime { t: self.t.sub_duration(other) }
[00:04:40] 
[00:04:40] error: aborting due to 4 previous errors
[00:04:40] 
[00:04:40] For more information about this error, try `rustc --explain E0599`.
[00:04:40] For more information about this error, try `rustc --explain E0599`.
[00:04:40] error: Could not compile `std`.
[00:04:40] 
[00:04:40] To learn more, run the command again with --verbose.
[00:04:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:40] expected success, got: exit code: 101
[00:04:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:40] Build completed unsuccessfully in 0:00:37
[00:04:40] Makefile:28: recipe for target 'all' failed
[00:04:40] make: *** [all] Error 1
15584 ./src/llvm/include
15212 ./src/llvm/include/llvm
15196 ./src/tools/lld
15056 ./src/test/run-pass
---
travis_time:end:06fa2a18:start=1544485243145643316,finish=1544485243153416429,duration=7773113
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11c5ed3e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ca86080
travis_time:start:1ca86080
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06e00ae6
$ dmesg | grep -i kill
