
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/lint-group.rs:7:9
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | #![deny(rustdoc)]
[01:42:22] -    |         ^^^^^^^
[01:42:22] -    = note: #[deny(private_doc_tests)] implied by #[deny(rustdoc)]
[01:42:22] - error: `[error]` cannot be resolved, ignoring it...
[01:42:22] -   --> $DIR/lint-group.rs:9:29
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// what up, let's make an [error]
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/lint-group.rs:7:9
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | #![deny(rustdoc)]
[01:42:22] -    |         ^^^^^^^
[01:42:22] -    = note: #[deny(intra_doc_link_resolution_failure)] implied by #[deny(rustdoc)]
[01:42:22] - 
[01:42:22] - error: Missing code example in this documentation
[01:42:22] -   --> $DIR/lint-group.rs:16:1
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// wait, this doesn't have a doctest?
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/lint-group.rs:7:9
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | #![deny(rustdoc)]
[01:42:22] -    |         ^^^^^^^
[01:42:22] -    = note: #[deny(missing_doc_code_examples)] implied by #[deny(rustdoc)]
[01:42:22] - error: aborting due to 3 previous errors
[01:42:22] - error: aborting due to 3 previous errors
[01:42:22] + error: Unrecognized option: 'out-dir'
[01:42:22] 47 
[01:42:22] 
[01:42:22] 
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-group/lint-group.stderr
[01:42:22] To update references, rerun the tests and pass the `--bless` flag
[01:42:22] To only update this specific test, also pass `--test-args lint-group.rs`
[01:42:22] error: 1 errors occurred comparing output.
[01:42:22] status: exit code: 1
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/lint-group.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-group" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-group/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/issue-58473-2.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-58473-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473-2" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473-2/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/unused.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/private-item-doc-test.rs stdout ----
[01:42:22] diff of stderr:
[01:42:22] 
[01:42:22] - error: Documentation test in private item
[01:42:22] -   --> $DIR/private-item-doc-test.rs:4:5
[01:42:22] -    |
[01:42:22] - LL | /     /// private doc test
[01:42:22] - LL | |     ///
[01:42:22] - LL | |     /// 