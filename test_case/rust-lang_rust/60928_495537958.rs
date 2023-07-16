plain
travis_time:end:0e00a7aa:start=1558688467617550443,finish=1558688470662037256,duration=3044486813
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:15:47]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:15:49] error[E0433]: failed to resolve: use of undeclared type or module `AggregateKind`
[00:15:49]    --> src/librustc_mir/transform/const_prop.rs:537:38
[00:15:49]     |
[00:15:49] 537 |                             Box::new(AggregateKind::Tuple),
[00:15:49]     |                                      ^^^^^^^^^^^^^ use of undeclared type or module `AggregateKind`
[00:16:03] error: aborting due to previous error
[00:16:03] 
[00:16:03] For more information about this error, try `rustc --explain E0433`.
[00:16:03] error: Could not compile `rustc_mir`.
---
travis_time:end:16f2f0fa:start=1558689610583775473,finish=1558689610589161448,duration=5385975
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2de6ab26
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07b8df90
travis_time:start:07b8df90
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12271400
$ dmesg | grep -i kill
