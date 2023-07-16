plain
travis_time:end:365a4000:start=1551247078774538330,finish=1551247081003360354,duration=2228822024
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:49]    Compiling libc v0.2.46
[00:03:49]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:49]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:50]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:55] error[E0433]: failed to resolve: could not find `ops` in `{{root}}`
[00:03:55] 
[00:03:55] error[E0433]: failed to resolve: could not find `option` in `{{root}}`
[00:03:55] 
[00:03:55] error[E0433]: failed to resolve: could not find `iter` in `{{root}}`
[00:03:55] 
[00:03:55] error[E0433]: failed to resolve: could not find `result` in `{{root}}`
[00:03:55] 
[00:03:55] error[E0433]: failed to resolve: could not find `convert` in `{{root}}`
[00:03:55]    Compiling compiler_builtins v0.1.5
[00:03:55]    Compiling cmake v0.1.33
[00:03:55]    Compiling backtrace-sys v0.1.27
[00:03:57]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:04:04]     |
[00:04:04] 492 |         for entry in entries {
[00:04:04]     |             ^^^^^ doesn't have a size known at compile-time
[00:04:04]     |
[00:04:04]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:04]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:04]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:04] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:04]    --> src/libcore/fmt/builders.rs:630:13
[00:04:04]     |
[00:04:04] 630 |         for entry in entries {
[00:04:04] 630 |         for entry in entries {
[00:04:04]     |             ^^^^^ doesn't have a size known at compile-time
[00:04:04]     |
[00:04:04]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:04]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:04]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:04] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:04]    --> src/libcore/fmt/builders.rs:791:13
[00:04:04]     |
[00:04:04]     |
[00:04:04] 791 |         for (k, v) in entries {
[00:04:04]     |
[00:04:04]     |
[00:04:04]     = help: within `(dyn fmt::Debug, dyn fmt::Debug)`, the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:04]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:04]     = note: required because it appears within the type `(dyn fmt::Debug, dyn fmt::Debug)`
[00:04:04]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:06] error: aborting due to 8 previous errors
[00:04:06] 
[00:04:06] Some errors occurred: E0277, E0433.
[00:04:06] For more information about an error, try `rustc --explain E0277`.
---
travis_time:end:2a9325e0:start=1551247340209332001,finish=1551247340213680467,duration=4348466
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0962875d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:026c6fa4
travis_time:start:026c6fa4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1810b474
$ dmesg | grep -i kill
