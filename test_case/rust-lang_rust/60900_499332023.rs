plain
travis_time:end:09e0a2e8:start=1559788856592022092,finish=1559788945367792987,duration=88775770895
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:29:49]    Compiling synstructure v0.10.2
[00:30:13]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:30:21]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:30:26]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:30:27] error[E0283]: type annotations required: cannot resolve `dyn emitter::Emitter + rustc_data_structures::sync::Send: rustc_data_structures::sync::Send`
[00:30:27]     |
[00:30:27] 392 |             e,
[00:30:27]     |             ^
[00:30:27]     |
---
166976 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
156504 ./src/llvm-project/clang
150608 ./obj/build/bootstrap/debug/incremental
135324 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1
135320 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1/s-fcwd815idg-arple9-3h3kftrpru154
120468 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
108532 ./src/llvm-project/lldb
98164 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
98036 ./obj/build/x-analysis/test_data
---
24124 ./src/llvm-projec797459669107,finish=1559790797467187798,duration=7518691
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1741906e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a9543d3
$ dmesg | grep -i kill
