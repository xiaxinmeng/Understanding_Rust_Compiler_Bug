plain
travis_time:end:02b5807c:start=1546559531273178824,finish=1546559533486623435,duration=2213444611
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:55:38] .................................................................................................... 2100/5232
[00:55:42] .................................................................................................... 2200/5232
[00:55:45] .................................................................................................... 2300/5232
[00:55:50] .................................................................................................... 2400/5232
[00:55:52] ..................................F..F.............................................................. 2500/5232
[00:56:00] .................................................................................................... 2700/5232
[00:56:04] .................................................................................................... 2800/5232
[00:56:07] .................................................................................................... 2900/5232
[00:56:10] .................................................................................................... 3000/5232
---
[00:57:24] 
[00:57:24] ---- [ui] ui/issues/issue-33140.rs stdout ----
[00:57:24] diff of stderr:
[00:57:24] 
[00:57:24] - error[E0119]: conflicting implementations of trait `Trait` for type `(dyn std::marker::Sync + std::marker::Send + 'static)`:
[00:57:24] + error[E0119]: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`:
[00:57:24] 3    |
[00:57:24] 3    |
[00:57:24] 4 LL | impl Trait for dyn Send + Sync {
[00:57:24] 
[00:57:24] 5    | ------------------------------ first implementation here
[00:57:24] 6 ...
[00:57:24] 7 LL | impl Trait for dyn Sync + Send {
[00:57:24] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Sync + std::marker::Send + 'static)`
[00:57:24] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
[00:57:24] 9 
[00:57:24] - error[E0119]: conflicting implementations of trait `Trait2` for type `(dyn std::marker::Sync + std::marker::Send + 'static)`:
[00:57:24] + error[E0119]: conflicting implementations of trait `Trait2` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`:
[00:57:24] 12    |
[00:57:24] 12    |
[00:57:24] 13 LL | impl Trait2 for dyn Send + Sync {
[00:57:24] 
[00:57:24] 14    | ------------------------------- first implementation here
[00:57:24] 15 ...
[00:57:24] 16 LL | impl Trait2 for dyn Sync + Send + Sync {
[00:57:24] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Sync + std::marker::Send + 'static)`
[00:57:24] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
[00:57:24] 18 
[00:57:24] 19 error[E0592]: duplicate definitions with name `abc`
[00:57:24] 
[00:57:24] 
[00:57:24] The actual stderr differed from the expected stderr.
[00:57:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33140/issue-33140.stderr
[00:57:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33140/issue-33140.stderr
[00:57:24] To update references, rerun the tests and pass the `--bless` flag
[00:57:24] To only update this specific test, also pass `--test-args issues/issue-33140.rs`
[00:57:24] error: 1 errors occurred comparing output.
[00:57:24] status: exit code: 1
[00:57:24] status: exit code: 1
[00:57:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-33140.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33140/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33140/auxiliary" "-A" "unused"
[00:57:24] ------------------------------------------
[00:57:24] 
[00:57:24] ------------------------------------------
[00:57:24] stderr:
[00:57:24] stderr:
[00:57:24] ------------------------------------------
[00:57:24] {"message":"conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`:","code":{"code":"E0119","explanation":"\nThere are conflicting trait implementations for the same type.\nExample of erroneous code:\n\n