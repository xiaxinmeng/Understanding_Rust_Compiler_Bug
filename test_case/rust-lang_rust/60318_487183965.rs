plain
travis_time:end:35499234:start=1556308605357371460,finish=1556308606099424323,duration=742052863
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:08]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:24] error[E0711]: feature `try_from` is declared stable since 1.36.0, but was previously declared stable since 1.34.0
[00:04:24]   --> src/libcore/array.rs:75:1
[00:04:24]    |
[00:04:24] 75 | #[stable(feature = "try_from", since = "1.36.0")]
[00:04:24] 
[00:04:24] error: aborting due to previous error
[00:04:24] 
[00:04:24] For more information about this error, try `rustc --explain E0711`.
---
travis_time:end:0d96e1d0:start=1556308882904880792,finish=1556308882910133810,duration=5253018
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0434e5d6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13db7300
travis_time:start:13db7300
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1e6afcfc
$ dmesg | grep -i kill
