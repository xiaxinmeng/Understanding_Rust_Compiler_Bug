\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gates/feature-gate-panic-implementation.rs","byte_start":520,"byte_end":543,"line_start":16,"line_end":16,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[panic_implementation] //~ ERROR #[panic_implementation] is an unstable feature (see issue #44489)","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(panic_implementation)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: #[panic_implementation] is an unstable feature (see issue #44489)\n  --> /checkout/src/test/ui/feature-gates/feature-gate-panic-implementation.rs:16:1\n   |\nLL | #[panic_implementation] //~ ERROR #[panic_implementation] is an unstable feature (see issue #44489)\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(panic_implementation)] to the crate attributes to enable\n\n"}
[00:49:44] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:44] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[00:49:44] ------------------------------------------
[00:49:44] 
[00:49:44] thread '[ui] ui/feature-gates/feature-gate-panic-implementation.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:49:44] 
[00:49:44] 
[00:49:44] ---- [ui] ui/resolve/issue-24968.rs stdout ----
[00:49:44] diff of stderr:
[00:49:44] 
[00:49:44] 2   --> $DIR/issue-24968.rs:11:11
[00:49:44] 3    |
[00:49:44] 4 LL | fn foo(_: Self) {
[00:49:44] -    |           ^^^^ `Self` is only available in traits and impls
[00:49:44] +    |           ^^^^ `Self` is only available in impls and traits
[00:49:44] 7 error: aborting due to previous error
[00:49:44] 8 
[00:49:44] 
[00:49:44] 
[00:49:44] 
[00:49:44] The actual stderr differed from the expected stderr.
[00:49:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-24968/issue-24968.stderr
[00:49:44] To update references, rerun the tests and pass the `--bless` flag
[00:49:44] To only update this specific test, also pass `--test-args resolve/issue-24968.rs`
[00:49:44] error: 1 errors occurred comparing output.
[00:49:44] status: exit code: 1
[00:49:44] status: exit code: 1
[00:49:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-24968.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-24968/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-24968/auxiliary" "-A" "unused"
[00:49:44] ------------------------------------------
[00:49:44] 
[00:49:44] ------------------------------------------
[00:49:44] stderr:
[00:49:44] stderr:
[00:49:44] ------------------------------------------
[00:49:44] {"message":"cannot find type `Self` in this scope","code":{"code":"E0411","explanation":"\nThe `Self` keyword was used outside an impl, trait, or type definition.\n\nErroneous code example:\n\n