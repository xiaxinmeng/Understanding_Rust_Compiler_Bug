plain
travis_time:end:0176a7d2:start=1561234408016249841,finish=1561234408832294141,duration=816044300
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:43:16]    --> src/librustc_ast_borrowck/lib.rs:4:9
[00:43:16]     |
[00:43:16] 4   | #![deny(rust_2018_idioms)]
[00:43:16]     |         ^^^^^^^^^^^^^^^^
[00:43:16]     = note: #[deny(explicit_outlives_requirements)] implied by #[deny(rust_2018_idioms)]
[00:43:16] error: aborting due to previous error
[00:43:16] 
[00:43:17] error: Could not compile `rustc_ast_borrowck`.
[00:43:17] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:00d1944e:start=1561237190124343615,finish=1561237190129058476,duration=4714861
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:20a1a32c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01c9f2db
travis_time:start:01c9f2db
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12ecc500
$ dmesg | grep -i kill
