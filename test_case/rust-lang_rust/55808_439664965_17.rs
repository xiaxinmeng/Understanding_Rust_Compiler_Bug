\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-23024.rs","byte_start":653,"byte_end":655,"line_start":19,"line_end":19,"column_start":35,"column_end":37,"is_primary":true,"text":[{"text":"    println!(\"{:?}\",(vfnfer[0] as Fn)(3));","highlight_start":35,"highlight_end":37}],"label":"associated type `Output` must be specified","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/rustc/42741ddb5182fac9cb0afb1f8425ae963b0b9db4/src/libcore/ops/function.rs","byte_start":8519,"byte_end":8531,"line_start":234,"line_end":234,"column_start":5,"column_end":17,"is_primary":false,"text":[{"text":"","highlight_start":5,"highlight_end":17}],"label":"`Output` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0191]: the value of the associated type `Output` (from the trait `std::ops::FnOnce`) must be specified\n  --> /checkout/src/test/ui/issues/issue-23024.rs:19:35\n   |\nLL |     println!(\"{:?}\",(vfnfer[0] as Fn)(3));\n   |                                   ^^ associated type `Output` must be specified\n\n"}
[00:48:35] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:48:35] {"message":"Some errors occurred: E0107, E0191, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0107, E0191, E0658.\n"}
[00:48:35] {"message":"For more information about an error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0107`.\n"}
[00:48:35] ------------------------------------------
[00:48:35] 
[00:48:35] thread '[ui] ui/issues/issue-23024.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:35] 
[00:48:35] 
[00:48:35] ---- [ui] ui/issues/issue-28344.rs stdout ----
[00:48:35] diff of stderr:
[00:48:35] 
[00:48:35] 3    |
[00:48:35] 4 LL |     let x: u8 = BitXor::bitor(0 as u8, 0 as u8);
[00:48:35] 5    |                 ^^^^^^^^^^^^^ associated type `Output` must be specified
[00:48:35] -    | 
[00:48:35] -   ::: $SRC_DIR/libcore/ops/bit.rs:LL:COL
[00:48:35] -    |
[00:48:35] - LL |     type Output;
[00:48:35] -    |     ------------ `Output` defined here
[00:48:35] 11 
[00:48:35] 12 error[E0599]: no function or associated item named `bitor` found for type `dyn std::ops::BitXor<_>` in the current scope
[00:48:35] 
[00:48:35] 22    |
[00:48:35] 22    |
[00:48:35] 23 LL |     let g = BitXor::bitor;
[00:48:35] 24    |             ^^^^^^^^^^^^^ associated type `Output` must be specified
[00:48:35] -    | 
[00:48:35] -   ::: $SRC_DIR/libcore/ops/bit.rs:LL:COL
[00:48:35] -    |
[00:48:35] - LL |     type Output;
[00:48:35] -    |     ------------ `Output` defined here
[00:48:35] 30 
[00:48:35] 31 error[E0599]: no function or associated item named `bitor` found for type `dyn std::ops::BitXor<_>` in the current scope
[00:48:35] 
[00:48:35] 
[00:48:35] The actual stderr differed from the expected stderr.
[00:48:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28344/issue-28344.stderr
[00:48:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28344/issue-28344.stderr
[00:48:35] To update references, rerun the tests and pass the `--bless` flag
[00:48:35] To only update this specific test, also pass `--test-args issues/issue-28344.rs`
[00:48:35] error: 1 errors occurred comparing output.
[00:48:35] status: exit code: 1
[00:48:35] status: exit code: 1
[00:48:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28344.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28344/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28344/auxiliary" "-A" "unused"
[00:48:35] ------------------------------------------
[00:48:35] 
[00:48:35] ------------------------------------------
[00:48:35] stderr:
[00:48:35] stderr:
[00:48:35] ------------------------------------------
[00:48:35] {"message":"the value of the associated type `Output` (from the trait `std::ops::BitXor`) must be specified","code":{"code":"E0191","explanation":"\nTrait objects need to have all associated types specified. Erroneous code\nexample:\n\n