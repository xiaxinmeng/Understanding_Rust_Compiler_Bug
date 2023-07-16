plain
[00:48:35] ---- [ui] ui/issues/issue-21950.rs stdout ----
[00:48:35] diff of stderr:
[00:48:35] 
[00:48:35] 11    |
[00:48:35] 12 LL |             &Add;
[00:48:35] 13    |              ^^^ associated type `Output` must be specified
[00:48:35] -    | 
[00:48:35] -   ::: $SRC_DIR/libcore/ops/arith.rs:LL:COL
[00:48:35] -    |
[00:48:35] - LL |     type Output;
[00:48:35] -    |     ------------ `Output` defined here
[00:48:35] 20 error: aborting due to 2 previous errors
[00:48:35] 21 
[00:48:35] 
[00:48:35] 
[00:48:35] 
[00:48:35] The actual stderr differed from the expected stderr.
[00:48:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21950/issue-21950.stderr
[00:48:35] To update references, rerun the tests and pass the `--bless` flag
[00:48:35] To only update this specific test, also pass `--test-args issues/issue-21950.rs`
[00:48:35] error: 1 errors occurred comparing output.
[00:48:35] status: exit code: 1
[00:48:35] status: exit code: 1
[00:48:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21950.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21950/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21950/auxiliary" "-A" "unused"
[00:48:35] ------------------------------------------
[00:48:35] 
[00:48:35] ------------------------------------------
[00:48:35] stderr:
[00:48:35] stderr:
[00:48:35] ------------------------------------------
[00:48:35] {"message":"the type parameter `RHS` must be explicitly specified","code":{"code":"E0393","explanation":"\nA type parameter which references `Self` in its default value was not specified.\nExample of erroneous code:\n\n