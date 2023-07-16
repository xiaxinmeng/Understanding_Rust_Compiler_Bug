plain
travis_time:end:050adab0:start=1554838355770233319,finish=1554838358021494345,duration=2251261026
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:05:32] .................................................................................................... 2600/5530
[01:05:36] .................................................................................................... 2700/5530
[01:05:40] .................................................................................................... 2800/5530
[01:05:44] .................................................................................................... 2900/5530
[01:05:49] .......................................................................F..F......................... 3000/5530
[01:05:52] ...................F.F.............................................................................. 3100/5530
[01:06:00] .................................................................................................... 3300/5530
[01:06:03] ..............................i..................................................................... 3400/5530
[01:06:07] .................................................................................................... 3500/5530
[01:06:10] ....ii...i..ii...................................................................................... 3600/5530
---
[01:07:25] failures:
[01:07:25] 
[01:07:25] ---- [ui] ui/issues/issue-54974.rs stdout ----
[01:07:25] 
[01:07:25] error: test compilation failed although it shouldn't!
[01:07:25] status: exit code: 1
[01:07:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-54974.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54974/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54974/auxiliary" "-A" "unused"
[01:07:25] ------------------------------------------
[01:07:25] 
[01:07:25] ------------------------------------------
[01:07:25] stderr:
[01:07:25] stderr:
[01:07:25] ------------------------------------------
[01:07:25] {"message":"type annotations needed","code":{"code":"E0282","explanation":"\nThis error indicates that type inference did not result in one unique possible\ntype, and extra information is required. In most cases this can be provided\nby adding a type annotation. Sometimes you need to specify a generic type\nparameter manually.\n\nA common example is the `collect` method on `Iterator`. It has a generic type\nparameter with a `FromIterator` bound, which for a `char` iterator is\nimplemented by `Vec` and `String` among others. Consider the following snippet\nthat reverses the characters of a string:\n\n