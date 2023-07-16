\n"},"level":"error","spans":[{"file_name":"<::core::macros::assert_eq macros>","byte_start":126,"byte_end":137,"line_start":5,"line_end":5,"column_start":22,"column_end":33,"is_primary":true,"text":[{"text":"if ! ( * left_val == * right_val ) {","highlight_start":22,"highlight_end":33}],"label":"expected enum `std::result::Result`, found integral variable","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/builtin-superkinds/builtin-superkinds-self-type.rs","byte_start":881,"byte_end":912,"line_start":26,"line_end":26,"column_start":5,"column_end":36,"is_primary":false,"text":[{"text":"    assert_eq!(rx.recv(), 1193182);","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert_eq!","def_site_span":{"file_name":"<::core::macros::assert_eq macros>","byte_start":0,"byte_end":671,"line_start":1,"line_end":21,"column_start":1,"column_end":69,"is_primary":false,"text":[{"text":"( $ left : expr , $ right : expr ) => (","highlight_start":1,"highlight_end":40},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & $ left , & $ right ) {","highlight_start":1,"highlight_end":33},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`\"# ,","highlight_start":1,"highlight_end":19},{"text":"left_val , right_val ) } } } } ) ; ( $ left : expr , $ right : expr , ) => (","highlight_start":1,"highlight_end":77},{"text":"{ assert_eq ! ( $ left , $ right ) } ) ; (","highlight_start":1,"highlight_end":43},{"text":"$ left : expr , $ right : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":59},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & ( $ left ) , & ( $ right ) ) {","highlight_start":1,"highlight_end":41},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`: {}\"# ,","highlight_start":1,"highlight_end":23},{"text":"left_val , right_val , format_args ! ( $ ( $ arg ) + ) ) } } } } ) ;","highlight_start":1,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"expected type `std::result::Result<{integer}, std::sync::mpsc::RecvError>`\n   found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try using a variant of the expected type","code":null,"level":"help","spans":[{"file_name":"<::core::macros::assert_eq macros>","byte_start":126,"byte_end":137,"line_start":5,"line_end":5,"column_start":22,"column_end":33,"is_primary":true,"text":[{"text":"if ! ( * left_val == * right_val ) {","highlight_start":22,"highlight_end":33}],"label":null,"suggested_replacement":"Ok(*right_val)","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"/checkout/src/test/ui/builtin-superkinds/builtin-superkinds-self-type.rs","byte_start":881,"byte_end":912,"line_start":26,"line_end":26,"column_start":5,"column_end":36,"is_primary":false,"text":[{"text":"    assert_eq!(rx.recv(), 1193182);","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert_eq!","def_site_span":{"file_name":"<::core::macros::assert_eq macros>","byte_start":0,"byte_end":671,"line_start":1,"line_end":21,"column_start":1,"column_end":69,"is_primary":false,"text":[{"text":"( $ left : expr , $ right : expr ) => (","highlight_start":1,"highlight_end":40},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & $ left , & $ right ) {","highlight_start":1,"highlight_end":33},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`\"# ,","highlight_start":1,"highlight_end":19},{"text":"left_val , right_val ) } } } } ) ; ( $ left : expr , $ right : expr , ) => (","highlight_start":1,"highlight_end":77},{"text":"{ assert_eq ! ( $ left , $ right ) } ) ; (","highlight_start":1,"highlight_end":43},{"text":"$ left : expr , $ right : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":59},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & ( $ left ) , & ( $ right ) ) {","highlight_start":1,"highlight_end":41},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`: {}\"# ,","highlight_start":1,"highlight_end":23},{"text":"left_val , right_val , format_args ! ( $ ( $ arg ) + ) ) } } } } ) ;","highlight_start":1,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/builtin-superkinds/builtin-superkinds-self-type.rs:26:5\n   |\nLL |     assert_eq!(rx.recv(), 1193182);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |     |\n   |     expected enum `std::result::Result`, found integral variable\n   |     help: try using a variant of the expected type: `Ok(*right_val)`\n   |\n   = note: expected type `std::result::Result<{integer}, std::sync::mpsc::RecvError>`\n              found type `{integer}`\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:52:11] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:52:11] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] thread '[ui] ui/builtin-superkinds/builtin-superkinds-self-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:52:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:11] 
[00:52:11] ---- [ui] ui/compare-method/proj-outlives-region.rs stdout ----
[00:52:11] 
[00:52:11] error: ui test compiled successfully!
[00:52:11] status: exit code: 0
[00:52:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/compare-method/proj-outlives-region.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/proj-outlives-region/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/proj-outlives-region/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
---
[00:52:11] ---- [ui] ui/compare-method/region-unrelated.rs stdout ----
[00:52:11] 
[00:52:11] error: ui test compiled successfully!
[00:52:11] status: exit code: 0
[00:52:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/compare-method/region-unrelated.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/region-unrelated/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/region-unrelated/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
---
[00:52:11] 
[00:52:11] ---- [ui] ui/existential_types/generic_type_does_not_live_long_enough.rs stdout ----
[00:52:11] diff of stderr:
[00:52:11] 
[00:52:11] 7    = note: expected type `i32`
[00:52:11] 8               found type `WrongGeneric::<&{integer}>`
[00:52:11] 9 
[00:52:11] - error[E0310]: the parameter type `T` may not live long enough
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | existential type WrongGeneric<T>: 'static;
[00:52:11] - ...
[00:52:11] - ...
[00:52:11] - LL | fn wrong_generic<T>(t: T) -> WrongGeneric<T> {
[00:52:11] -    |                  - help: consider adding an explicit lifetime bound `T: 'static`...
[00:52:11] -    |
[00:52:11] - note: ...so that the type `T` will meet its required lifetime bounds
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | existential type WrongGeneric<T>: 'static;
[00:52:--------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
[00:52:11] stderr:
[00:52:11] ------------------------------------------
[00:52:11] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n