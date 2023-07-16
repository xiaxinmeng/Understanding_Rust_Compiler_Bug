plain
travis_time:end:11d4708c:start=1552080646867777205,finish=1552080764362249955,duration=117494472750
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:30:02]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:30:03] error: unexpected token: `,`
[00:30:03]     --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse.rs:4015:31
[00:30:03]      |
[00:30:03] 4015 |         let a = _mm_setr_ps(0., 0., 3., 4.);
[00:30:03] 
[00:30:03] error: aborting due to previous error
[00:30:03] 
[00:30:03] error: Could not compile `core`.
---
travis_time:end:047d6d11:start=1552082583427374113,finish=1552082583432344001,duration=4969888
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b959e5a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00c08760
travis_time:start:00c08760
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i3
