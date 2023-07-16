plain
travis_time:end:0be993d2:start=1552524552108732045,finish=1552524655288034000,duration=103179301955
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:14:02] ............................iiiii................................................................... 1100/5468
[01:14:05] .................................................................................................... 1200/5468
[01:14:08] .................................................................................................... 1300/5468
[01:14:11] .................................................................................................... 1400/5468
[01:14:14] ..............................................................F..................................... 1500/5468
[01:14:21] ..................................i................................................................. 1700/5468
[01:14:24] .................................................................................................... 1800/5468
[01:14:29] .................................................................................................... 1900/5468
[01:14:33] .................................................................................................... 2000/5468
---
[01:17:00] 
[01:17:00] 1 error[E0725]: the feature `rustc_const_unstable` is not in the list of allowed features
[01:17:00] 2   --> $DIR/allow-features.rs:6:12
[01:17:00] 3    |
[01:17:00] - LL | #![feature(rustc_const_unstable)] //~ ERROR not in the list of allowed features
[01:17:00] + LL | #![feature(rustc_const_unstable)]
[01:17:00] 6 
[01:17:00] 7 error: aborting due to previous error
[01:17:00] 
[01:17:00] 
[01:17:00] 
[01:17:00] The actual stderr differed from the expected stderr.
[01:17:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/allow-features/allow-features.stderr
[01:17:00] To update references, rerun the tests and pass the `--bless` flag
[01:17:00] To only update this specific test, also pass `--test-args feature-gate/allow-features.rs`
[01:17:00] error: 1 errors occurred comparing output.
[01:17:00] status: exit code: 1
[01:17:00] status: exit code: 1
[01:17:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate/allow-features.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/allow-features/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "allow_features=rustc_diagnostic_macros" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/allow-features/auxiliary" "-A" "unused"
[01:17:00] ------------------------------------------
[01:17:00] 
[01:17:00] ------------------------------------------
[01:17:00] stderr:
[01:17:00] stderr:
[01:17:00] ------------------------------------------
[01:17:00] {"message":"the feature `rustc_const_unstable` is not in the list of allowed features","code":{"code":"E0725","explanation":"\nA feature attribute named a feature that was disallowed in the compiler\ncommand line flags.\n\nErroneous code example:\n\n