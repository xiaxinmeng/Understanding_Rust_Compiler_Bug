plain
travis_time:end:0a6d3960:start=1558649915388906376,finish=1558649917764909927,duration=2376003551
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:48:32]    --> src/librustc_lint/lib.rs:23:9
[00:48:32]     |
[00:48:32] 23  | #![deny(internal)]
[00:48:32]     |         ^^^^^^^^
[00:48:32]     = note: #[deny(usage_of_qualified_ty)] implied by #[deny(internal)]
[00:48:32] error: aborting due to previous error
[00:48:32] 
[00:48:32] error: Could not compile `rustc_lint`.
[00:48:32] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:1fb7cd40:start=1558652929905999783,finish=1558652929911379303,duration=5379520
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c80a5bc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b74a770
travis_time:start:0b74a770
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:073d1150
$ dmesg | grep -i kill
