plain
travis_time:end:054f6762:start=1546969874797764666,finish=1546969946782999800,duration=71985235134
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:01:52] .................................................................................................... 3100/5299
[01:01:56] ..................................................................................i................. 3200/5299
[01:01:59] .................................................................................................... 3300/5299
[01:02:02] .............................................ii...i..ii............................................. 3400/5299
[01:02:07] ...........................................................................................F........ 3500/5299
[01:02:13] .....................................ii............................................................. 3700/5299
[01:02:15] .......................................................i............................................ 3800/5299
[01:02:17] .................................................................................................... 3900/5299
[01:02:19] ...........i........................................................................................ 4000/5299
---
[01:03:12] 
[01:03:12] ---- [ui] ui/nll/issue-53773.rs stdout ----
[01:03:12] diff of stderr:
[01:03:12] 
[01:03:12] 1 error[E0713]: borrow may still be in use when destructor runs
[01:03:12] +   --> $DIR/issue-53773.rs:43:22
[01:03:12] 3    |
[01:03:12] 3    |
[01:03:12] 4 LL |         members.push(child.raw);
[01:03:12] 
[01:03:12] 
[01:03:12] The actual stderr differed from the expected stderr.
[01:03:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53773/issue-53773.stderr
[01:03:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53773/issue-53773.stderr
[01:03:12] To update references, rerun the tests and pass the `--bless` flag
[01:03:12] To only update this specific test, also pass `--test-args nll/issue-53773.rs`
[01:03:12] error: 1 errors occurred comparing output.
[01:03:12] status: exit code: 1
[01:03:12] status: exit code: 1
[01:03:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-53773.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53773/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53773/auxiliary" "-A" "unused"
[01:03:12] ------------------------------------------
[01:03:12] 
[01:03:12] ------------------------------------------
[01:03:12] stderr:
[01:03:12] stderr:
[01:03:12] ------------------------------------------
[01:03:12] {"message":"borrow may still be in use when destructor runs","code":{"code":"E0713","explanation":"\nThis error occurs when an attempt is made to borrow state past the end of the\nlifetime of a type that implements the `Drop` trait.\n\nExample of erroneous code:\n\n