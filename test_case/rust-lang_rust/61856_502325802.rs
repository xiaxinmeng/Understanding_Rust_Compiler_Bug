plain
travis_time:end:01f9d1c4:start=1560562370341558251,finish=1560562371141575900,duration=800017649
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:29:42] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:29:42] 
[00:29:42] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:29:42] 
[00:29:42] note: compiler flags: -Z unstable-options -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 --crate-type lib
[00:29:42] note: some of the compiler flags provided by cargo are hidden
[00:29:42] 
[00:29:42] error: Could not compile `itertools`.
[00:29:42] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:0adc6530:start=1560564168850226104,finish=1560564168855099820,duration=4873716
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04630a34
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22f36f66
travis_time:start:22f36f66
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d830702
$ dmesg | grep -i kill
