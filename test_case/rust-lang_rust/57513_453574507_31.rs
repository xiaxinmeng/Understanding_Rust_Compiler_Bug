\n"},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs","byte_start":5608,"byte_end":5611,"line_start":132,"line_end":132,"column_start":63,"column_end":66,"is_primary":true,"text":[{"text":"const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }","highlight_start":63,"highlight_end":66}],"label":"returns a reference to data owned by the current function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs","byte_start":5609,"byte_end":5611,"line_start":132,"line_end":132,"column_start":64,"column_end":66,"is_primary":false,"text":[{"text":"const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }","highlight_start":64,"highlight_end":66}],"label":"temporary value created here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this error has been downgraded to a warning for backwards compatibility with previous releases","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"this represents potential undefined behavior in your code and this warning will become a hard error in the future","code":null,"level":"warning","spans":[],"children":[],"rendered":null}],"rendered":"warning[E0515]: cannot return reference to temporary value\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:132:63\n   |\nLL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }\n   |                                                               ^--\n   |                                                               ||\n   |                                                               |temporary value created here\n   |                                                               returns a reference to data owned by the current function\n   |\n   = warning: this error has been downgraded to a warning for backwards compatibility with previous releases\n   = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future\n\n"}
[01:31:44] {"message":"trait bounds other than `Sized` on const fn parameters are unstable","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs","byte_start":5734,"byte_end":5763,"line_start":137,"line_end":137,"column_start":41,"column_end":70,"is_primary":true,"text":[{"text":"const fn really_no_traits_i_mean_it() { (&() as &std::fmt::Debug, ()).1 }","highlight_start":41,"highlight_end":70}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: trait bounds other than `Sized` on const fn parameters are unstable\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:137:41\n   |\nLL | const fn really_no_traits_i_mean_it() { (&() as &std::fmt::Debug, ()).1 }\n   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:31:44] {"message":"function pointers in const fn are unstable","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs","byte_start":5832,"byte_end":5834,"line_start":140,"line_end":140,"column_start":21,"column_end":23,"is_primary":true,"text":[{"text":"const fn no_fn_ptrs(_x: fn()) {}","highlight_start":21,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: function pointers in const fn are unstable\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:140:21\n   |\nLL | const fn no_fn_ptrs(_x: fn()) {}\n   |                     ^^\n\n"}
[01:31:44] {"message":"function pointers in const fn are unstable","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs","byte_start":5925,"byte_end":5929,"line_start":142,"line_end":142,"column_start":27,"column_end":31,"is_primary":true,"text":[{"text":"const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }","highlight_start":27,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: function pointers in const fn are unstable\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:142:27\n   |\nLL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }\n   |                           ^^^^\n\n"}
[01:31:44] {"message":"aborting due to 34 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 34 previous errors\n\n"}
[01:31:44] {"message":"Some errors occurred: E0493, E0515.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0493, E0515.\n"}
[01:31:44] 
[01:31:44] ------------------------------------------
[01:31:44] 
[01:31:44] thread '[ui (nll)] ui/consts/min_const_fn/min_const_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:31:44] thread '[ui (nll)] ui/consts/min_const_fn/min_const_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:31:44] 
[01:31:44] ---- [ui (nll)] ui/consts/promote_const_let.rs stdout ----
[01:31:44] diff of stderr:
[01:31:44] 
[01:31:44] 1 error[E0597]: `y` does not live long enough
[01:31:44] +   --> $DIR/promote_const_let.rs:4:9
[01:31:44] 3    |
[01:31:44] 3    |
[01:31:44] 4 LL |     let x: &'static u32 = {
[01:31:44] 5    |            ------------ type annotation requires that `y` is borrowed for `'static`
[01:31:44] 9 LL |     };
[01:31:44] 9 LL |     };
[01:31:44] 10    |     - `y` dropped here while still borrowed
[01:31:44] - error: aborting due to previous error
[01:31:44] - error: aborting due to previous error
[01:31:44] + error[E0716]: temporary value dropped while borrowed
[01:31:44] +    |
[01:31:44] +    |
[01:31:44] + LL |       let x: &'static u32 = &{ //~ ERROR does not live long enough
[01:31:44] +    |  ____________------------____^
[01:31:44] +    | |            type annotation requires that borrow lasts for `'static`
[01:31:44] +    | |            type annotation requires that borrow lasts for `'static`
[01:31:44] + LL | |         let y = 42;
[01:31:44] + LL | |         y
[01:31:44] + LL | |     };
[01:31:44] +    | |_____^ creates a temporary which is freed while still in use
[01:31:44] + LL |   }
[01:31:44] +    |   - temporary value is freed at the end of this statement
[01:31:44] - For more information about this error, try `rustc --explain E0597`.
[01:31:44] + error: aborting due to 2 previous errors
[01:31:44] + 
[01:31:44] + Some errors occurred: E0597, E0716.
[01:31:44] + Some errors occurred: E0597, E0716.
[01:31:44] + For more information about an error, try `rustc --explain E0597`.
[01:31:44] 15 
[01:31:44] 
[01:31:44] 
[01:31:44] The actual stderr differed from the expected stderr.
[01:31:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_const_let.nll/promote_const_let.nll.stderr
[01:31:44] To update references, rerun the tests and pass the `--bless` flag
[01:31:44] To only update this specific test, also pass `--test-args consts/promote_const_let.rs`
[01:31:44] error: 1 errors occurred comparing output.
[01:31:44] status: exit code: 1
[01:31:44] status: exit code: 1
[01:31:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promote_const_let.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_const_let.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_const_let.nll/auxiliary" "-A" "unused"
[01:31:44] ------------------------------------------
[01:31:44] 
[01:31:44] ------------------------------------------
[01:31:44] stderr:
[01:31:44] stderr:
[01:31:44] ------------------------------------------
[01:31:44] {"message":"`y` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n