plain
travis_time:end:030ad52c:start=1555497453158385892,finish=1555497453911678869,duration=753292977
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:16:55] failures:
[01:16:55] 
[01:16:55] ---- [run-pass] run-pass/unsized-locals/unsized-index.rs stdout ----
[01:16:55] 
[01:16:55] error: test compilation failed although it shouldn't!
[01:16:55] status: exit code: 1
[01:16:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/unsized-locals/unsized-index.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/unsized-locals/unsized-index/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/unsized-locals/unsized-index/auxiliary"
[01:16:55] ------------------------------------------
[01:16:55] 
[01:16:55] ------------------------------------------
[01:16:55] stderr:
[01:16:55] stderr:
[01:16:55] ------------------------------------------
[01:16:55] {"message":"method `index` is not a member of trait `ops::IndexMut`","code":{"code":"E0407","explanation":"\nA definition of a method not in the implemented trait was given in a trait\nimplementation.\n\nErroneous code example:\n\n