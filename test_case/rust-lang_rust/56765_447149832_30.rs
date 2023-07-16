\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-types/associated-types-multiple-types-one-trait.rs","byte_start":608,"byte_end":614,"line_start":23,"line_end":23,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    want_y(t); //~ ERROR type mismatch","highlight_start":5,"highlight_end":11}],"label":"expected associated type, found i32","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `<T as Foo>::Y`\n   found type `i32`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required by `want_y`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/associated-types/associated-types-multiple-types-one-trait.rs","byte_start":1033,"byte_end":1063,"line_start":54,"line_end":54,"column_start":1,"column_end":31,"is_primary":true,"text":[{"text":"fn want_y<T:Foo<Y=i32>>(t: &T) { }","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0271]: type mismatch resolving `<T as Foo>::Y == i32`\n  --> /checkout/src/test/ui/associated-types/associated-types-multiple-types-one-trait.rs:23:5\n   |\nLL |     want_y(t); //~ ERROR type mismatch\n   |     ^^^^^^ expected associated type, found i32\n   |\n   = note: expected type `<T as Foo>::Y`\n              found type `i32`\nnote: required by `want_y`\n  --> /checkout/src/test/ui/associated-types/associated-types-multiple-types-one-trait.rs:54:1\n   |\nLL | fn want_y<T:Foo<Y=i32>>(t: &T) { }\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:31] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:31] {"message":"For more information about this error, try `rustc --explain E0271`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0271`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/associated-types/associated-types-multiple-types-one-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/associated-types/associated-types-path-2.rs stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] - error[E0308]: mismatched types
[00:49:31] -   --> $DIR/associated-types-path-2.rs:29:14
[00:49:31] -    |
[00:49:31] - LL |     f1(2i32, 4i32);
[00:49:31] - 
[00:49:31] - 
[00:49:31] 7 error[E0277]: the trait bound `u32: Foo` is not satisfied
[00:49:31] 9    |
[00:49:31] 
[00:49:31] 45    |
[00:49:31] 45    |
[00:49:31] 46 LL |     let _: i32 = f2(2i32);
[00:49:31] 47    |                  ^^^^^^^^ expected i32, found u32
[00:49:31] + error[E0308]: mismatched types
[00:49:31] +   --> $DIR/associated-types-path-2.rs:29:14
[00:49:31] +    |
[00:49:31] +    |
[00:49:31] + LL |     f1(2i32, 4i32);
[00:49:31] 48 
[00:49:31] 49 error: aborting due to 6 previous errors
[00:49:31] 50 
[00:49:31] 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-2/associated-types-path-2.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args associated-types/associated-types-path-2.rs`
[00:49:31] error: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-path-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-2/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"the trait bound `u32: Foo` is not satisfied","le input types. In\norder to make this example compile, we need to restrict the generic type we're\naccepting:\n\n