plain
travis_time:end:062fb614:start=1561300587232598359,finish=1561300588155680723,duration=923082364
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:51]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:52]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:52]    Compiling autocfg v0.1.4
[00:04:54]    Compiling backtrace v0.3.29
[00:04:56] error: `#[deprecated]` cannot be used in staged api, use `#[rustc_deprecated]` instead
[00:04:56]     |
[00:04:56]     |
[00:04:56] 307 | / macro_rules! r#try {
[00:04:56] 308 | |     ($expr:expr) => (match $expr {
[00:04:56] 309 | |         $crate::result::Result::Ok(val) => val,
[00:04:56] 310 | |         $crate::result::Result::Err(err) => {
[00:04:56] ...   |
[00:04:56] 314 | |     ($expr:expr,) => ($crate::r#try!($expr));
[00:04:56]     | |_^
[00:04:56] 
[00:04:57]    Compiling compiler_builtins v0.1.16
[00:04:57]    Compiling cmake v0.1.38
---
travis_time:end:16e7c478:start=1561300907420506664,finish=1561300907425008089,duration=4501425
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:27b14ed2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0336e59c
travis_time:start:0336e59c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:001e6e20
$ dmesg | grep -i kill
