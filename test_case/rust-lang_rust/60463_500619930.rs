plain
travis_time:end:1c2ce40c:start=1560205785648816071,finish=1560205786514098603,duration=865282532
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:36]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:54] error[E0425]: cannot find value `transparent_enums` in module `sym`
[00:07:54]    --> src/libsyntax/feature_gate.rs:565:14
[00:07:54]     |
[00:07:54] 565 |     (active, transparent_enums, "1.37.0", Some(60405), None),
[00:07:54]     |              ^^^^^^^^^^^^^^^^^ not found in `sym`
[00:07:54] error[E0425]: cannot find value `transparent_unions` in module `sym`
[00:07:54]    --> src/libsyntax/feature_gate.rs:568:14
[00:07:54]     |
[00:07:54]     |
[00:07:54] 568 |     (active, transparent_unions, "1.37.0", Some(60405), None),
[00:07:54]     |              ^^^^^^^^^^^^^^^^^^ not found in `sym`
[00:07:59] error: aborting due to 2 previous errors
[00:07:59] 
[00:07:59] For more information about this error, try `rustc --explain E0425`.
[00:07:59] error: Could not compile `syntax`.
---
156496 ./src/llvm-project/clang
150572 ./obj/build/bootstrap/debug/incremental
143076 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
135300 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1
135296 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1/s-fd1oq944tb-1gzkvy5-19ca4iajxr4qm
108532 ./src/llvm-project/lldb
103496 ./.git
98060 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
97584 ./src/llvm-project/clang/test
