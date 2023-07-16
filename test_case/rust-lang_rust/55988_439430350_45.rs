\n\nIf we control the definition of a type, we can implement `Clone` on it ourselves\nwith `#[derive(Clone)]`.\n\nSome types have no owne--
[00:51:24] {"message":"the feature `conservative_impl_trait` has been stable since 1.26.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/issue-55608-captures-empty-region.rs","byte_start":107,"byte_end":130,"line_start":4,"line_end":4,"column_start":12,"column_end":35,"is_primary":true,"text":[{"text":"#![feature(conservative_impl_trait)]","highlight_start":12,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(stable_features)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: the feature `conservative_impl_trait` has been stable since 1.26.0 and no longer requires an attribute to enable\n  --> /checkout/src/test/ui/impl-trait/issue-55608-captures-empty-region.rs:4:12\n   |\nLL | #![feature(conservative_impl_trait)]\n   |            ^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(stable_features)] on by default\n\n"}
[00:51:24] ------------------------------------------
[00:51:24] 
[00:51:24] thread '[ui] ui/impl-trait/issue-55608-captures-empty-region.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:51:24] 
[00:51:24] 
[00:51:24] ---- [ui] ui/issues/issue-40510-1.rs stdout ----
[00:51:24] 
[00:51:24] error: test compilation failed although it shouldn't!
[00:51:24] status: exit code: 1
[00:51:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40510-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-1/auxiliary" "-A" "unused"
[00:51:24] ------------------------------------------
[00:51:24] 
[00:51:24] ------------------------------------------
[00:51:24] stderr:
[00:51:24] stderr:
[00:51:24] ------------------------------------------
[00:51:24] {"message":"cannot infer an appropriate lifetime for borrow expression due to conflicting requirements","code":{"code":"E0495","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-40510-1.rs","byte_start":568,"byte_end":574,"line_start":18,"line_end":18,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        &mut x","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"first, the lifetime cannot outlive the lifetime  as defined on the body at 17:5...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-40510-1.rs","byte_start":555,"byte_end":557,"line_start":17,"line_end":17,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    || {","highlight_start":5,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":n":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0495]: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements\n  --> /checkout/src/test/ui/issues/issue-40510-1.rs:18:9\n   |\nLL |         &mut x\n   |         ^^^^^^\n   |\nnote: first, the lifetime cannot outlive the lifetime  as defined on the body at 17:5...\n  --> /checkout/src/test/ui/issues/issue-40510-1.rs:17:5\n   |\nLL |     || {\n   |     ^^\nnote: ...so that closure can access `x`\n  --> /checkout/src/test/ui/issues/issue-40510-1.rs:18:9\n   |\nLL |         &mut x\n   |         ^^^^^^\nnote: but, the lifetime must be valid for the expression at 17:5...\n  --> /checkout/src/test/ui/issues/issue-40510-1.rs:17:5\n   |\nLL | /     || {\nLL | |         &mut x\nLL | |     };\n   | |_____^\nnote: ...so type `[closure@/checkout/src/test/ui/issues/issue-40510-1.rs:17:5: 19:6 x:&mut std::boxed::Box<()>]` of expression is valid during the expression\n  --> /checkout/src/test/ui/issues/issue-40510-1.rs:17:5\n   |\nLL | /     || {\nLL | |         &mut x\nLL | |     };\n   | |_____^\n\n"}
[00:51:24] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:24] {"message":"For more information about this error, try `rustc --explain E0495`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0495`.\n"}
[00:51:24] ------------------------------------------
[00:51:24] 
[00:51:24] thread '[ui] ui/issues/issue-40510-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:51:24] 
[00:51:24] 
[00:51:24] ---- [ui] ui/issues/issue-40510-3.rs stdout ----
[00:51:24] 
[00:51:24] error: test compilation failed although it shouldn't!
[00:51:24] status: exit code: 1
[00:51:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40510-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-3/auxiliary" "-A" "unused"
[00:51:24] ------------------------------------------
[00:51:24] 
[00:51:24] ------------------------------------------
[00:51:24] stderr:
[00:51:24] stderr:
[00:51:24] ------------------------------------------
[00:51:24] {"message":"lifetime of `x` is too short to guarantee its contents can be safely reborrowed","code":{"code":"E0598","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-40510-3.rs","byte_start":566,"byte_end":568,"line_start":18,"line_end":18,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        || {","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`x` would have to be valid for the expression at 17:5...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-40510-3.rs","byte_start":553,"byte_end":609,"line_start":17,"line_end":21,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    || {","highlight_start":5,"highlight_end":9},{"text":"        || {","highlight_start":1,"highlight_end":13},{"text":"            x.push(())","highlight_start":1,"highlight_end":23},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...but `x` is only valid for the lifetime  as defined on the body at 17:5","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-40510-3.rs","byte_start":553,"byte_end":555,"line_start":17,"line_end":17,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    || {","highlight_start":5,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0598]: lifetime of `x` is too short to guarantee its contents can be safely reborrowed\n  --> /checkout/src/test/ui/issues/issue-40510-3.rs:18:9\n   |\nLL |         || {\n   |         ^^\n   |\nnote: `x` would have to be valid for the expression at 17:5...\n  --> /checkout/src/test/ui/issues/issue-40510-3.rs:17:5\n   |\nLL | /     || {\nLL | |         || {\nLL | |             x.push(())\nLL | |         }\nLL | |     };\n   | |_____^\nnote: ...but `x` is only valid for the lifetime  as defined on the body at 17:5\n  --> /checkout/src/test/ui/issues/issue-40510-3.rs:17:5\n   |\nLL |     || {\n   |     ^^\n\n"}
[00:51:24] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:24] {"message":"For more information about this error, try `rustc --explain E0598`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0598`.\n"}
[00:51:24] ------------------------------------------
[00:51:24] 
[00:51:24] thread '[ui] ui/issues/issue-40510-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:51:24] 
[00:51:24] 
[00:51:24] ---- [ui] ui/issues/issue-49824.rs stdout ----
[00:51:24] diff of stderr:
[00:51:24] 
[00:51:24] - error: compilation successful
[00:51:24] -   --> $DIR/issue-49824.rs:18:1
[00:51:24] + error[E0598]: lifetime of `x` is too short to guarantee its contents can be safely reborrowed
[00:51:24] +   --> $DIR/issue-49824.rs:22:9
[00:51:24] 3    |
[00:51:24] - LL | / fn main() {
[00:51:24] - LL | |     //~^ compilation successful
[00:51:24] - LL | |     let mut x = 0;
[00:51:24] - LL | |     || {
[00:51:24] - ...  |
[00:51:24] + LL |         || {
[00:51:24] +    |
[00:51:24] +    |
[00:51:24] + note: `x` would have to be valid for the expression at 21:5...
[00:51:24] +   --> $DIR/issue-49824.rs:21:5
[00:51:24] +    |
[00:51:24] + LL | /     || {
[00:51:24] + LL | |         || {
[00:51:24] + LL | |             let _y = &mut x;
[00:51:24] + LL | |         }
[00:51:24] 9 LL | |     };
[00:51:24] - LL | | }-----------------------------------------
[00:51:24] {"message":"lifetime of `x` is too short to guarantee its contents can be safely reborrowed","code":{"code":"E0598","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-49824.rs","byte_start":802,"byte_end":804,"line_start":22,"line_end":22,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        || {","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`x` would have to be valid for the expression at 21:5...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-49824.rs","byte_start":789,"byte_end":851,"line_start":21,"line_end":25,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    || {","highlight_start":5,"highlight_end":9},{"text":"        || {","highlight_start":1,"highlight_end":13},{"text":"            let _y = &mut x;","highlight_start":1,"highlight_end":29},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...but `x` is only valid for the lifetime  as defined on the body at 21:5","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-49824.rs","byte_start":789,"byte_end":791,"line_start":21,"line_end":21,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    || {","highlight_start":5,"highlight_end":7}],"lore borrower
[00:51:24] -    |
[00:51:24] -    = note: values in a scope are dropped in the opposite order they are created
[00:51:24] +    |     - borrowed value only lives until here
[00:51:24] + ...
[00:51:24] + LL | }
[00:51:24] +    | - borrowed value needs to live until here
[00:51:24] 13 
[00:51:24] 14 error[E0597]: `y` does not live long enough
[00:51:24] 
[00:51:24] 20    |                          capture occurs here
[00:51:24] 21 ...
[00:51:24] 21 ...
[00:51:24] 22 LL |     });
[00:51:24] -    |     - borrowed value dropped before borrower
[00:51:24] -    |
[00:51:24] -    = note: values in a scope are dropped in the opposite order they are created
[00:51:24] +    |     - borrowed value only lives until here
[00:51:24] + ...
[00:51:24] + LL | }
[00:51:24] +    | - borrowed value needs to live until here
[00:51:24] 27 error: aborting due to 2 previous errors
[00:51:24] 28 
[00:51:24] 
[00:51:24] 
[00:51:24] 
[00:51:24] The actual stderr differed from the expected stderr.
[00:51:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-big.ast/region-borrow-params-issue-29793-big.ast.stderr
[00:51:24] To update references, rerun the tests and pass the `--bless` flag
[00:51:24] To only update this specific test, also pass `--test-args regions/region-borrow-params-issue-29793-big.rs`
[00:51:24] 
[00:51:24] error in revision `ast`: 1 errors occurred comparing output.
[00:51:24] status: exit code: 1
[00:51:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-borrow-params-issue-29793-big.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-big.ast/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-big.ast/auxiliary" "-A" "unused"
[00:51:24] ------------------------------------------
[00:51:24] 
[00:51:24] ------------------------------------------
[00:51:24] stderr:
[00:51:24] stderr:
[00:51:24] ------------------------------------------
[00:51:24] {"message":"`x` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n