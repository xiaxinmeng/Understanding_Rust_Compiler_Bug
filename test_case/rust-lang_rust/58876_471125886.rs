plain
travis_time:end:230ad230:start=1552090226561587353,finish=1552090299205771288,duration=72644183935
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:26:55]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:26:55] error: unexpected token: `,`
[00:26:55]     --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse.rs:4015:31
[00:26:55]      |
[00:26:55] 4015 |         let a = _mm_setr_ps(0., 0., 3., 4.);
[00:26:55] 
[00:26:55] error: aborting due to previous error
[00:26:55] 
[00:26:55] error: Could not compile `core`.
---
travis_time:end:202deb44:start=1552091928962841357,finish=1552091928967726741,duration=4885384
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:154e0ee8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:056bef5a
travis_time:start:056bef5a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:036aead4
$ dmesg | grep -i kill
