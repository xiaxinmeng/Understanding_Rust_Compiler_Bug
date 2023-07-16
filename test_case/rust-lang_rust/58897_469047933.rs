plain
travis_time:end:107bea0c:start=1551632293586616737,finish=1551632294502033328,duration=915416591
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:06:56] travis_fold:end:stage2-error_index_generator

[01:06:56] travis_time:end:stage2-error_index_generator:start=1551636156401021597,finish=1551636320987460941,duration=164586439344

[01:06:56] /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/error_index_generator: error while loading shared libraries: librustc_driver-920677224a87bbf0.so: cannot open shared object file: No such file or directory
[01:06:56] 
[01:06:56] 
[01:06:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/error_index_generator" "html" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc/error-index.html"
[01:06:56] 
[01:06:56] 
[01:06:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:06:56] Build completed unsuccessfully in 0:11:10
