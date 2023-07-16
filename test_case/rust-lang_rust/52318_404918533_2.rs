\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/closure_promotion.rs","byte_start":526,"byte_end":545,"line_start":14,"line_end":14,"column_start":26,"column_end":45,"is_primary":true,"text":[{"text":"    let x: &'static _ = &|| { let z = 3; z }; //~ ERROR does not live long enough","highlight_start":26,"highlight_end":45}],"label":"temporary value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/const-eval/closure_promotion.rs","byte_start":583,"byte_end":584,"line_start":15,"line_end":15,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value only lives until here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: borrowed value does not live long enough\n  --> /checkout/src/test/ui/const-eval/closure_promotion.rs:14:26\n   |\nLL |     let x: &'static _ = &|| { let z = 3;    = note: borrowed value must be valid for the static lifetime...
[00:45:13] + error: aborting due to previous error
[00:45:13] 18 
[00:45:13] - error[E0597]: borrowed value does not live long enough
[00:45:13] -   --> $DIR/dont_promote_unstable_const_fn.rs:32:28
[00:45:13] -    |
[00:45:13] - LL |     let _: &'static u32 = &meh(); //~ ERROR does not live long enough
[00:45:13] -    |                            ^^^^^ temporary value does not live long enough
[00:45:13] - LL |     let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();
[00:45:13] - LL | }
[00:45:13] -    | - temporary value only lives until here
[00:45:13] -    |
[00:45:13] -    = note: borrowed value must be valid for the static lifetime...
[00:45:13] - error: aborting due to 3 previous errors
[00:45:13] - 
[00:45:13] - For more information about this error, try `rustc --explain E0597`.
[00:45:13] 33 
[00:45:13] 33 
[00:45:13] 
[00:45:13] 
[00:45:13] The actual stderr differed from the expected stderr.
[00:45:13] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/dont_promote_unstable_const_fn/dont_promote_unstable_const_fn.stderr
[00:45:13] To update references, rerun the tests and pass the `--bless` flag
[00:45:13] To only update this specific test, also pass `--test-args const-eval/dont_promote_unstable_const_fn.rs`
[00:45:13] error: 1 errors occurred comparing output.
[00:45:13] status: exit code: 101
[00:45:13] status: exit code: 101
[00:45:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/dont_promote_unstable_const_fn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/dont_promote_unstable_const_fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/dont_promote_unstable_const_fn/auxiliary" "-A" "unused"
[00:45:13] ------------------------------------------
[00:45:13] 
[00:45:13] ------------------------------------------
[00:45:13] stderr:
[00:45:13] stderr:
[00:45:13] ------------------------------------------
[00:45:13] {"message":"`foo` is not yet stable as a const fn","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/dont_promote_unstable_const_fn.rs","byte_start":867,"byte_end":872,"line_start":25,"line_end":25,"column_start":25,"column_end":30,"is_primary":true,"text":[{"text":"const fn bar() -> u32 { foo() } //~ ERROR `foo` is not yet stable as a const fn","highlight_start":25,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"in Nightly builds, add `#![feature(foo)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `foo` is not yet stable as a const fn\n  --> /checkout/src/test/ui/const-eval/dont_promote_unstable_const_fn.rs:25:25\n   |\nLL | const fn bar() -> u32 { foo() } //~ ERROR `foo` is not yet stable as a const fn\n   |                         ^^^^^\n   |\n   = help: in Nightly builds, add `#![feature(foo)]` to the crate attributes to enable\n\n"}
[00:45:13] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:13] ------------------------------------------
[00:45:13] 
[00:45:13] thread '[ui] ui/const-eval/dont_promote_unstable_const_fn.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:45:13] 
[00:45:13] 
[00:45:13] ---- [ui] ui/span/borrowck-let-suggestion-suffixes.rs stdout ----
[00:45:13] diff of stderr:
[00:45:13] 
[00:45:13] 9    |
[00:45:13] 10    = note: values in a scope are dropped in the opposite order they are created
[00:45:13] 11 
[00:45:13] - error[E0597]: borrowed value does not live long enough
[00:45:13] -    |
[00:45:13] -    |
[00:45:13] - LL |     v3.push(&id('x'));           // statement 6
[00:45:13] -    |              ^^^^^^^ - temporary value dropped here while still borrowed
[00:45:13] -    |              |
[00:45:13] -    |              temporary value does not live long enough
[00:45:13] - ...
[00:45:13] - LL | }
[00:45:13] -    | - temporary value needs to live until here
[00:45:13] -    |
[00:45:13] -    = note: consider using a `let` binding to increase its lifetime
[00:45:13] - 
[00:45:13] - error[E0597]: borrowed value does not live long enough
[00:45:13] -    |
[00:45:13] -    |
[00:45:13] - LL |         v4.push(&id('y'));
[00:45:13] -    |                  ^^^^^^^ - temporary value dropped here while still borrowed
[00:45:13] -    |                  |
[00:45:13] -    |                  temporary value does not live long enough
[00:45:13] - ...
[00:45:13] - LL |     }                       // (statement 7)
[00:45:13] -    |     - temporary value needs to live until here
[00:45:13] -    |
[00:45:13] -    = note: consider using a `let` binding to increase its lifetime
[00:45:13] - 
[00:45:13] - error[E0597]: borrowed value does not live long enough
[00:45:13] -    |
[00:45:13] -    |
[00:45:13] - LL |     v5.push(&id('z'));
[00:45:13] -    |              ^^^^^^^ - temporary value dropped here while still borrowed
[00:45:13] -    |              |
[00:45:13] -    |              temporary value does not live long enough
[00:45:13] - ...
[00:45:13] - LL | }
[00:45:13] -    | - temporary value needs to live until here
[00:45:13] -    |
[00:45:13] -    = note: consider using a `let` binding to increase its lifetime
[00:45:13] - error: aborting due to 4 previous errors
[00:45:13] + error: aborting due to previous error
[00:45:13] 52 
[00:45:13] 53 For more information about this error, try `rustc --explain E0597`.
[00:45:13] 53 For more information about this error, try `rustc --explain E0597`.
[00:45:13] 54 
[00:45:13] 
[00:45:13] 
[00:45:13] The actual stderr differed from the expected stderr.
[00:45:13] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-let-suggestion-suffixes/borrowck-let-suggestion-suffixes.stderr
[00:45:13] To update references, rerun the tests and pass the `--bless` flag
[00:45:13] To only update this specific test, also pass `--test-args span/borrowck-let-suggestion-suffixes.rs`
[00:45:13] error: 1 errors occurred comparing output.
[00:45:13] status: exit code: 101
[00:45:13] status: exit code: 101
[00:45:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/borrowck-let-suggestion-suffixes.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-let-suggestion-suffixes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-let-suggestion-suffixes/auxiliary" "-A" "unused"
[00:45:13] ------------------------------------------
[00:45:13] 
[00:45:13] ------------------------------------------
[00:45:13] stderr:
[00:45:13] stderr:
[00:45:13] ------------------------------------------
[00:45:13] {"message":"`young[..]` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n