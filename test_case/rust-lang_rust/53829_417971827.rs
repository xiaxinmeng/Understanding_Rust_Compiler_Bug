plain
[00:46:45] 
[00:46:45] ---- [ui] ui/impl-trait/impl-generic-mismatch.rs stdout ----
[00:46:45] diff of stderr:
[00:46:45] 
[00:46:45] 1 error[E0643]: method `foo` has incompatible signature for trait
[00:46:45] +   --> $DIR/impl-generic-mismatch.rs:20:12
[00:46:45] 3    |
[00:46:45] 3    |
[00:46:45] 4 LL |     fn foo(&self, _: &impl Debug);
[00:46:45] 5    |                       ---------- declaration in trait here
[00:46:45] 12    |          --           ^^^^^^^^^^
[00:46:45] 13 
[00:46:45] 13 
[00:46:45] 14 error[E0643]: method `bar` has incompatible signature for trait
[00:46:45] +   --> $DIR/impl-generic-mismatch.rs:29:23
[00:46:45] 16    |
[00:46:45] 16    |
[00:46:45] 17 LL |     fn bar<U: Debug>(&self, _: &U);
[00:46:45] 18    |            - declaration in trait here
[00:46:45] 25    |           ^^^^^^^^^^            ^
[00:46:45] 26 
[00:46:45] 27 error[E0643]: method `hash` has incompatible signature for trait
[00:46:45] -   --> $DIR/impl-generic-mismatch.rs:38:33
[00:46:45] -   --> $DIR/impl-generic-mismatch.rs:38:33
[00:46:45] +   --> $DIR/impl-generic-mismatch.rs:40:33
[00:46:45] 29    |
[00:46:45] 30 LL |     fn hash(&self, hasher: &mut impl Hasher) {}
[00:46:45] 31    |                                 ^^^^^^^^^^^ expected generic parameter, found `impl Trait`
[00:46:45] 
[00:46:45] The actual stderr differed from the expected stderr.
[00:46:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/impl-generic-mismatch.stderr
[00:46:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/impl-generic-mismatch.stderr
[00:46:45] To update references, rerun the tests and pass the `--bless` flag
[00:46:45] To only update this specific test, also pass `--test-args impl-trait/impl-generic-mismatch.rs`
[00:46:45] error: 1 errors occurred comparing output.
[00:46:45] status: exit code: 1
[00:46:45] status: exit code: 1
[00:46:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/auxiliary" "-A" "unused"
[00:46:45] ------------------------------------------
[00:46:45] 
[00:46:45] ------------------------------------------
[00:46:45] stderr:
[00:46:45] stderr:
[00:46:45] ------------------------------------------
[00:46:45] {"message":"method `foo` has incompatible signature for trait","code":{"code":"E0643","explanation":"\nThis error indicates that there is a mismatch between generic parameters and\nimpl Trait parameters in a trait declaration versus its impl.\n\n