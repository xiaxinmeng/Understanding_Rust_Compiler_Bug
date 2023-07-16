plain
travis_time:end:028328dc:start=1548857934856210822,finish=1548857936932583265,duration=2076372443
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:07:33] 
[01:07:33] ---- [ui] ui/target-feature-gate.rs stdout ----
[01:07:33] diff of stderr:
[01:07:33] 
[01:07:33] 1 error[E0658]: the target feature `avx512bw` is currently unstable (see issue #44839)
[01:07:33] +   --> $DIR/target-feature-gate.rs:28:18
[01:07:33] 3    |
[01:07:33] 3    |
[01:07:33] 4 LL | #[target_feature(enable = "avx512bw")]
[01:07:33] 
[01:07:33] 
[01:07:33] The actual stderr differed from the expected stderr.
[01:07:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-gate/target-feature-gate.stderr
[01:07:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-gate/target-feature-gate.stderr
[01:07:33] To update references, rerun the tests and pass the `--bless` flag
[01:07:33] To only update this specific test, also pass `--test-args target-feature-gate.rs`
[01:07:33] error: 1 errors occurred comparing output.
[01:07:33] status: exit code: 1
[01:07:33] status: exit code: 1
[01:07:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-gate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-gate/auxiliary" "-A" "unused"
[01:07:33] ------------------------------------------
[01:07:33] 
[01:07:33] ------------------------------------------
[01:07:33] stderr:
[01:07:33] stderr:
[01:07:33] ------------------------------------------
[01:07:33] {"message":"the target feature `avx512bw` is currently unstable (see issue #44839)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n