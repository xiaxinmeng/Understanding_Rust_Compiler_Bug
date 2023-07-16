plain
travis_time:end:0798357c:start=1552219861258666612,finish=1552219935991441339,duration=74732774727
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:21]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:06:21] error: unused variable: `lo`
[00:06:21]    --> src/libsyntax/parse/attr.rs:270:13
[00:06:21]     |
[00:06:21] 270 |         let lo = self.span;
[00:06:21]     |             ^^ help: consider prefixing with an underscore: `_lo`
[00:06:21]     = note: `-D unused-variables` implied by `-D warnings`
[00:06:21] 
[00:06:21] error: aborting due to previous error
[00:06:21] 
---
travis_time:end:21a3fa3d:start=1552220331235791963,finish=1552220331240240250,duration=4448287
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d4b6724
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3630b76e
travis_time:start:3630b76e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or
