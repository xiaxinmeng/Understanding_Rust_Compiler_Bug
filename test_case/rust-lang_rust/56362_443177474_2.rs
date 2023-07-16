\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-match-check.rs","byte_start":1012,"byte_end":1013,"line_start":35,"line_end":35,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"    A = { let 0 = 0; 0 },","highlight_start":15,"highlight_end":16}],"label":"pattern `-2147483648i32..=-1i32` not covered","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0005]: refutable pattern in local binding: `-2147483648i32..=-1i32` not covered\n  --> /checkout/src/test/ui/consts/const-match-check.rs:35:15\n   |\nLL |     A = { let 0 = 0; 0 },\n   |               ^ pattern `-2147483648i32..=-1i32` not covered\n\n"}
[00:51:08] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:08] {"message":"For more information about this error, try `rustc --explain E0005`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0005`.\n"}
[00:51:08] ------------------------------------------
[00:51:08] 
[00:51:08] thread '[ui] ui/consts/const-match-check.rs#eval1' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:51:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:08] 
[00:51:08] ---- [ui] ui/consts/const-match-check.rs#eval2 stdout ----
[00:51:08] diff of stderr:
[00:51:08] 
[00:51:08] - error[E0005]: refutable pattern in local binding: `_` not covered
[00:51:08] + error[E0005]: refutable pattern in local binding: `-2147483648i32..=-1i32` not covered
[00:51:08] 3    |
[00:51:08] 3    |
[00:51:08] 4 LL |     let x: [i32; { let 0 = 0; 0 }] = [];
[00:51:08] 
[00:51:08] -    |                        ^ pattern `_` not covered
[00:51:08] +    |                        ^ pattern `-2147483648i32..=-1i32` not covered
[00:51:08] 7 error: aborting due to previous error
[00:51:08] 8 
[00:51:08] 
[00:51:08] 
[00:51:08] 
[00:51:08] The actual stderr differed from the expected stderr.
[00:51:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-match-check.eval2/const-match-check.eval2.stderr
[00:51:08] To update references, rerun the tests and pass the `--bless` flag
[00:51:08] To only update this specific test, also pass `--test-args consts/const-match-check.rs`
[00:51:08] 
[00:51:08] error in revision `eval2`: 1 errors occurred comparing output.
[00:51:08] status: exit code: 1
[00:51:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-match-check.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "eval2" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-match-check.eval2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-match-check.eval2/auxiliary" "-A" "unused"
[00:51:08] ------------------------------------------
[00:51:08] 
[00:51:08] ------------------------------------------
[00:51:08] stderr:
[00:51:08] stderr:
[00:51:08] ------------------------------------------
[00:51:08] {"message":"refutable pattern in local binding: `-2147483648i32..=-1i32` not covered","code":{"code":"E0005","explanation":"\nPatterns used to bind names must be irrefutable, that is, they must guarantee\nthat a name will be extracted in all cases. Erroneous code example:\n\n