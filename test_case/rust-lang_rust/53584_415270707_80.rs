\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/typeck/typeck-builtin-bound-type-parameters.rs","byte_start":907,"byte_end":908,"line_start":24,"line_end":24,"column_start":24,"column_end":25,"is_primary":true,"text":[{"text":"fn foo2<'a, T:Copy<'a, U>, U>(x: T) {}","highlight_start":24,"highlight_end":25}],"label":"unexpected type argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of type arguments: expected 0, found 1\n  --> /checkout/src/test/ui/typeck/typeck-builtin-bound-type-parameters.rs:24:24\n   |\nLL | fn foo2<'a, T:Copy<'a, U>, U>(x: T) {}\|     let c: Foo<_, _> = Foo { r: &5 };\n   |                   ^ unexpected type argument\n\n"}
[00:46:53] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:53] {"message":"For more information about this error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0107`.\n"}
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] thread '[ui] ui/typeck/typeck_type_placeholder_lifetime_1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:53] 
[00:46:53] 
[00:46:53] ---- [ui] ui/typeck/typeck_type_placeholder_lifetime_2.rs stdout ----
[00:46:53] diff of stderr:
[00:46:53] 
[00:46:53] - error[E0244]: wrong number of type arguments: expected 1, found 2
[00:46:53] + error[E0107]: wrong number of type arguments: expected 1, found 2
[00:46:53] 2   --> $DIR/typeck_type_placeholder_lifetime_2.rs:19:19
[00:46:53] 3    |
[00:46:53] 4 LL |     let c: Foo<_, usize> = Foo { r: &5 };
[00:46:53] 6 
[00:46:53] 7 error: aborting due to previous error
[00:46:53] 8 
[00:46:53] - For more information about this error, try `rustc --explain E0244`.
[00:46:53] - For more information about this error, try `rustc --explain E0244`.
[00:46:53] + For more information about this error, try `rustc --explain E0107`.
[00:46:53] 10 
[00:46:53] 
[00:46:53] 
[00:46:53] The actual stderr differed from the expected stderr.
[00:46:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_lifetime_2/          //        expected 0, found 1\n}\n