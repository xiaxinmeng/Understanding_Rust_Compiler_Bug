plain
travis_time:end:006e0f20:start=1559626098691822325,finish=1559626099515009203,duration=823186878
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:53]    Compiling autocfg v0.1.4
[00:04:54] error: couldn't read src/libcore/panic.md: No such file or directory (os error 2)
[00:04:54]  --> src/libcore/macros/mod.rs:1:17
[00:04:54]   |
[00:04:54] 1 | #[doc(include = "panic.md")]
[00:04:54]   |                 ^^^^^^^^^^ couldn't read file
[00:04:54]   |
[00:04:54]   = help: external doc paths are relative to the crate root
[00:04:54]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:55]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:56]    Compiling backtrace v0.3.25
[00:05:00]    Compiling compiler_builtins v0.1.15
---
travis_time:end:048e1bc6:start=1559626421812777979,finish=1559626421817262760,duration=4484781
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c76e4c5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f1ca043
travis_time:start:0f1ca043
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:en
