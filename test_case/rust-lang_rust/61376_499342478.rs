plain
travis_time:end:0ec8c708:start=1559794433813545531,finish=1559794521789619111,duration=87976073580
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:14]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:14]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:14]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:15]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:15] error[E0614]: type `ops::range::Bound<&T>` cannot be dereferenced
[00:04:15]    --> src/libcore/ops/range.rs:714:15
[00:04:15] 714 |         match *self {
[00:04:15]     |               ^^^^^
[00:04:15] 
[00:04:20] error: aborting due to previous error
---
travis_time:end:047caa98:start=1559794792975504018,finish=1559794792980609582,duration=5105564
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bc46370
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:083d402c
travis_time:start:083d402c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:244d78e8
$ dmesg | grep -i kill
