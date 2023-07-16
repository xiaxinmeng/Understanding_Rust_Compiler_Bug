plain
travis_time:end:150c3430:start=1554037388575546144,finish=1554037480599845475,duration=92024299331
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:19:41]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:19:42] error[E0061]: this function takes 5 parameters but 3 parameters were supplied
[00:19:42]    --> src/librustc_lint/lib.rs:495:11
[00:19:42]     |
[00:19:42] 495 |     store.register_late_pass(sess, false, box TyKindUsage);
[00:19:42] 
[00:19:43] error: aborting due to previous error
[00:19:43] 
[00:19:43] For more information about this error, try `rustc --explain E0061`.
---
travis_time:end:10ee5f2a:start=1554038748633009014,finish=1554038748638082337,duration=5073323
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:17b738f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c1d9afc
travis_time:start:0c1d9afc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such f
