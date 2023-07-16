plain
travis_time:end:01dbc2c0:start=1553451669458924952,finish=1553451750264122637,duration=80805197685
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:15:28]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:15:45] error: unused variable: `ErrorReported`
[00:15:45]    --> src/librustc_mir/const_eval.rs:657:21
[00:15:45]     |
[00:15:45] 657 |                 Err(ErrorReported) => ErrorHandled::Reported,
[00:15:45]     |                     ^^^^^^^^^^^^^ help: consider prefixing with an underscore: `_ErrorReported`
[00:15:45]     = note: `-D unused-variables` implied by `-D warnings`
[00:15:45] 
[00:15:45] error: aborting due to previous error
[00:15:45] 
---
travis_time:end:252e2d4c:start=1553452930727091633,finish=1553452930734893005,duration=7801372
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08f99725
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:083ae290
travis_time:start:083ae290
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02f51bec
$ dmesg | grep -i kill
