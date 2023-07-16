plain
travis_time:end:29aa2f1f:start=1552598511995536208,finish=1552598512909902377,duration=914366169
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:13:00] .................................................................................................... 1900/5467
[01:13:04] .................................................................................................... 2000/5467
[01:13:07] .................................................................i.................................. 2100/5467
[01:13:11] .................................................................................................... 2200/5467
[01:13:15] .....................................................F.............F................................ 2300/5467
[01:13:24] .................................................................................................... 2500/5467
[01:13:27] .................................................................................................... 2600/5467
[01:13:31] .................................................................................................... 2700/5467
[01:13:36] .................................................................................................... 2800/5467
---
[01:15:17] 
[01:15:17] ---- [ui] ui/issues/issue-21950.rs stdout ----
[01:15:17] diff of stderr:
[01:15:17] 
[01:15:17] - error[E0393]: the type parameter `RHS` must be explicitly specified
[01:15:17] + error[E0393]: the type parameter `Rhs` must be explicitly specified
[01:15:17] 3    |
[01:15:17] 4 LL |             &Add;
[01:15:17] 
[01:15:17] -    |              ^^^ missing reference to `RHS`
[01:15:17] -    |              ^^^ missing reference to `RHS`
[01:15:17] +    |              ^^^ missing reference to `Rhs`
[01:15:17] 6    |
[01:15:17] 7    = note: because of the default `Self` reference, type parameters must be specified on object types
[01:15:17] 
[01:15:17] 
[01:15:17] The actual stderr differed from the expected stderr.
[01:15:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21950/issue-21950.stderr
[01:15:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21950/issue-21950.stderr
[01:15:17] To update references, rerun the tests and pass the `--bless` flag
[01:15:17] To only update this specific test, also pass `--test-args issues/issue-21950.rs`
[01:15:17] error: 1 errors occurred comparing output.
[01:15:17] status: exit code: 1
[01:15:17] status: exit code: 1
[01:15:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21950.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21950/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21950/auxiliary" "-A" "unused"
[01:15:17] ------------------------------------------
[01:15:17] 
[01:15:17] ------------------------------------------
[01:15:17] stderr:
[01:15:17] stderr:
[01:15:17] ------------------------------------------
[01:15:17] {"message":"the type parameter `Rhs` must be explicitly specified","code":{"code":"E0393","explanation":"\nA type parameter which references `Self` in its default value was not specified.\nExample of erroneous code:\n\n