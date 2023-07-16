plain
travis_time:end:0098efa0:start=1552347712761695321,finish=1552347713665627457,duration=903932136
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:17:52] ............................................................................ii...................... 2300/2957
[01:18:08] ..............................................ii.................................................... 2400/2957
[01:18:22] .................................................................................................... 2500/2957
[01:18:47] .................................................................................................... 2600/2957
[01:19:05] .............................................F...................................................... 2700/2957
[01:19:26] .................................................................................................... 2900/2957
[01:19:35] .........................................................
[01:19:35] failures:
[01:19:35] 
[01:19:35] 
[01:19:35] ---- [run-pass] run-pass/traits/trait-alias-syntax.rs stdout ----
[01:19:35] 
[01:19:35] error: test compilation failed although it shouldn't!
[01:19:35] status: exit code: 1
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/traits/trait-alias-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/traits/trait-alias-syntax/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/traits/trait-alias-syntax/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"associated type `Out` not found for `Self`","code":{"code":"E0220","explanation":"\nYou used an associated type which isn't defined in the trait.\nErroneous code example:\n\n