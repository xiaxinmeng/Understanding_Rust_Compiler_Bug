plain
travis_time:end:00cf6588:start=1556593117805165028,finish=1556593201005321216,duration=83200156188
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:42:12]    --> src/librustc_typeck/lib.rs:74:9
[00:42:12]     |
[00:42:12] 74  | #![deny(internal)]
[00:42:12]     |         ^^^^^^^^
[00:42:12]     = note: #[deny(ty_pass_by_reference)] implied by #[deny(internal)]
[00:42:12] error: aborting due to previous error
[00:42:12] 
[00:42:13] error: Could not compile `rustc_typeck`.
[00:42:13] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:1c19dcc0:start=1556596038269307017,finish=1556596038274198408,duration=4891391
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0aa27e08
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1612f632
travis_time:start:1612f632
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08c90524
$ dmesg | grep -i kill
