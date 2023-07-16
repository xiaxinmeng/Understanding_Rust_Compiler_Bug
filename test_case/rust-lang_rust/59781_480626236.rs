plain
travis_time:end:04e56820:start=1554664619584972731,finish=1554664698919645468,duration=79334672737
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:06:20] .................................................................................................... 2000/5527
[01:06:23] ...........................................................................................i........ 2100/5527
[01:06:27] .................................................................................................... 2200/5527
[01:06:31] .................................................................................................... 2300/5527
[01:06:35] ..............................FF.................................................................... 2400/5527
[01:06:43] .................................................................................................... 2600/5527
[01:06:43] .................................................................................................... 2600/5527
[01:06:47] .................F.................................................................................. 2700/5527
[01:06:56] .................................................................................................... 2900/5527
[01:06:56] .................................................................................................... 2900/5527
[01:07:00] ...................F................................................................................ 3000/5527
[01:07:07] .................................................................................................... 3200/5527
[01:07:12] .................................................................................................... 3300/5527
[01:07:15] ...........................i........................................................................ 3400/5527
[01:07:19] .................................................................................................... 3500/5527
---
[01:08:38] diff of stderr:
[01:08:38] 
[01:08:38] 5    |         ^^^^^^^^^^^^^
[01:08:38] 6    |
[01:08:38] 7    = note: ...which again requires processing `X::A::{{constant}}#0`, completing the cycle
[01:08:38] - note: cycle used when const-evaluating `X::A::{{constant}}#0`
[01:08:38] + note: cycle used when processing `X::A::{{constant}}#0`
[01:08:38] 10    |
[01:08:38] 10    |
[01:08:38] 11 LL |     A = X::A as isize,
[01:08:38] 
[01:08:38] The actual stderr differed from the expected stderr.
[01:08:38] The actual stderr differed from the expected stderr.
[01:08:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/issue-23302-1.stderr
[01:08:38] To update references, rerun the tests and pass the `--bless` flag
[01:08:38] To only update this specific test, also pass `--test-args issues/issue-23302-1.rs`
[01:08:38] error: 1 errors occurred comparing output.
[01:08:38] status: exit code: 1
[01:08:38] status: exit code: 1
[01:08:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/auxiliary" "-A" "unused"
[01:08:38] ------------------------------------------
[01:08:38] 
[01:08:38] ------------------------------------------
[01:08:38] stderr:
[01:08:38] stderr:
[01:08:38] ------------------------------------------
[01:08:38] {"message":"cycle detected when processing `X::A::{{constant}}#0`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n