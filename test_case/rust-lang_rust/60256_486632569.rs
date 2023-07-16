plain
travis_time:end:0bcb3092:start=1556191101801299075,finish=1556191102583006167,duration=781707092
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:05]    Compiling backtrace-sys v0.1.27
[00:04:06] error[E0547]: missing 'issue'
[00:04:06]     --> src/libcore/option.rs:1417:1
[00:04:06]      |
[00:04:06] 1417 | #[unstable(feature = "option_flattening")]
[00:04:06] 
[00:04:08]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:04:08]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:08]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
---
travis_time:end:07bae5bd:start=1556191372703481790,finish=1556191372707937603,duration=4455813
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:25411a52
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0783980d
travis_time:start:0783980d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22c7dfd0
$ dmesg | grep -i kill
