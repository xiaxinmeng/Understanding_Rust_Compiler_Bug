plain
travis_time:end:22da64cc:start=1559615694654688155,finish=1559615794894299143,duration=100239610988
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:01]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:04:03] error[E0093]: unrecognized intrinsic function: `unchecked_add`
[00:04:03]     --> src/libcore/intrinsics.rs:1246:5
[00:04:03]      |
[00:04:03] 1246 |     pub fn unchecked_add<T>(x: T, y: T) -> T;
[00:04:03]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unrecognized intrinsic
[00:04:03] 
[00:04:03] error[E0093]: unrecognized intrinsic function: `unchecked_sub`
[00:04:03]      |
[00:04:03]      |
[00:04:03] 1251 |     pub fn unchecked_sub<T>(x: T, y: T) -> T;
[00:04:03]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unrecognized intrinsic
[00:04:03] 
[00:04:03] error[E0093]: unrecognized intrinsic function: `unchecked_mul`
[00:04:03]      |
[00:04:03]      |
[00:04:03] 1256 |     pub fn unchecked_mul<T>(x: T, y: T) -> T;
[00:04:03]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unrecognized intrinsic
[00:04:03]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:03]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:04]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:04]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
---
travis_time:end:07656ac8:start=1559616055171504766,finish=1559616055176159209,duration=4654443
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f54cdec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0adac73c
travis_time:start:0adac73c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5

