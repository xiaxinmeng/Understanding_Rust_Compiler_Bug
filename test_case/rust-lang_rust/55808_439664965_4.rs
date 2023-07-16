\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-21950.rs","byte_start":558,"byte_end":561,"line_start":17,"line_end":17,"column_start":14,"column_end":17,"is_primary":true,"text":[{"text":"            &Add;","highlight_start":14,"highlight_end":17}],"label":"associated type `Output` must be specified","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/rustc/42741ddb5182fac9cb0afb1f8425ae963b0b9db4/src/libcore/ops/arith.rs","byte_start":2607,"byte_end":2619,"line_start":94,"line_end":94,"column_start":5,"column_end":17,"is_primary":false,"text":[{"text":"","highlight_start":5,"highlight_end":17}],"label":"`Output` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0191]: the value of the associated type `Output` (from the trait `std::ops::Add`) must be specified\n  --> /checkout/src/test/ui/issues/issue-21950.rs:17:14\n   |\nLL |             &Add;\n   |              ^^^ associated type `Output` must be specified\n\n"}
[00:48:35] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:35] {"message":"Some errors occurred: E0191, E0393.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0191, E0393.\n"}
[00:48:35] {"message":"For more information about an error, try `rustc --explain E0191`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0191`.\n"}
[00:48:35] ------------------------------------------
[00:48:35] 
[00:48:35] thread '[ui] ui/issues/issue-21950.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:35] 
[00:48:35] ---- [ui] ui/issues/issue-22560.rs stdout ----
[00:48:35] diff of stderr:
[00:48:35] 
[00:48:35] 29 LL | |             //~| ERROR E0191
[00:48:35] 30 LL | |             Sub;
[00:48:35] 31    | |_______________^ associated type `Output` must be specified
[00:48:35] -    | 
[00:48:35] -   ::: $SRC_DIR/libcore/ops/arith.rs:LL:COL
[00:48:35] -    |
[00:48:35] - LL |       type Output;
[00:48:35] -    |       ------------ `Output` defined here
[00:48:35] 38 error: aborting due to 4 previous errors
[00:48:35] 39 
[00:48:35] 
[00:48:35] 
[00:48:35] 
[00:48:35] The actual stderr differed from the expected stderr.
[00:48:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/issue-22560.stderr
[00:48:35] To update references, rerun the tests and pass the `--bless` flag
[00:48:35] To only update this specific test, also pass `--test-args issues/issue-22560.rs`
[00:48:35] error: 1 errors occurred comparing output.
[00:48:35] status: exit code: 1
[00:48:35] status: exit code: 1
[00:48:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22560.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/auxiliary" "-A" "unused"
[00:48:35] ------------------------------------------
[00:48:35] 
[00:48:35] ------------------------------------------
[00:48:35] stderr:
[00:48:35] stderr:
[00:48:35] ------------------------------------------
[00:48:35] {"message":"the type parameter `RHS` must be explicitly specified","code":{"code":"E0393","explanation":"\nA type parameter which references `Self` in its default value was not specified.\nExample of erroneous code:\n\n