\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-type-projection-from-multiple-supertraits.rs","byte_start":1071,"byte_end":1079,"line_start":38,"line_end":38,"column_start":29,"column_end":37,"is_primary":true,"text":[{"text":"fn paint<C:BoxCar>(c: C, d: C::Color) {","highlight_start":29,"highlight_end":37}],"label":"ambiguous associated type `Color`","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/associated-type-projection-from-multiple-supertraits.rs","byte_start":654,"byte_end":665,"line_start":21,"line_end":21,"column_start":5,"column_end":16,"is_primary":false,"text":[{"text":"    type Color;","highlight_start":5,"highlight_end":16}],"label":"ambiguous `Color` from `Box`","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/associated-type-projection-from-multiple-supertraits.rs","byte_start":596,"byte_end":607,"line_start":15,"line_end":15,"column_start":5,"column_end":16,"is_primary":false,"text":[{"text":"    type Color;","highlight_start":5,"highlight_end":16}],"label":"ambiguous `Color` from `Vehicle`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0221]: ambiguous associated type `Color` in bounds of `C`\n  --> /checkout/src/test/ui/associated-type-projection-from-multiple-supertraits.rs:38:29: in fn paint\n   |\nLL |     type Color;\n   |     ----------- ambiguous `Color` from `Vehicle`\n...\nLL |     type Color;\n   |     ----------- ambiguous `Color` from `Box`\n...\nLL | fn paint<C:BoxCar>(c: C, d: C::Color) {\n   |                             ^^^^^^^^ ambiguous associated type `Color`\n\n"}
[00:48:30] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:48:30] {"message":"Some errors occurred: E0191, E0221.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0191, E0221.\n"}
[00:48:30] {"message":"For more information about an error, try `rustc --explain E0191`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0191`.\n"}
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] thread '[ui] ui/associated-type-projection-from-multiple-supertraits.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:48:30] 
[00:48:30] 
[00:48:30] ---- [ui] ui/associated-types-in-ambiguous-context.rs stdout ----
[00:48:30]  diff of stderr:
[00:48:30] 
[00:48:30] 1 error[E0223]: ambiguous associated type
[00:48:30] -   --> $DIR/associated-types-in-ambiguous-context.rs:16:36
[00:48:30] +   --> $DIR/associated-types-in-ambiguous-context.rs:16:36: in fn get
[00:48:30] 3    |
[00:48:30] 4 LL | fn get<T:Get,U:Get>(x: T, y: U) -> Get::Value {}
[00:48:30] 5    |                                    ^^^^^^^^^^ ambiguous associated type
[00:48:30] 
[00:48:30] 15    = note: specify the type using the syntax `<Type as std::ops::Deref>::Target`
[00:48:30] 17 error[E0223]: ambiguous associated type
[00:48:30] -   --> $DIR/associated-types-in-ambiguous-context.rs:21:23
[00:48:30] -   --> $DIR/associated-types-in-ambiguous-context.rs:21:23
[00:48:30] +   --> $DIR/associated-types-in-ambiguous-context.rs:21:23: in trait Grab::grab
[00:48:30] 19    |
[00:48:30] 20 LL |     fn grab(&self) -> Grab::Value;
[00:48:30] 21    |                       ^^^^^^^^^^^ ambiguous associated type
[00:48:30] 
[00:48:30] The actual stderr differed from the expected stderr.
[00:48:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types-in-ambiguous-context.stderr
[00:48:30] To update references, run this command from build directory:
[00:48:30] To update references, run this command from build directory:
[00:48:30] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'associated-types-in-ambiguous-context.rs'
[00:48:30] 
[00:48:30] error: 1 errors occurred comparing output.
[00:48:30] status: exit code: 101
[00:48:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types-in-ambiguous-context.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types-in-ambiguous-context.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types-in-ambiguous-context.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] ------------------------------------------
[00:48:30] stderr:
[00:48:30] stderr:
[00:48:30] ------------------------------------------
[00:48:30] {"message":"ambiguous associated type","code":{"code":"E0223","explanation":"\nAn attempt was made to retrieve an associated type, but the type was ambiguous.\nFor example:\n\n