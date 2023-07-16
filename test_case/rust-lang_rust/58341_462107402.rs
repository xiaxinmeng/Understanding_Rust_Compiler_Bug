plain
travis_time:end:0fa43db4:start=1549775499837352556,finish=1549775502865067410,duration=3027714854
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:05:05] .................................................................................................... 2400/5380
[01:05:10] .................................................................................................... 2500/5380
[01:05:14] .................................................................................................... 2600/5380
[01:05:18] .................................................................................................... 2700/5380
[01:05:23] .............................................................................................F...... 2800/5380
[01:05:32] .................................................................................................... 3000/5380
[01:05:35] .................................................................................................... 3100/5380
[01:05:40] .................................................................................................... 3200/5380
[01:05:44] ................................i................................................................... 3300/5380
---
[01:07:13] ---- [ui] ui/issues/issue-48636.rs stdout ----
[01:07:13] diff of stderr:
[01:07:13] 
[01:07:13] 3    |
[01:07:13] 4 LL |     x: u8
[01:07:13] 5    |          - help: missing comma here: `,`
[01:07:13] - LL |     /// The id of the parent core
[01:07:13] + LL |     /// The ID of the parent core
[01:07:13] 8    |
[01:07:13] 8    |
[01:07:13] 9    = help: doc comments must come before what they document, maybe a comment was intended with `//`?
[01:07:13] 
[01:07:13] The actual stderr differed from the expected stderr.
[01:07:13] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48636/issue-48636.stderr
[01:07:13] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48636/issue-48636.stderr
[01:07:13] To update references, rerun the tests and pass the `--bless` flag
[01:07:13] To only update this specific test, also pass `--test-args issues/issue-48636.rs`
[01:07:13] error: 1 errors occurred comparing output.
[01:07:13] status: exit code: 1
[01:07:13] status: exit code: 1
[01:07:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-48636.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48636/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48636/auxiliary" "-A" "unused"
[01:07:13] ------------------------------------------
[01:07:13] 
[01:07:13] ------------------------------------------
[01:07:13] stderr:
[01:07:13] stderr:
[01:07:13] ------------------------------------------
[01:07:13] {"message":"found a documentation comment that doesn't document anything","code":{"code":"E0585","explanation":"\nA documentation comment that doesn't document anything was found.\n\nErroneous code example:\n\n