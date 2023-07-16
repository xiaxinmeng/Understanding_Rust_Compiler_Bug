\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs","byte_start":220,"byte_end":226,"line_start":15,"line_end":15,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"    let _: &str = bomp(); //~ ERROR mismatched types","highlight_start":19,"highlight_end":25}],"label":"expected &str, found opaque type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `&str`\n   found type `Boo`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs:15:19\n   |\nLL |     let _: &str = bomp(); //~ ERROR mismatched types\n   |                   ^^^^^^ expected &str, found opaque type\n   |\n   = note: expected type `&str`\n              found type `Boo`\n\n"}
[01:13:50] {"message":"concrete type differs from previous defining existential type use","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs","byte_start":319,"byte_end":361,"line_start":22,"line_end":24,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn bomp_loop() -> boo::Boo {","highlight_start":1,"highlight_end":29},{"text":"    loop {}","highlight_start":1,"highlight_end":12},{"text":"}","highlight_start":1,"highlight_end":2}],"label":"expected `&'static str`, got `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"previous use here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs","byte_start":108,"byte_end":143,"line_start":7,"line_end":9,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn bomp() -> Boo {","highlight_start":5,"highlight_end":23},{"text":"        \"\"","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: concrete type differs from previous defining existential type use\n  --> /checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs:22:1\n   |\nLL | / fn bomp_loop() -> boo::Boo {\nLL | |     loop {}\nLL | | }\n   | |_^ expected `&'static str`, got `()`\n   |\nnote: previous use here\n  --> /checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs:7:5\n   |\nLL | /     fn bomp() -> Boo {\nLL | |         \"\"\nLL | |     }\n   | |_____^\n\n"}
[01:13:50] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[01:13:50] 
[01:13:50] ------------------------------------------
[01:13:50] 
[01:13:50] 
[01:13:50] thread '[ui] ui/existential_types/no_revealing_outside_defining_module.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:13:50] 
[01:13:50] ---- [ui] ui/feature-gates/feature-gate-associated_type_bounds.rs stdout ----
[01:13:50] diff of stderr:
[01:13:50] 
[01:13:50] 87    = help: add #![feature(associated_type_bounds)] to the crate attributes to enable
[01:13:50] 89 error[E0658]: associated type bounds are unstable (see issue #52662)
[01:13:50] -   --> $DIR/feature-gate-associated_type_bounds.rs:62:24
[01:13:50] +   --> $DIR/feature-gate-associated_type_bounds.rs:59:24
[01:13:50] 91    |
[01:13:50] 91    |
[01:13:50] 92 LL | static _sdef: impl Tr1<As1: Copy> = S1;
[01:13:50] 
[01:13:50] 
[01:13:50] 95    = help: add #![feature(associated_type_bounds)] to the crate attributes to enable
[01:13:50] 97 error[E0658]: associated type bounds are unstable (see issue #52662)
[01:13:50] -   --> $DIR/feature-gate-associated_type_bounds.rs:72:21
[01:13:50] +   --> $DIR/feature-gate-associated_type_bounds.rs:66:21
[01:13:50] 99    |
[01:13:50] 99    |
[01:13:50] 100 LL |     let _: impl Tr1<As1: Copy> = S1;
[01:13:50] 
[01:13:50] 111    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[01:13:50] 112 
[01:13:50] 112 
[01:13:50] 113 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[01:13:50] +   --> $DIR/feature-gate-associated_type_bounds.rs:59:15
[01:13:50] 115    |
[01:13:50] 115    |
[01:13:50] 116 LL | static _sdef: impl Tr1<As1: Copy> = S1;
[01:13:50] 
[01:13:50] 119    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[01:13:50] 120 
[01:13:50] 120 
[01:13:50] 121 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[01:13:50] +   --> $DIR/feature-gate-associated_type_bounds.rs:66:12
[01:13:50] 123    |
[01:13:50] 123    |
[01:13:50] 124 LL |     let _: impl Tr1<As1: Copy> = S1;
[01:13:50] 
[01:13:50] 
[01:13:50] The actual stderr differed from the expected stderr.
[01:13:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/feature-gate-associated_type_bounds.stderr
[01:13:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/feature-gate-associated_type_bounds.stderr
[01:13:50] To update references, rerun the tests and pass the `--bless` flag
[01:13:50] To only update this specific test, also pass `--test-args feature-gates/feature-gate-associated_type_bounds.rs`
[01:13:50] error: 1 errors occurred comparing output.
[01:13:50] status: exit code: 1
[01:13:50] status: exit code: 1
[01:13:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/auxiliary" "-A" "unused"
[01:13:50] ------------------------------------------
[01:13:50] 
[01:13:50] ------------------------------------------
[01:13:50] stderr:
[01:13:50] stderr:
[01:13:50] ------------------------------------------
[01:13:50] {"message":"associated type bounds are unstable (see issue #52662)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n