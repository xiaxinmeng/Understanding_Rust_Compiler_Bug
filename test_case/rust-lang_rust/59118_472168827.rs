plain
travis_time:end:06c7f910:start=1552418315642024004,finish=1552418391967434985,duration=76325410981
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:09:12] .................................................................................................... 2600/5452
[01:09:16] .................................................................................................... 2700/5452
[01:09:21] .................................................................................................... 2800/5452
[01:09:25] .................................................................................................... 2900/5452
[01:09:29] .......................................................................F............................ 3000/5452
[01:09:36] .................................................................................................... 3200/5452
[01:09:39] ..............................................................................i..................... 3300/5452
[01:09:43] .................................................................................................... 3400/5452
[01:09:46] ....................................................ii...i...ii..................................... 3500/5452
---
[01:10:59] 
[01:10:59] ---- [ui] ui/issues/issue-59029-1.rs stdout ----
[01:10:59] diff of stderr:
[01:10:59] 
[01:10:59] - error: associated type `Res` not found for `Self`
[01:10:59] -   --> $DIR/issue-59029.rs:5:52
[01:10:59] + error[E0220]: associated type `Res` not found for `Self`
[01:10:59] 3    |
[01:10:59] 3    |
[01:10:59] 4 LL | trait MkSvc<Target, Req> = Svc<Target> where Self::Res: Svc<Req>;
[01:10:59] 5    |                                              ^^^^^^^^^ associated type `Res` not found
[01:10:59] 6 
[01:10:59] 7 error: aborting due to previous error
[01:10:59] + 
[01:10:59] + For more information about this error, try `rustc --explain E0220`.
[01:10:59] + For more information about this error, try `rustc --explain E0220`.
[01:10:59] 8 
[01:10:59] 
[01:10:59] 
[01:10:59] The actual stderr differed from the expected stderr.
[01:10:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59029-1/issue-59029-1.stderr
[01:10:59] To update references, rerun the tests and pass the `--bless` flag
[01:10:59] To only update this specific test, also pass `--test-args issues/issue-59029-1.rs`
[01:10:59] error: 1 errors occurred comparing output.
[01:10:59] status: exit code: 1
[01:10:59] status: exit code: 1
[01:10:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59029-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59029-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59029-1/auxiliary" "-A" "unused"
[01:10:59] ------------------------------------------
[01:10:59] 
[01:10:59] ------------------------------------------
[01:10:59] stderr:
[01:10:59] stderr:
[01:10:59] ------------------------------------------
[01:10:59] {"message":"associated type `Res` not found for `Self`","code":{"code":"E0220","explanation":"\nYou used an associated type which isn't defined in the trait.\nErroneous code example:\n\n