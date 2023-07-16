plain
travis_time:end:0191e519:start=1549365826433048810,finish=1549365828870880530,duration=2437831720
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:00:04] .................................................................................................... 2500/5365
[01:00:08] .................................................................................................... 2600/5365
[01:00:12] .................................................................................................... 2700/5365
[01:00:16] .................................................................................................... 2800/5365
[01:00:20] .............................................................F...................................... 2900/5365
[01:00:23] ..........F......................................................................................... 3000/5365
[01:00:30] .................................................................................................... 3200/5365
[01:00:34] ......................i............................................................................. 3300/5365
[01:00:37] ........................................................................................ii...i..ii.. 3400/5365
[01:00:41] .................................................................................................... 3500/5365
---
[01:01:49] diff of stderr:
[01:01:49] 
[01:01:49] 90 LL |       m,
[01:01:49] 91    |  ______-
[01:01:49] 92 LL | |     a}; //~ ERROR `a` is defined multiple times
[01:01:49] +    | |     ^
[01:01:49] 94    | |     |
[01:01:49] 94    | |     |
[01:01:49] 95    | |_____`a` reimported here
[01:01:49] 96    |       help: remove unnecessary import
[01:01:49] 
[01:01:49] The actual stderr differed from the expected stderr.
[01:01:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52891/issue-52891.stderr
[01:01:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52891/issue-52891.stderr
[01:01:49] To update references, rerun the tests and pass the `--bless` flag
[01:01:49] To only update this specific test, also pass `--test-args issues/issue-52891.rs`
[01:01:49] error: 1 errors occurred comparing output.
[01:01:49] status: exit code: 1
[01:01:49] status: exit code: 1
[01:01:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52891.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52891/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52891/auxiliary" "-A" "unused"
[01:01:49] ------------------------------------------
[01:01:49] 
[01:01:49] ------------------------------------------
[01:01:49] stderr:
[01:01:49] stderr:
[01:01:49] ------------------------------------------
[01:01:49] {"message":"the name `a` is defined multiple times","code":{"code":"E0252","explanation":"\nTwo items of the same name cannot be imported without rebinding one of the\nitems under a new local name.\n\nErroneous code example:\n\n