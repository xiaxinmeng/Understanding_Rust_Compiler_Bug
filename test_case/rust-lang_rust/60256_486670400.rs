plain
travis_time:end:1099e3f0:start=1556198265995629787,finish=1556198268549277999,duration=2553648212
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:11]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:14] error[E0433]: failed to resolve: use of undeclared type or module `core`
[00:04:14]     --> src/libcore/option.rs:1435:23
[00:04:14]      |
[00:04:14] 1435 |         self.and_then(core::convert::identity)
[00:04:14] 
[00:04:16]    Compiling compiler_builtins v0.1.10
[00:04:16]    Compiling cmake v0.1.38
[00:04:16]    Compiling backtrace-sys v0.1.27
---
travis_time:end:0949c493:start=1556198549555944115,finish=1556198549560357787,duration=4413672
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:029a6678
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b09f5f6
travis_time:start:0b09f5f6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bdc9a20
$ dmesg | grep -i kill
