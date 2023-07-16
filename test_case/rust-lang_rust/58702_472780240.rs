plain
travis_time:end:0aa16620:start=1552557039363224160,finish=1552557041657701670,duration=2294477510
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:43]    Compiling libc v0.2.46
[00:03:43]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:43]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:44]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:48] error[E0433]: failed to resolve: could not find `ops` in `{{root}}`
[00:03:48] 
[00:03:48] error[E0433]: failed to resolve: could not find `option` in `{{root}}`
[00:03:48] 
[00:03:48] error[E0433]: failed to resolve: could not find `iter` in `{{root}}`
[00:03:48] 
[00:03:48] error[E0433]: failed to resolve: could not find `result` in `{{root}}`
[00:03:48] 
[00:03:48] error[E0433]: failed to resolve: could not find `convert` in `{{root}}`
[00:03:49]    Compiling compiler_builtins v0.1.5
[00:03:49]    Compiling cmake v0.1.33
[00:03:49]    Compiling backtrace-sys v0.1.27
[00:03:52]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:03:58]     |
[00:03:58] 492 |         for entry in entries {
[00:03:58]     |             ^^^^^ doesn't have a size known at compile-time
[00:03:58]     |
[00:03:58]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:03:58]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:03:58]     = note: the left-hand-side of an assignment must have a statically known size
[00:03:58] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:03:58]    --> src/libcore/fmt/builders.rs:630:13
[00:03:58]     |
[00:03:58] 630 |         for entry in entries {
[00:03:58] 630 |         for entry in entries {
[00:03:58]     |             ^^^^^ doesn't have a size known at compile-time
[00:03:58]     |
[00:03:58]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:03:58]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:03:58]     = note: the left-hand-side of an assignment must have a statically known size
[00:03:58] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:03:58]    --> src/libcore/fmt/builders.rs:791:13
[00:03:58]     |
[00:03:58]     |
[00:03:58] 791 |         for (k, v) in entries {
[00:03:58]     |
[00:03:58]     |
[00:03:58]     = help: within `(dyn fmt::Debug, dyn fmt::Debug)`, the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:03:58]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:03:58]     = note: required because it appears within the type `(dyn fmt::Debug, dyn fmt::Debug)`
[00:03:58]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:00] error: aborting due to 8 previous errors
[00:04:00] 
[00:04:00] Some errors occurred: E0277, E0433.
[00:04:00] For more information about an error, try `rustc --explain E0277`.
---
travis_time:end:0d09cc02:start=1552557293604811279,finish=1552557293612982703,duration=8171424
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04878b8e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b9f6768
travis_time:start:1b9f6768
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ca26718
$ dmesg | grep -i kill
