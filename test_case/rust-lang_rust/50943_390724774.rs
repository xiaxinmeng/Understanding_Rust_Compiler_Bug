plain
[00:43:24] ....................................................................................................
[00:43:27] ....................................................................................................
[00:43:31] ....................................................................................................
[00:43:34] ....................................................................................................
[00:43:39] ...............................................F....................................................
[00:43:48] ....................................................................................................
[00:43:53] ......................................................................i.............................
[00:43:58] ...............................................i....................................................
[00:44:02] ...................................................................ii...............................
---
[00:44:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:44:16] ---- [ui] ui/impl-trait/impl-generic-mismatch.rs stdout ----
[00:44:16] diff of stderr:
[00:44:16] 
[00:44:16] 34 LL |     fn hash(&self, hasher: &mut impl Hasher) {}
[00:44:16] 35    |                                 ^^^^^^^^^^^ expected generic parameter, found `impl Trait`
[00:44:16] 36    | 
[00:44:16] -   ::: /home/oliver/Projects/rust/rust3/src/libcore/hash/mod.rs:185:13
[00:44:16] +   ::: /checkout/src/libcore/hash/mod.rs:185:13
[00:44:16] 38    |
[00:44:16] 39 LL |     fn hash<H: Hasher>(&self, state: &mut H);
[00:44:16] 40    |             - declaration in trait here
[00:44:16] 
[00:44:16] The actual stderr differed from the expected stderr.
[00:44:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/impl-generic-mismatch.stderr
[00:44:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/impl-generic-mismatch.stderr
[00:44:16] To update references, rerun the tests and pass the `--bless` flag
[00:44:16] To only update this specific test, also pass `--test-args impl-trait/impl-generic-mismatch.rs`
[00:44:16] error: 1 errors occurred comparing output.
[00:44:16] status: exit code: 101
[00:44:16] status: exit code: 101
[00:44:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/auxiliary" "-A" "unused"
[00:44:16] ------------------------------------------
[00:44:16] 
[00:44:16] ------------------------------------------
[00:44:16] stderr:
[00:44:16] stderr:
[00:44:16] ------------------------------------------
[00:44:16] {"message":"method `foo` has incompatible signature for trait","code":{"code":"E0643","explanation":"\nThis error indicates that there is a mismatch between generic parameters and\nimpl Trait parameters in a trait declaration versus its impl.\n\n