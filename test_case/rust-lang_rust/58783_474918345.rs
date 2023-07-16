plain
travis_time:end:034d753c:start=1553099378419362962,finish=1553099460147314145,duration=81727951183
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:36]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:39] error[E0308]: mismatched types
[00:03:39]     --> src/libcore/intrinsics.rs:1346:63
[00:03:39]      |
[00:03:39] 1346 | fn overlaps<T>(src: *const T, dst: *const T, count: usize) -> bool {
[00:03:39]      |    -------- this function's body doesn't return               ^^^^ expected bool, found ()
[00:03:39] 1356 |     diff > size;
[00:03:39]      |                - help: consider removing this semicolon
[00:03:39]      |
[00:03:39]      = note: expected type `bool`
---
travis_time:end:06beb96c:start=1553099694942060182,finish=1553099694947569578,duration=5509396
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:29b306fe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19918110
travis_time:start:19918110
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05583140
$ dmesg | grep -i kill
