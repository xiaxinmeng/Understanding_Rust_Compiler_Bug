plain
travis_time:end:0e61dcd3:start=1554144645283948378,finish=1554144646313798924,duration=1029850546
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:05:07]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:09] error: unused macro definition
[00:05:09]   --> src/libstd/sys_common/mod.rs:31:1
[00:05:09]    |
[00:05:09] 31 | / macro_rules! rtunwrap {
[00:05:09] 32 | |     ($ok:ident, $e:expr) => (if let $ok(v) = $e {
[00:05:09] 34 | |     } else {
[00:05:09] 34 | |     } else {
[00:05:09] 35 | |         rtabort!(concat!("unwrap failed: ", stringify!($e)));
[00:05:09] 37 | | }
[00:05:09]    | |_^
[00:05:09]    |
[00:05:09]    = note: `-D unused-macros` implied by `-D warnings`
---
travis_time:end:0d36a400:start=1554144969314693911,finish=1554144969321449282,duration=6755371
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bb30d58
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0316b5d6
travis_time:start:0316b5d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b83e852
$ dmesg | grep -i kill
