plain
travis_time:end:14062c86:start=1556197677091790503,finish=1556197679129041419,duration=2037250916
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:30]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:33] error[E0433]: failed to resolve: use of undeclared type or module `convert`
[00:04:33]     --> src/libcore/option.rs:1435:23
[00:04:33]      |
[00:04:33] 1435 |         self.and_then(convert::identity)
[00:04:33]      |                       ^^^^^^^ use of undeclared type or module `convert`
[00:04:35]    Compiling compiler_builtins v0.1.10
[00:04:35]    Compiling cmake v0.1.38
[00:04:35]    Compiling backtrace-sys v0.1.27
[00:04:38]    Compiling std v0.0.0 (/checkout/src/libstd)
---
travis_time:end:0341417c:start=1556197978828891349,finish=1556197978833763162,duration=4871813
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a3a36ed
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:162208ba
travis_time:start:162208ba
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:161d5928
$ dmesg | grep -i kill
