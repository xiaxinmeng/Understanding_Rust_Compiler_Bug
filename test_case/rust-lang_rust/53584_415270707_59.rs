\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-object-vs-lifetime.rs","byte_start":942,"byte_end":961,"line_start":23,"line_end":23,"column_start":12,"column_end":31,"is_primary":true,"text":[{"text":"    let _: S<'static, 'static>;","highlight_start":12,"highlight_end":31}],"label":"expected 1 type argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of type arguments: expected 1, found 0\n  --> /checkout/src/test/ui/traits/trait-object-vs-lifetime.rs:23:12\n   |\nLL |     let _: S<'static, 'static>;\n   |            ^^^^^^^^^^^^^^^^^^^ expected 1 type argument\n\n"}
[00:46:53] {"message":"at least one non-builtin trait is required for an object type","code":{"code":"E0224","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-object-vs-lifetime.rs","byte_start":1114,"byte_end":1123,"line_start":26,"line_end":26,"column_start":14,"column_end":23,"is_primary":true,"text":[{"text":"    let _: S<'static +, 'static>;","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0224]: at least one non-builtin trait is required for an object type\n  --> /checkout/src/test/ui/traits/trait-object-vs-lifetime.rs:26:14\n   |\nLL |     let _: S<'static +, 'static>;\n   |              ^^^^^^^^^\n\n"}
[00:46:53] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:46:53] {"message":"Some errors occurred: E0107, E0224.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0107, E0224.\n"}
[00:46:53] {"message":"For more information about an error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0107`.\n"}
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] thread '[ui] ui/traits/trait-object-vs-lifetime.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:53] 
[00:46:53] 
[00:46:53] ---- [ui] ui/traits/trait-test-2.rs stdout ----
[00:46:53] diff of stderr:
[00:46:53] 
[00:46:53] - error[E0087]: wrong number of type arguments: expected 0, found 1
[00:46:53] + error[E0107]: wrong number of type arguments: expected 0, found 1
[00:46:53] 2   --> $DIR/trait-test-2.rs:18:14
[00:46:53] 3    |
[00:46:53] 4 LL |     10.dup::<i32>(); //~ ERROR wrong number of type arguments: expected 0, found 1
[00:46:53] 5    |              ^^^ unexpected type argument
[00:46:53] 6 
[00:46:53] - error[E0087]: wrong number of type arguments: expected 1, found 2
[00:46:53] + error[E0107]: wrong number of type arguments: expected 1, found 2
[00:46:53] + error[E0107]: wrong number of type arguments: expected 1, found 2
[00:46:53] 8   --> $DIR/trait-test-2.rs:19:20
[00:46:53] 9    |
[00:46:53] 10 LL |     10.blah::<i32, i32>(); //~ ERROR wrong number of type arguments: expected 1, found 2
[00:46:53] 37 
[00:46:53] 38 error: aborting due to 5 previous errors
[00:46:53] 39 
[00:46:53] - Some errors occurred: E0038, E0087, E0277.
[00:46:53] - Some errors occurred: E0038, E0087, E0277.
[00:46:53] + Some errors occurred: E0038, E0107, E0277.
[00:46:53] 41 For more information about an error, try `rustc --explain E0038`.
[00:46:53] 42 
[00:46:53] 
[00:46:53] 
[00:46:53] The actual stderr differed from the expected stderr.
[00:46:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-test-2/trait-test-2.stderr
[00:46:53] To update references, rerun the tests and pass the `--bless` flag
[00:46:53] To only update this specific test, also pass `--test-args traits/trait-test-2.rs`
[00:46:53] error: 1 errors occurred comparing output.
[00:46:53] status: exit code: 1
[00:46:53] status: exit code: 1
[00:46:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-test-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-test-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-test-2/auxiliary" "-A" "unused"
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] ------------------------------------------
[00:46:53] stderr:
[00:46:53] stderr:
[00:46:53] ------------------------------------------
[00:46:53] {"message":"wrong number of type arguments: expected 0, found 1","code":{"code":"E0107","explanation":"\nThisnd 1\n    foo::<f64, f64, i32>(x); // error: wrong number of type arguments:\n                             //        expected 2, found 3\n}\n