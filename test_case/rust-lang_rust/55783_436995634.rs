plain
travis_time:end:0832e7ad:start=1541683819506719552,finish=1541683882212720476,duration=62706000924
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:21]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:22]    Compiling alloc_system v0.0.0 (/checkout/src/liballoc_system)
[00:04:22]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:26]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:27] error: `#[deprecated]` cannot be used in staged api, use `#[rustc_deprecated]` instead
[00:04:27]    |
[00:04:27]    |
[00:04:27] 71 | / pub struct Select {
[00:04:27] 72 | |     inner: UnsafeCell<SelectInner>,
[00:04:27] 73 | |     next_id: Cell<usize>,
[00:04:27]    | |_^
[00:04:27] 
[00:04:27] 
[00:04:27] error: `#[deprecated]` cannot be used in staged api, use `#[rustc_deprecated]` instead
[00:04:27]     |
[00:04:27] 403 | / macro_rules! select {
[00:04:27] 404 | |     (
[00:04:27] 404 | |     (
[00:04:27] 405 | |         $($name:pat = $rx:ident.$meth:ident() => $code:expr),+
[00:04:27] 406 | |     ) => ({
[00:04:27] 416 | |     })
[00:04:27] 417 | | }
[00:04:27]     | |_^
[00:04:27] 
---
travis_time:end:00045a02:start=1541684163684422453,finish=1541684163690039770,duration=5617317
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:006a711b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0035813a
travis_time:start:0035813a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:st
