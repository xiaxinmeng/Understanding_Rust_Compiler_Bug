plain
travis_time:end:14884658:start=1551209049594140408,finish=1551209121431415854,duration=71837275446
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:02:06] .................................................................................................... 1200/5417
[01:02:08] .................................................................................................... 1300/5417
[01:02:11] .................................................................................................... 1400/5417
[01:02:14] .................................................................................................... 1500/5417
[01:02:16] ..............................................................................................F..... 1600/5417
[01:02:23] .................................................................................................... 1800/5417
[01:02:26] .................................................................................................... 1900/5417
[01:02:29] .................................................................................................... 2000/5417
[01:02:33] ..................................................i................................................. 2100/5417
---
[01:04:32] 
[01:04:32] ---- [ui] ui/feature-gates/feature-gate-unwind-attributes.rs stdout ----
[01:04:32] diff of stderr:
[01:04:32] 
[01:04:32] - error[E0658]: #[unwind] is experimental
[01:04:32] + error[E0658]: #[unwind] is experimental (see issue #58760)
[01:04:32] 2   --> $DIR/feature-gate-unwind-attributes.rs:11:5
[01:04:32] 3    |
[01:04:32] 4 LL |     #[unwind(allowed)] //~ ERROR #[unwind] is experimental
[01:04:32] 
[01:04:32] The actual stderr differed from the expected stderr.
[01:04:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unwind-attributes/feature-gate-unwind-attributes.stderr
[01:04:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unwind-attributes/feature-gate-unwind-attributes.stderr
[01:04:32] To update references, rerun the tests and pass the `--bless` flag
[01:04:32] To only update this specific test, also pass `--test-args feature-gates/feature-gate-unwind-attributes.rs`
[01:04:32] error: 1 errors occurred comparing output.
[01:04:32] status: exit code: 1
[01:04:32] status: exit code: 1
[01:04:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unwind-attributes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unwind-attributes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "no-prepopulate-passes" "-Cpasses=name-anon-globals" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unwind-attributes/auxiliary" "-A" "unused"
[01:04:32] ------------------------------------------
[01:04:32] 
[01:04:32] ------------------------------------------
[01:04:32] stderr:
[01:04:32] stderr:
[01:04:32] ------------------------------------------
[01:04:32] {"message":"#[unwind] is experimental (see issue #58760)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n