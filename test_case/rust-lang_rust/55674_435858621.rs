plain
travis_time:end:1b8cd4b5:start=1541420765623417153,finish=1541420766810025814,duration=1186608661
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:07:50]    |                     ^^^^ doesn't have a size known at compile-time
[00:07:50]    |
[00:07:50]    = help: the trait `std::marker::Sized` is not implemented for `Self`
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = help: consider adding a `where Self: std::marker::Sized` bound
[00:07:50]    = help: unsized locals are gated as an unstable feature
[00:07:50] 
[00:07:50] error[E0277]: the size for values of type `Self` cannot be known at compilation time
[00:07:50]   --> librustc/mir/interpret/pointer.rs:21:24
[00:07:50]   --> librustc/mir/interpret/pointer.rs:21:24
[00:07:50]    |
[00:07:50] 21 |     fn truncate_to_ptr(self, val: u128) -> (u64, bool) {
[00:07:50]    |
[00:07:50]    = help: the trait `std::marker::Sized` is not implemented for `Self`
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = help: consider adding a `where Self: std::marker::Sized` bound
[00:07:50]    = help: unsized locals are gated as an unstable feature
[00:07:50] 
[00:07:50] error[E0277]: the size for values of type `Self` cannot be known at compilation time
[00:07:50]   --> librustc/mir/interpret/pointer.rs:27:34
[00:07:50]   --> librustc/mir/interpret/pointer.rs:27:34
[00:07:50]    |
[00:07:50] 27 |     fn overflowing_signed_offset(self, val: u64, i: i128) -> (u64, bool) {
[00:07:50]    |
[00:07:50]    = help: the trait `std::marker::Sized` is not implemented for `Self`
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = help: consider adding a `where Self: std::marker::Sized` bound
[00:07:50]    = help: unsized locals are gated as an unstable feature
[00:07:50] 
[00:07:50] error[E0277]: the size for values of type `Self` cannot be known at compilation time
[00:07:50]   --> librustc/mir/interpret/pointer.rs:39:27
[00:07:50]   --> librustc/mir/interpret/pointer.rs:39:27
[00:07:50]    |
[00:07:50] 39 |     fn overflowing_offset(self, val: u64, i: u64) -> (u64, bool) {
[00:07:50]    |
[00:07:50]    = help: the trait `std::marker::Sized` is not implemented for `Self`
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = help: consider adding a `where Self: std::marker::Sized` bound
[00:07:50]    = help: unsized locals are gated as an unstable feature
[00:07:50] 
[00:07:50] error[E0277]: the size for values of type `Self` cannot be known at compilation time
[00:07:50]   --> librustc/mir/interpret/pointer.rs:45:28
[00:07:50]   --> librustc/mir/interpret/pointer.rs:45:28
[00:07:50]    |
[00:07:50] 45 |     fn signed_offset<'tcx>(self, val: u64, i: i64) -> EvalResult<'tcx, u64> {
[00:07:50]    |
[00:07:50]    = help: the trait `std::marker::Sized` is not implemented for `Self`
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = help: consider adding a `where Self: std::marker::Sized` bound
[00:07:50]    = help: unsized locals are gated as an unstable feature
[00:07:50] 
[00:07:50] error[E0277]: the size for values of type `Self` cannot be known at compilation time
[00:07:50]   --> librustc/mir/interpret/pointer.rs:50:21
[00:07:50]   --> librustc/mir/interpret/pointer.rs:50:21
[00:07:50]    |
[00:07:50] 50 |     fn offset<'tcx>(self, val: u64, i: u64) -> EvalResult<'tcx, u64> {
[00:07:50]    |
[00:07:50]    = help: the trait `std::marker::Sized` is not implemented for `Self`
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = help: consider adding a `where Self: std::marker::Sized` bound
[00:07:50]    = help: unsized locals are gated as an unstable feature
[00:07:50] 
[00:07:50] error[E0277]: the size for values of type `Self` cannot be known at compilation time
[00:07:50]   --> librustc/mir/interpret/pointer.rs:55:31
[00:07:50]   --> librustc/mir/interpret/pointer.rs:55:31
[00:07:50]    |
[00:07:50] 55 |     fn wrapping_signed_offset(self, val: u64, i: i64) -> u64 {
[00:07:50]    |
[00:07:50]    = help: the trait `std::marker::Sized` is not implemented for `Self`
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:07:50]    = help: consider adding a `where Self: std::marker::Sized` bound
[00:07:50]    = help: unsized locals are gated as an unstable feature
[00:07:50] 
[00:07:58] error: aborting due to 7 previous errors
[00:07:58] 
---
[00:07:58] travis_fold:end:stage0-rustc

[00:07:58] travis_time:end:stage0-rustc:start=1541421079570814179,finish=1541421255667987263,duration=176097173084

[00:07:58] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:07:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:58] Build completed unsuccessfully in 0:04:04
[00:07:58] Makefile:28: recipe for target 'all' failed
[00:07:58] Makefile:28: recipe for target 'all' failed
[00:07:58] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16a7108c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:274e2916:start=1541421256300368392,finish=1541421256303877578,duration=3509186
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03cc7800
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1abfcb19
travis_time:start:1abfcb19
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2bd90828
$ dmesg | grep -i kill
