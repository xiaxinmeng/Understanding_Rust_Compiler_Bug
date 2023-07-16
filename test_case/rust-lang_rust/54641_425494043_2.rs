\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/existential_types/generic_type_does_not_live_long_enough.rs","byte_start":574,"byte_end":575,"line_start":16,"line_end":16,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"    let z: i32 = x; //~ ERROR mismatched types","highlight_start":18,"highlight_end":19}],"label":"expected i32, found opaque type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `i32`\n   found type `WrongGeneric::<&{integer}>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/existential_types/generic_type_does_not_live_long_enough.rs:16:18\n   |\nLL |     let z: i32 = x; //~ ERROR mismatched typeed"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
---
[00:52:11] 
[00:52:11] ---- [ui] ui/impl-trait/must_outlive_least_region_or_bound.rs stdout ----
[00:52:11] diff of stderr:
[00:52:11] 
[00:52:11] 51    |                                                 |           ...but data from `y` is returned here
[00:52:11] 52    |                                                 this parameter and the return type are declared with different lifetimes...
[00:52:11] 53 
[00:52:11] - error[E0310]: the parameter type `T` may not live long enough
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | fn ty_param_wont_outlive_static<T:Debug>(x: T) -> impl Debug + 'static {
[00:52:11] -    |                                 |
[00:52:11] -    |                                 |
[00:52:11] -    |                                 help: consider adding an explicit lifetime bound `T: 'static`...
[00:52:11] -    |
[00:52:11] - note: ...so that the type `T` will meet its required lifetime bounds
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | fn ty_param_wont_outlive_static<T:Debug>(x: T) -> impl Debug + 'static {
[00:52:11] + error: aborting due to 4 previous errors
[00:52:11] 67 
[00:52:11] - error: aborting due to 5 previous errors
[00:52:11] - 
---
[00:52:11] 72 
[00:52:11] 
[00:52:11] 
[00:52:11] The actual stderr differed from the expected stderr.
[00:52:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound/must_outlive_least_region_or_bound.stderr
[00:52:11] To update references, rerun the tests and pass the `--bless` flag
[00:52:11] To only update this specific test, also pass `--test-args impl-trait/must_outlive_least_region_or_bound.rs`
[00:52:11] error: 1 errors occurred comparing output.
[00:52:11] status: exit code: 1
[00:52:11] status: exit code: 1
[00:52:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-replacement":"&'static i32","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0621]: explicit lifetime required in the type of `x`\n  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:13:23\n   |\nLL | fn elided(x: &i32) -> impl Copy { x }\n   |              ----     ^^^^^^^^^ lifetime `'static` required\n   |              |\n   |              help: add explicit lifetime `'static` to the type of `x`: `&'static i32`\n\n"}
[00:52:11] {"message":"cannot infer an appropriate lifetime","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":624,"byte_end":633,"line_start":16,"line_end":16,"column_start":32,"column_end":41,"is_primary":false,"text":[{"text":"fn explicit<'a>(x: &'a i32) -> impl Copy { x }","highlight_start":32,"highlight_end":41}],"label":"this return type evaluates to the `'static` lifetime...","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":624,"byte_end":633,"line_start":16,"line_end":16,"column_start":32,"column_end":41,"is_primary":false,"text":[{"text":"fn explicit<'a>(x: &'a i32) -> impl Copy { x }","highlight_start":32,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `existential type`","def_site_span":{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":624,"byte_end":633,"mary":true,"text":[{"text":"fn explicit<'a>(x: &'a i32) -> impl Copy { x }","highlight_start":32,"highlight_end":41}],"label":null,"suggested_replacement":"impl Copy + 'a","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":624,"byte_end":633,"line_start":16,"line_end":16,"column_start":32,"column_end":41,"is_primary":false,"text":[{"text":"fn explicit<'a>(x: &'a i32) -> impl Copy { x }","highlight_start":32,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `existential type`","def_site_span":{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":624,"byte_end":633,"line_start":16,"line_end":16,"column_start":32,"column_end":41,"is_primary":false,"text":[{"text":"fn explicit<'a>(x: &'a i32) -> impl Copy { x }","highlight_start":32,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: cannot infer an appropriate lifetime\n  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:16:44\n   |\nLL | fn explicit<'a>(x: &'a i32) -> impl Copy { x }\n   |                                ---------   ^ ...but this borrow...\n   |                                |\n   |                                this return type evaluates to the `'static` lifetime...\n   |\nnote: ...can't outlive the lifetime 'a as defined on the function body at 16:13\n  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:16:13\n   |\nLL | fn explicit<'a>(x: &'a i32) -> impl Copy { x }\n   |             ^^\nhelp: you can add a constraint to the return type to make it last less than `'static` and match the lifetime 'a as defined on the function body at 16:13\n   |\nLL | fn explicit<'a>(x: &'a i32) -> impl Copy + 'a { x }\n   |                                ^^^^^^^^^^^^^^\n\n"}
[00:52:11] {"message":"cannot infer an appropriate lifetime","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":792,"byte_end":824,"line_start":22,"line_end":22,"column_start":34,"column_end":66,"is_primary":false,"text":[{"text":"fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }","highlight_start":34,"highlight_end":66}],"label":"this return type evaluates to the `'static` lifetime...","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":792,"byte_end":824,"line_start":22,"line_end":22,"column_start":34,"column_end":66,"is_primary":false,"text":[{"text":"fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }","highlight_start":34,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `existential type`","def_site_span":{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":792,"byte_end":824,"line_start":22,"line_end":22,"column_start":34t":34,"column_end":66,"is_primary":true,"text":[{"text":"fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }","highlight_start":34,"highlight_end":66}],"label":null,"suggested_replacement":"impl LifetimeTrait<'a> + 'static + 'a","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":792,"byte_end":824,"line_start":22,"line_end":22,"column_start":34,"column_end":66,"is_primary":false,"text":[{"text":"fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }","highlight_start":34,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `existential type`","def_site_span":{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":792,"byte_end":824,"line_start":22,"line_end":22,"column_start":34,"column_end":66,"is_primary":false,"text":[{"text":"fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }","highlight_start":34,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: cannot infer an appropriate lifetime\n  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:22:69\n   |\nLL | fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }\n   |                                  --------------------------------   ^ ...but this borrow...\n   |                                  |\n   |                     d_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":1045,"byte_end":1061,"line_start":27,"line_end":27,"column_start":61,"column_end":77,"is_primary":true,"text":[{"text":"fn move_lifetime_into_fn<'a, 'b>(x: &'a u32, y: &'b u32) -> impl Fn(&'a u32) {","highlight_start":61,"highlight_end":77}],"label":"...but data from `y` is returned here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":1045,"byte_end":1061,"line_start":27,"line_end":27,"column_start":61,"column_end":77,"is_primary":false,"text":[{"text":"fn move_lifetime_into_fn<'a, 'b>(x: &'a u32, y: &'b u32) -> impl Fn(&'a u32) {","highlight_start":61,"highlight_end":77}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `existential type`","def_site_span":{"file_name":"/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs","byte_start":1045,"byte_end":1061,"line_start":27,"line_end":27,"column_start":61,"column_end":77,"is_primary":false,"text":[{"text":"fn move_lifetime_into_fn<'a, 'b>(x: &'a u32, y: &'b u32) -> impl Fn(&'a u32) {","highlight_start":61,"highlight_end":77}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error[E0623]: lifetime mismatch\n  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:27:61\n   |\nLL | fn move_lifetime_into_fn<'a, 'b>(x: &'a u32, y: &'b u32) -> impl Fn(&'a u32) {\n   |                                                 -------     ^^^^^^^^^^^^^^^^\n   |                                                 |           |\n   |                                                 |           ...but data from `y` is returned here\n   |                                                 this parameter and the return type are declared with different lifetimes...\n\n"}
[00:52:11] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:52:11] {"message":"Some errors occurred: E0621, E0623.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0621, E0623.\n"}
[00:52:11] {"message":"For more information about an error, try `rustc --explain E0621`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0621`.\n"}
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] thread '[ui] ui/impl-trait/must_outlive_least_region_or_bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:52:11] 
[00:52:11] 
[00:52:11] ---- [ui] ui/impl-trait/type_parameters_captured.rs stdout ----
[00:52:11] 
[00:52:11] error: ui test compiled successfully!
[00:52:11] status: exit code: 0
[00:52:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/type_parameters_captured.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/type_parameters_captured/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/type_parameters_captured/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
---
[00:52:11] ---- [ui] ui/issues/issue-18937.rs stdout ----
[00:52:11] 
[00:52:11] error: ui test compiled successfully!
[00:52:11] status: exit code: 0
[00:52:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18937.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18937/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18937/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
[00:52:11] stderr:
[00:52:11]lly!
[00:52:11] status: exit code: 0
[00:52:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-where-clause-env-wrong-lifetime.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-where-clause-env-wrong-lifetime/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-where-clause-env-wrong-lifetime/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
---
[00:52:11] ---- [ui] ui/regions/regions-close-associated-type-into-object.rs stdout ----
[00:52:11] 
[00:52:11] error: ui test compiled successfully!
[00:52:11] status: exit code: 0
[00:52:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-associated-type-into-object.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-associated-type-into-object/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-associated-type-into-object/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
---
[00:52:11] 
[00:52:11] ---- [ui] ui/regions/regions-close-object-into-object-5.rs stdout ----
[00:52:11] diff of stderr:
[00:52:11] 
[00:52:11] - error[E0310]: the parameter type `T` may not live long enough
[00:52:11] -   --> $DIR/regions-close-object-into-object-5.rs:27:5
[00:52:11] + error[E0597]: `*v` does not live long enough
[00:52:11] +   --> $DIR/regions-close-object-into-object-5.rs:27:12
[00:52:11] 3    |
[00:52:11] - LL | fn f<'a, T, U>(v: Box<A<T>+'static>) -> Box<X+'static> {
[00:52:11] -    |          - help: consider adding an explicit lifetime bound `T: 'static`...
[00:52:11] - LL |     // oh dear!
[00:52:11] 7 LL |     box B(&*v) as Box<X>
[00:52:11] -    |     ^^^^^^^^^^
[00:52:11] +    |            ^^ borrowed value does not live long enough
[00:52:11] + ...
[00:52:11] + LL | }
[00:52:11] +    | - borrowed value only lives until here
[00:52:11] 9    |
[00:52:11] - note: ...so that the type `B<'_, T>` will meet its required lifetime bounds
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL |     box B(&*v) as Box<X>
[00:52:11] -    |     ^^^^^^^^^^
[00:52:11] +    = note: borrowed value must be valid for the static lifetime...
[00:52:11] 15 
[00:52:11] - error[E0310]: the parameter type `T` may not live long enough
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | fn f<'a, T, U>(v: Box<A<T>+'static>) -> Box<X+'static> {
[00:52:11] -    |          - help: consider adding an explicit lifetime bound `T: 'static`...
[00:52:11] - LL |     // oh dear!
[00:52:11] - LL |     box B(&*v) as Box<X>
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - note: ...so that it can be closed over into an object
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL |     box B(&*v) as Box<X>
[00:52:11] + error: aborting due to previous error
[00:52:11] 30 
[00:52:11] 30 
[00:52:11] - error[E0310]: the parameter type `T` may not live long enough
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | fn f<'a, T, U>(v: Box<A<T>+'static>) -> Box<X+'static> {
[00:52:11] -    |          - help: consider adding an explicit lifetime bound `T: 'static`...
[00:52:11] - LL |     // oh dear!
[00:52:11] - LL |     box B(&*v) as Box<X>
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - note: ...so that the type `T` will meet its required lifetime bounds
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL |     box B(&*v) as Box<X>
[00:52:11] - 
[00:52:11] - 
[00:52:11] - error[E0310]: the parameter type `T` may not live long enough
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | fn f<'a, T, U>(v: Box<A<T>+'static>) -> Box<X+'static> {
[00:52:11] -    |          - help: consider adding an explicit lifetime bound `T: 'static`...
[00:52:11] - LL |     // oh dear!
[00:52:11] - LL |     box B(&*v) as Box<X>
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - note: ...so that the reference type `&dyn A<T>` does not outlive the data it points at
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL |     box B(&*v) as Box<X>
[00:52:11] - 
[00:52:11] - 
[00:52:11] - error[E0310]: the parameter type `T` may not live long enough
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | fn f<'a, T, U>(v: Box<A<T>+'static>) -> Box<X+'static> {
[00:52:11] -    |          - help: consider adding an explicit lifetime bound `T: 'static`...
[00:52:11] - LL |     // oh dear!
[00:52:11] - LL |     box B(&*v) as Box<X>
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - note: ...so that the type `T` will meet its required lifetime bounds
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL |     box B(&*v) as Box<X>
[00:5-object-5.rs","byte_start":745,"byte_end":747,"line_start":27,"line_end":27,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"    box B(&*v) as Box<X>","highlight_start":12,"highlight_end":14}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/regions/regions-close-object-into-object-5.rs","byte_start":1161,"byte_end":1162,"line_start":34,"line_end":34,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"borrowed value only lives until here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: `*v` does not live long enough\n  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:27:12\n   |\nLL |     box B(&*v) as Box<X>\n   |            ^^ borrowed value does not live long enough\n...\nLL | }\n   | - borrowed value only lives until here\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
[00:52:11] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:52:11] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:52:112:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-param-into-object.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-param-into-object/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-param-into-object/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
---
[00:52:11] ---- [ui] ui/regions/regions-enum-not-wf.rs stdout ----
[00:52:11] 
[00:52:11] error: ui test compiled successfully!
[00:52:11] status: exit code: 0
[00:52:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-enum-not-wf.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-enum-not-wf/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-enum-not-wf/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
---
[00:52:11] ---- [ui] ui/regions/regions-implied-bounds-projection-gap-1.rs stdout ----
[00:52:11] 
[00:52:11] error: ui test compiled successfully!
[00:52:11] status: exit code: 0
[00:52:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-implied-bounds-projection-gap-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-implied-bounds-projection-gap-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-implied-bounds-projection-gap-1/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
[00:52:11] stderr:
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] 
[00:rustc" "/checkout/src/test/ui/regions/regions-infer-bound-from-trait.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-bound-from-trait/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-bound-from-trait/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
---
[00:52:11] 
[00:52:11] ---- [ui] ui/regions/regions-struct-not-wf.rs stdout ----
[00:52:11] diff of stderr:
[00:52:11] 
[00:52:11] - error[E0309]: the parameter type `T` may not live long enough
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | impl<'a, T> Trait<'a, T> for usize {
[00:52:11] -    |          - help: consider adding an explicit lifetime bound `T: 'a`...
[00:52:11] - LL |     type Out = &'a T;
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - note: ...so that the reference type `&'a T` does not outlive the data it points at
[00:52:11] -    |
[00:52:11] -    |
[00:52:11-wf.stderr
[00:52:11] To update references, rerun the tests and pass the `--bless` flag
[00:52:11] To only update this specific test, also pass `--test-args regions/regions-struct-not-wf.rs`
[00:52:11] error: 1 errors occurred comparing output.
[00:52:11] status: exit code: 1
[00:52:11] status: exit code: 1
[00:52:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-struct-not-wf.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-struct-not-wf/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-struct-not-wf/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
[00:52:11] stderr:
[00:52:11] ------------------------------------------
[00:52:11] {"message":"in type `&'a &'b T`, reference has a longer lifetime than the data it references","code":{"code":"E0491","explanation":"\nA reference has a longer lifetime than the data it references.\n\nErroneous code example:\n\n