plain
travis_time:end:0459c7fe:start=1551255534238757826,finish=1551255536579016933,duration=2340259107
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:07]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:04:07]    Compiling libc v0.2.46
[00:04:07]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:08]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:13] error[E0433]: failed to resolve: could not find `ops` in `{{root}}`
[00:04:13] 
[00:04:13] error[E0433]: failed to resolve: could not find `option` in `{{root}}`
[00:04:13] 
[00:04:13] error[E0433]: failed to resolve: could not find `iter` in `{{root}}`
[00:04:13] 
[00:04:13] error[E0433]: failed to resolve: could not find `result` in `{{root}}`
[00:04:13] 
[00:04:13] error[E0433]: failed to resolve: could not find `convert` in `{{root}}`
[00:04:13]    Compiling compiler_builtins v0.1.5
[00:04:13]    Compiling cmake v0.1.33
[00:04:13]    Compiling backtrace-sys v0.1.27
[00:04:15]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:04:22]     |
[00:04:22] 492 |         for entry in entries {
[00:04:22]     |             ^^^^^ doesn't have a size known at compile-time
[00:04:22]     |
[00:04:22]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:22]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:22]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:22] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:22]    --> src/libcore/fmt/builders.rs:630:13
[00:04:22]     |
[00:04:22] 630 |         for entry in entries {
[00:04:22] 630 |         for entry in entries {
[00:04:22]     |             ^^^^^ doesn't have a size known at compile-time
[00:04:22]     |
[00:04:22]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:22]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:22]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:22] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:22]    --> src/libcore/fmt/builders.rs:791:13
[00:04:22]     |
[00:04:22]     |
[00:04:22] 791 |         for (k, v) in entries {
[00:04:22]     |
[00:04:22]     |
[00:04:22]     = help: within `(dyn fmt::Debug, dyn fmt::Debug)`, the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:22]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:22]     = note: required because it appears within the type `(dyn fmt::Debug, dyn fmt::Debug)`
[00:04:22]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:24] error: aborting due to 8 previous errors
[00:04:24] 
[00:04:24] Some errors occurred: E0277, E0433.
[00:04:24] For more information about an error, try `rustc --explain E0277`.
---
199324 ./obj/build/cache/2019-02-17
156148 ./src/llvm-project/clang
155940 ./obj/build/bootstrap/debug/incremental
141172 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn
141168 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn/s-f9vh14xnk9-1uubex4-okq6jwyp562e
108528 ./src/llvm-project/lldb
97552 ./src/llvm-project/clang/test
93200 ./.git
89964 ./src/llvm-emscripten/test/CodeGen
---
travis_time:end:08dfcd60:start=1551255812847456048,finish=1551255812852286612,duration=4830564
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:020d0909
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02f89dac
travis_time:start:02f89dac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1843ec58
$ dmesg | grep -i kill
