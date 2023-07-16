\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs","byte_start":216,"byte_end":222,"line_start":15,"line_end":15,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"    let _: &str = bomp(); //~ ERROR mismatched types","highlight_start":19,"highlight_end":25}],"label":"expected &str, found opaque type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `&str`\n   found type `Boo`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs:15:19\n   |\nLL |     let _: &str = bomp(); //~ ERROR mismatched types\n   |                   ^^^^^^ expected &str, found opaque type\n   |\n   = note: expected type `&str`\n              found type `Boo`\n\n"}
[01:18:42] {"message":"concrete type differs from previous defining existential type use","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs","byte_start":315,"byte_end":357,"line_start":22,"line_end":24,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn bomp_loop() -> boo::Boo {","highlight_start":1,"highlight_end":29},{"text":"    loop {}","highlight_start":1,"highlight_end":12},{"text":"}","highlight_start":1,"highlight_end":2}],"label":"expected `&'static str`, got `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"previous use here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs","byte_start":108,"byte_end":143,"line_start":7,"line_end":9,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn bomp() -> Boo {","highlight_start":5,"highlight_end":23},{"text":"        \"\"","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: concrete type differs from previous defining existential type use\n  --> /checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs:22:1\n   |\nLL | / fn bomp_loop() -> boo::Boo {\nLL | |     loop {}\nLL | | }\n   | |_^ expected `&'static str`, got `()`\n   |\nnote: previous use here\n  --> /checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs:7:5\n   |\nLL | /     fn bomp() -> Boo {\nLL | |         \"\"\nLL | |     }\n   | |_____^\n\n"}
[01:18:42] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[01:18:42] 
[01:18:42] ------------------------------------------
[01:18:42] 
[01:18:42] 
[01:18:42] thread '[ui] ui/existential_types/no_revealing_outside_defining_module.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:18:42] 
[01:18:42] ---- [ui] ui/feature-gates/feature-gate-associated_type_bounds.rs stdout ----
[01:18:42] 
[01:18:42] error: /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:57: expected error not found: associated type bounds are unstable (see issue #52662) [E0658]
[01:18:42] 
[01:18:42] error: /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:57: expected error not found: `impl Trait` not allowed outside of function and inherent method return types [E0562]
[01:18:42] error: /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:66: expected error not found: associated type bounds are unstable (see issue #52662) [E0658]
[01:18:42] 
[01:18:42] 
[01:18:42] error: /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:66: expected error not found: `impl Trait` not allowed outside of function and inherent method return types [E0562]
[01:18:42] error: /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:76: expected error not found: associated type bounds are unstable (see issue #52662) [E0658]
[01:18:42] 
[01:18:42] 
[01:18:42] error: /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:76: expected error not found: `impl Trait` not allowed outside of function and inherent method return types [E0562]
[01:18:42] error: 0 unexpected errors found, 6 expected errors not found
[01:18:42] status: exit code: 1
[01:18:42] status: exit code: 1
[01:18:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/auxiliary" "-A" "unused"
[01:18:42]     Error {
[01:18:42]         line_num: 57,
[01:18:42]         kind: Some(
[01:18:42]             Error
[01:18:42]             Error
[01:18:42]         ),
[01:18:42]         msg: "associated type bounds are unstable (see issue #52662) [E0658]"
[01:18:42]     Error {
[01:18:42]         line_num: 57,
[01:18:42]         kind: Some(
[01:18:42]             Error
[01:18:42]             Error
[01:18:42]         ),
[01:18:42]         msg: "`impl Trait` not allowed outside of function and inherent method return types [E0562]"
[01:18:42]     Error {
[01:18:42]         line_num: 66,
[01:18:42]         kind: Some(
[01:18:42]             Error
[01:18:42]             Error
[01:18:42]         ),
[01:18:42]         msg: "associated type bounds are unstable (see issue #52662) [E0658]"
[01:18:42]     Error {
[01:18:42]         line_num: 66,
[01:18:42]         kind: Some(
[01:18:42]             Error
[01:18:42]             Error
[01:18:42]         ),
[01:18:42]         msg: "`impl Trait` not allowed outside of function and inherent method return types [E0562]"
[01:18:42]     Error {
[01:18:42]         line_num: 76,
[01:18:42]         kind: Some(
[01:18:42]             Error
[01:18:42]             Error
[01:18:42]         ),
[01:18:42]         msg: "associated type bounds are unstable (see issue #52662) [E0658]"
[01:18:42]     Error {
[01:18:42]         line_num: 76,
[01:18:42]         kind: Some(
[01:18:42]             Error
[01:18:42]             Error
[01:18:42]         ),
[01:18:42]         msg: "`impl Trait` not allowed outside of function and inherent method return types [E0562]"
[01:18:42] ]
[01:18:42] 
[01:18:42] thread '[ui] ui/feature-gates/feature-gate-associated_type_bounds.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1381:13
[01:18:42] 
[01:18:42] 
[01:18:42] ---- [ui] ui/lint/lint-duplicate-bounds.rs stdout ----
[01:18:42] 
[01:18:42] error: test compilation failed although it shouldn't!
[01:18:42] status: exit code: 1
[01:18:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-duplicate-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-duplicate-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-duplicate-bounds/auxiliary" "-A" "unused"
[01:18:42] ------------------------------------------
[01:18:42] 
[01:18:42] ------------------------------------------
[01:18:42] stderr:
[01:18:42] stderr:
[01:18:42] ------------------------------------------
[01:18:42] {"message":"only auto traits can be used as additional traits in a trait object","code":{"code":"E0225","explanation":"\nYou attempted to use multiple types as bounds for a closure or trait object.\nRust does not currently support this. A simple example that causes this error:\n\n