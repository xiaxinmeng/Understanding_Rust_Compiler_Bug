plain
travis_time:end:0ddc6458:start=1560978219817211093,finish=1560978220675733124,duration=858522031
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:40:18]    --> src/librustc_ast_borrowck/lib.rs:4:9
[00:40:18]     |
[00:40:18] 4   | #![deny(rust_2018_idioms)]
[00:40:18]     |         ^^^^^^^^^^^^^^^^
[00:40:18]     = note: #[deny(explicit_outlives_requirements)] implied by #[deny(rust_2018_idioms)]
[00:40:18] error: aborting due to previous error
[00:40:18] 
[00:40:18] error: Could not compile `rustc_ast_borrowck`.
[00:40:18] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:055f0864:start=1560980938363642735,finish=1560980938371029584,duration=7386849
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c0eaac4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17699b68
$ dmesg | grep -i kill
