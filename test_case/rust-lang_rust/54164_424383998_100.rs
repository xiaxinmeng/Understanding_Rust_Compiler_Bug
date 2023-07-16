\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-int-wrapping.rs","byte_start":858,"byte_end":881,"line_start":16,"line_end":16,"column_start":28,"column_end":51,"is_primary":true,"text":[{"text":"    let b: &'static i32 = &(5_i32.wrapping_shr(3)); //~ ERROR does not live long enough","highlight_start":28,"highlight_end":51}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-int-wrapping.rs","byte_start":919,"byte_end":920,"line_start":17,"line_end":17,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/const-int-wrapping.rs:16:28\n   |\nLL |     let b: &'static i32 = &(5_i32.wrapping_shr(3)); //~ ERROR does not live long enough\n   |                            ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use\nLL | }\n   | - temporary value is freed at the end of this statement\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
[00:59:39] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:59:39] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[00:59:39] ------------------------------------------
[00:59:39] 
[00:59:39] thread '[ui (nll)] ui/consts/const-int-wrapping.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:59:39] 
[00:59:39] 
[00:59:39] ---- [ui (nll)] ui/consts/min_const_fn/min_const_fn_dyn.rs stdout ----
[00:59:39] diff of stderr:
[00:59:39] 
[00:59:39] 10 LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
[00:59:39] 12 
[00:59:39] 12 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 15    |
[00:59:39] 15    |
[00:59:39] 16 LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
[00:59:39] 22 
[00:59:39] 23 error: aborting due to 3 previous errors
[00:59:39] 24 
[00:59:39] - For more information about this error, try `rustc --explain E0714`.
[00:59:39] - For more information about this error, try `rustc --explain E0714`.
[00:59:39] + For more information about this error, try `rustc --explain E0716`.
[00:59:39] 26 
[00:59:39] 
[00:59:39] 
[00:59:39] The actual stderr differed from the expected stderr.
[00:59:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn.nll/min_const_fn_dyn.nll.stderr
[00:59:39] To update references, rerun the tests and pass the `--bless` flag
[00:59:39] To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_dyn.rs`
[00:59:39] error: 1 errors occurred comparing output.
[00:59:39] status: exit code: 1
[00:59:39] status: exit code: 1
[00:59:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn.nll/auxiliary" "-A" "unused"
[00:59:39] ------------------------------------------
[00:59:39] 
[00:59:39] ------------------------------------------
[00:59:39] stderr:
[00:59:39] stderr:
[00:59:39] ------------------------------------------
[00:59:39] {"message":"trait bounds other than `Sized` on const fn parameters are unstable","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs","byte_start":661,"byte_end":670,"line_start":21,"line_end":21,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    x.0.field;","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: trait bounds other than `Sized` on const fn parameters are unstable\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs:21:5\n   |\nLL |     x.0.field;\n   |     ^^^^^^^^^\n\n"}
[00:59:39] {"message":"trait bounds other than `Sized` on const fn parameters are unstable","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs","byte_start":782,"byte_end":784,"line_start":24,"line_end":24,"column_start":66,"column_end":68,"is_primary":true,"text":[{"text":"const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }","highlight_start":66,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: trait bounds other than `Sized` on const fn parameters are unstable\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs:24:66\n   |\nLL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }\n   |                                                                  ^^\n\n"}
[00:59:39] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n