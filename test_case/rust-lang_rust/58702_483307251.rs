plain
travis_time:end:081c89a8:start=1555342805052977765,finish=1555342809705343975,duration=4652366210
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:00]    Compiling libc v0.2.51
[00:04:00]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:04:00]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:01]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:05] error[E0433]: failed to resolve: could not find `ops` in `{{root}}`
[00:04:05] 
[00:04:05] error[E0433]: failed to resolve: could not find `result` in `{{root}}`
[00:04:05] 
[00:04:05] error[E0433]: failed to resolve: could not find `convert` in `{{root}}`
[00:04:05] 
[00:04:05] error[E0433]: failed to resolve: could not find `option` in `{{root}}`
[00:04:05] 
[00:04:05] error[E0433]: failed to resolve: could not find `iter` in `{{root}}`
[00:04:06]    Compiling compiler_builtins v0.1.10
[00:04:06]    Compiling cmake v0.1.38
[00:04:06]    Compiling backtrace-sys v0.1.27
[00:04:09]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:04:15] 
[00:04:15] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:15]    --> src/libcore/fmt/builders.rs:770:13
[00:04:15]     |
[00:04:15] 770 |         for (k, v) in entries {
[00:04:15]     |
[00:04:15]     |
[00:04:15]     = help: within `(dyn fmt::Debug, dyn fmt::Debug)`, the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:15]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:15]     = note: required because it appears within the type `(dyn fmt::Debug, dyn fmt::Debug)`
[00:04:15]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:17] error: aborting due to 8 previous errors
[00:04:17] 
[00:04:17] Some errors occurred: E0277, E0433.
[00:04:17] For more information about an error, try `rustc --explain E0277`.
---
travis_time:end:162b8b20:start=1555343077832579438,finish=1555343077837291776,duration=4712338
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b6bed7f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06b5f590
travis_time:start:06b5f590
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:055f8fd4
$ dmesg | grep -i kill
