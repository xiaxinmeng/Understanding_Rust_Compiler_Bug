plain
travis_time:end:0470b2cc:start=1549820779882253809,finish=1549820852285608571,duration=72403354762
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:26] .................................................................................................... 2400/5380
[00:59:30] .................................................................................................... 2500/5380
[00:59:34] .................................................................................................... 2600/5380
[00:59:37] .................................................................................................... 2700/5380
[00:59:42] .............................................................................................F...... 2800/5380
[00:59:50] .................................................................................................... 3000/5380
[00:59:53] .................................................................................................... 3100/5380
[00:59:57] .................................................................................................... 3200/5380
[01:00:01] ................................i................................................................... 3300/5380
---
[01:01:21] diff of fixed:
[01:01:21] 
[01:01:21] 8     y: u8,
[01:01:21] 9 }
[01:01:21] 10 //~^^^ ERROR found a documentation comment that doesn't document anything
[01:01:21] 11 fn main() {}
[01:01:21] 12 
[01:01:21] 
[01:01:21] 
[01:01:21] 
[01:01:21] The actual fixed differed from the expected fixed.
[01:01:21] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48636/issue-48636.fixed
[01:01:21] To update references, rerun the tests and pass the `--bless` flag
[01:01:21] To only update this specific test, also pass `--test-args issues/issue-48636.rs`
[01:01:21] error: 1 errors occurred comparing output.
[01:01:21] status: exit code: 1
[01:01:21] status: exit code: 1
[01:01:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-48636.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48636/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48636/auxiliary" "-A" "unused"
[01:01:21] ------------------------------------------
[01:01:21] 
[01:01:21] ------------------------------------------
[01:01:21] stderr:
[01:01:21] stderr:
[01:01:21] ------------------------------------------
[01:01:21] {"message":"found a documentation comment that doesn't document anything","code":{"code":"E0585","explanation":"\nA documentation comment that doesn't document anything was found.\n\nErroneous code example:\n\n