\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/allocator/not-an-allocator.rs","byte_start":487,"byte_end":507,"line_start":12,"line_end":12,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"static A: usize = 0;","highlight_start":1,"highlight_end":21}],"label":"the trait `std::alloc::GlobalAlloc` is not implemented for `usize`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/allocator/not-an-allocator.rs","byte_start":487,"byte_end":507,"line_start":12,"line_end":12,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"static A: usize = 0;","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[global_allocator]","def_site_span":null}}],"children":[{"message":"required by `std::alloc::GlobalAlloc::alloc_zeroed`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `usize: std::alloc::GlobalAlloc` is not satisfied\n  --> /checkout/src/test/ui/allocator/not-an-allocator.rs:12:1\n   |\nLL | static A: usize = 0;\n   | ^^^^^^^^^^^^^^^^^^^^ the trait `std::alloc::GlobalAlloc` is not implemented for `usize`\n   |\n   = note: required by `std::alloc::GlobalAlloc::alloc_zeroed`\n\n"}
[00:49:31] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:49:31] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/allocator/not-an-allocator.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:49:31] 1 error[E0308]: mismatched types
[00:49:31] -   --> $DIR/associated-type-projection-from-supertrait.rs:43:23
[00:49:31] +   --> $DIR/associated-type-projection-from-supertrait.rs:51:28
[00:49:31] 3    |
[00:49:31] - LL | fn b() { dent(ModelT, Blue); } //~ ERROR mismatched types
[00:49:31] -    |                       ^^^^ expected struct `Black`, found struct `Blue`
[00:49:31] + LL | fn g() { ModelU.chip_paint(Black); } //~ ERROR mismatched types
[00:49:31] +    |                            ^^^^^ expected struct `Blue`, found struct `Black`
[00:49:31] -    = note: expected type `Black`
[00:49:31] -               found type `Blue`
[00:49:31] +    = note: expected type `Blue`
[00:49:31] +               found type `Black`
---
[00:49:31] 28 error[E0308]: mismatched types
[00:49:31] -   --> $DIR/associated-type-projection-from-supertrait.rs:51:28
[00:49:31] +   --> $DIR/associated-type-projection-from-supertrait.rs:43:23
[00:49:31] 30    |
[00:49:31] - LL | fn g() { ModelU.chip_paint(Black); } //~ ERROR mismatched types
[00:49:31] -    |                            ^^^^^ expected struct `Blue`, found struct `Black`
[00:49:31] + LL | fn b() { dent(ModelT, Blue); } //~ ERROR mismatched types
[00:49:31] +    |                       ^^^^ expected struct `Black`, found struct `Blue`
[00:49:31] -    = note: expected type `Blue`
[00:49:31] -               found type `Black`
[00:49:31] +    = note: expected type `Black`
[00:49:31] +               found type `Blue`
[00:49:31] +               found type `Blue`
[00:49:31] 36 
[00:49:31] 37 error: aborting due to 4 previous errors
[00:49:31] 38 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/associated-type-projection-from-supertrait.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args associated-type/associated-type-projection-from-supertrait.rs`
[00:49:31] error: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-supertrait/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n