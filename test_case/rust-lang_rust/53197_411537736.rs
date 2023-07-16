plain
[00:47:48] 
[00:47:48] ---- [ui] ui/target-feature-gate.rs stdout ----
[00:47:48] diff of stderr:
[00:47:48] 
[00:47:48] 1 error[E0658]: the target feature `avx512bw` is currently unstable (see issue #44839)
[00:47:48] +   --> $DIR/target-feature-gate.rs:29:18
[00:47:48] 3    |
[00:47:48] 3    |
[00:47:48] 4 LL | #[target_feature(enable = "avx512bw")]
[00:47:48] 
[00:47:48] 
[00:47:48] The actual stderr differed from the expected stderr.
[00:47:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-gate/target-feature-gate.stderr
[00:47:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-gate/target-feature-gate.stderr
[00:47:48] To update references, rerun the tests and pass the `--bless` flag
[00:47:48] To only update this specific test, also pass `--test-args target-feature-gate.rs`
[00:47:48] error: 1 errors occurred comparing output.
[00:47:48] status: exit code: 1
[00:47:48] status: exit code: 1
[00:47:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature-gate.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-gate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-gate/auxiliary" "-A" "unused"
[00:47:48] ------------------------------------------
[00:47:48] 
[00:47:48] ------------------------------------------
[00:47:48] stderr:
[00:47:48] stderr:
[00:47:48] ------------------------------------------
[00:47:48] {"message":"the target feature `avx512bw` is currently unstable (see issue #44839)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n