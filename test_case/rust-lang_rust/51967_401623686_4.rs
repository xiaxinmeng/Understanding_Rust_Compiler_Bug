\n\nIf you don't know the basics of Rust, you can go look to the Rust Book to get\nstarted: https://doc.rust-lang.org/book/\n"},"level":"error","spans":[],"children":[{"message":"consider adding a `main` function to `/checkout/src/test/ui/hygiene/no_implicit_prelude.rs`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0601]: `main` function not found in crate `no_implicit_prelude`\n   |\n   = note: consider adding a `main` function to `/checkout/src/test/ui/hygiene/no_implicit_prelude.rs`\n\n"}
[00:45:01] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:45:01] {"message":"Some errors occurred: E0433, E0601.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0433, E0601.\n"}
[00:45:01] {"message":"For more information about an error, try `rustc --explain E0433`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0433`.\n"}
[00:45:01] ------------------------------------------
[00:45:01] 
0:45:01] error: 1 errors occurred comparing output.
[00:45:01] status: exit code: 101
[00:45:01] status: exit code: 101
[00:45:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl_trait_projections.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl_trait_projections/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl_trait_projections/auxiliary" "-A" "unused"
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] ------------------------------------------
[00:45:01] stderr:
[00:45:01] stderr:
[00:45:01] ------------------------------------------
[00:45:01] {"message":"`impl Trait` is not allowed in path parameters","code":{"code":"E0667","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl_trait_projections.rs","byte_start":720,"byte_end":733,"line_start":21,"line_end":21,"column_start":51,"column_end":64,"is_primary":true,"text":[{"text":"fn projection_is_disallowed(x: impl Iterator) -> <impl Iterator>::Item {","highlight_start":51,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0667]: `impl Trait` is not allowed in path parameters\n  --> /checkout/src/test/ui/impl_trait_projections.rs:21:51\n   |\nLL | fn projection_is_disallowed(x: impl Iterator) -> <impl Iterator>::Item {\n   |                          /issue-33525.rs:13:8
[00:45:01] -    |
[00:45:01] - LL |     "".lorem; //~ ERROR no field
[00:45:01] + error: aborting due to previous error
[00:45:01] 12 
[00:45:01] 12 
[00:45:01] - error[E0609]: no field `ipsum` on type `&'static str`
[00:45:01] -   --> $DIR/issue-33525.rs:14:8
[00:45:01] -    |
[00:45:01] - LL |     "".ipsum; //~ ERROR no field
[00:45:01] - 
[00:45:01] - error: aborting due to 3 previous errors
[00:45:01] - 
[00:45:01] - Some errors occurred: E0425, E0609.
[00:45:01] - Some errors occurred: E0425, E0609.
[00:45:01] - For more information about an error, try `rustc --explain E0425`.
[00:45:01] + For more information about this error, try `rustc --explain E0425`.
[00:45:01] 23 
[00:45:01] 
[00:45:01] 
[00:45:01] The actual stderr differed from the expected stderr.
[00:45:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-33525/issue-33525.stderr
[00:45:01] To update references, rerun the tests and pass the `--bless` flag
[00:45:01] To only update this specific test, also pass `--test-args issue-33525.rs`
[00:45:01] error: 1 errors occurred comparing output.
[00:45:01] status: exit code: 101
[00:45:01] status: exit code: 101
[00:45:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-33525.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-33525/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-litart":5,"column_end":6,"is_primary":true,"text":[{"text":"    a; //~ ERROR cannot find value `a`","highlight_start":5,"highlight_end":6}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0425]: cannot find value `a` in this scope\n  --> /checkout/src/test/ui/issue-33525.rs:12:5\n   |\nLL |     a; //~ ERROR cannot find value `a`\n   |     ^ not found in this scope\n\n"}
[00:45:01] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:01] {"message":"For more information about this error, try `rustc --explain E0425`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0425`.\n"}
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] thread '[ui] ui/issue-33525.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:45:01] 
[00:45:01] 
[00:45:01] ---- [ui] ui/issue-50480.rs stdout ----
[00:45:01] diff of stderr:
[00:45:01] 
[00:45:01] 4 LL | struct Foo(NotDefined, <i32 as Iterator>::Item, Vec<i32>, String);
[00:45:01] 6 
[00:45:01] 6 
[00:45:01] - error[E0277]: the trait bound `i32: std::iter::Iterator` is not satisfied
[00:45:01] -   --> $DIR/issue-50480.rs:13:24
[00:45:01] -    |
[00:45:01] - LL | struct Foo(NotDefined, <i32 as Iterator>::Item, Vec<i32>, String);
[00:45:01] -    |                        ^^^^^^^^^^^^^^^^^^^^^^^ `i32` is not an iterator; maybe try calling `.iter()` or a similar method
[00:45:01] -    |
[00:45:01] -    = help: the trait `std::iter::Iterator` is not implemented for `i32`
[00:45:01] + error: aborting due to previous error
[00:45:01] 14 
[00:45:01] - error[E0204]: the trait `Copy` may not be implemented for this type
[00:45:01] -   --> $DIR/issue-50480.rs:11:17
[00:45:01] -    |
[00:45:01] - LL | #[derive(Clone, Copy)]
[00:45:01] -    |                 ^^^^
[00:45:01] - LL | //~^ ERROR the trait `Copy` may not be implemented for this type
[00:45:01] - LL | struct Foo(NotDefined, <i32 as Iterator>::Item, Vec<i32>, String);
[00:45:01] -    |                                                 --------  ------ this field does not implement `Copy`
[00:45:01] -    |                                                 |
[00:45:01] -    |                                                 this field does not implement `Copy`
[00:45:01] - error: aborting due to 3 previous errors
[00:45:01] - 
[00:45:01] - Some errors occurred: E0204, E0277, E0412.
[00:45:01] - For more information about an error, try `rustc --explain E0204`.
[00:45:01] - For more information about an error, try `rustc --explain E0204`.
[00:45:01] + For more information about this error, try `rustc --explain E0412`.
[00:45:01] 30 
[00:45:01] 
[00:45:01] 
[00:45:01] The actual stderr differed from the expected stderr.
[00:45:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-50480/issue-50480.stderr
[00:45:01] To update references, rerun the tests and pass the `--bless` flag
[00:45:01] To only update this specific test, also pass `--test-args issue-50480.rs`
[00:45:01] error: 1 errorsssue-50480.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:45:01] 
[00:45:01] ---- [ui] ui/issue-50585.rs stdout ----
[00:45:01] diff of stderr:
[00:45:01] diff of stderr:
[00:45:01] 
[00:45:01] - error[E0308]: mismatched types
[00:45:01] -   --> $DIR/issue-50585.rs:12:18
[00:45:01] -    |
[00:45:01] - LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
[00:45:01] -    |                  ^^^^^^^^^^^^^^^^ expected usize, found ()
[00:45:01] -    = note: expected type `usize`
[00:45:01] -               found type `()`
[00:45:01] - 
[00:45:01] - error: aborting due to previous error
[00:45:01] - error: aborting due to previous error
[00:45:01] - 
[00:45:01] - For more information about this error, try `rustc --explain E0308`.
[00:45:01] - 
[00:45:01] 
[00:45:01] 
[00:45:01] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-50585/issue-50585.stderr`: No such file or directory (os error 2)
[00:45:01] 
[00:45:01] ---- [ui] ui/issue-51714.rs stdout ----
[00:45:01] diff of stderr:
[00:45:01] 
[00:45:01] 
[00:45:01] 22 LL |     [(); return while let Some(n) = Some(0) {}];
[00:45:01] 24 
[00:45:01] - error: aborting due to 4 previous errors
[00:45:01] - error: aborting due to 4 previous errors
[00:45:01] + error[E0165]: irrefutable while-let pattern
[00:45:01] +   --> $DIR/issue-51714.rs:21:27
[00:45:01] +    |
[00:45:01] + LL |     [(); return while let Some(n) = Some(0) {}];
[00:45:01] +    |                           ^^^^^^^ irrefutable pattern
[00:45:01] 26 
[00:45:01] - For more informap {\n    let Irrefutable(x) = irr;\n    // ...\n}\n