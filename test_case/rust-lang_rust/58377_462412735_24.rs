\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"<::alloc::macros::vec macros>","byte_start":121,"byte_end":124,"line_start":3,"line_end":3,"column_start":7,"column_end":10,"is_primary":true,"text":[{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":7,"highlight_end":10}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/elide-errors-on-mismatched-tuple.rs","byte_start":290,"byte_end":306,"line_start":16,"line_end":16,"column_start":23,"column_end":39,"is_primary":false,"text":[{"text":"    let ts: Vec<&T> = vec![&a, &b, &c];","highlight_start":23,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"vec!","def_site_span":{"file_name":"<::alloc::macros::vec macros>","byte_start":0,"byte_end":242,"line_start":1,"line_end":4,"column_start":1,"column_end":54,"is_primary":false,"text":[{"text":"( $ elem : expr ; $ n : expr ) => (","highlight_start":1,"highlight_end":36},{"text":"$ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (","highlight_start":1,"highlight_end":79},{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":1,"highlight_end":74},{"text":"$ ( $ x : expr , ) * ) => ( vec ! [ $ ( $ x ) , * ] )","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `[&dyn T]`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"all local variables must have a statically known size","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"unsized locals are gated as an unstable feature","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `[&dyn T]` cannot be known at compilation time\n  --> /checkout/src/test/ui/elide-errors-on-mismatched-tuple.rs:16:23\n   |\nLL |     let ts: Vec<&T> = vec![&a, &b, &c];\n   |                       ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `[&dyn T]`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>\n   = note: all local variables must have a statically known size\n   = help: unsized locals are gated as an unstable feature\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:05:24] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:05:24] {"message":"Some errors occurred: E0277, E0308.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0277, E0308.\n"}
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] thread '[ui] ui/elide-errors-on-mismatched-tuple.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:24] thread '[ui] ui/elide-errors-on-mismatched-tuple.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:24] 
[01:05:24] ---- [ui] ui/issues/issue-23589.rs stdout ----
[01:05:24] diff of stderr:
[01:05:24] 
[01:05:24] 8    |               help: use angle brackets instead: `<&str>`
[01:05:24] 10 error[E0308]: mismatched types
[01:05:24] -   --> $DIR/issue-23589.rs:2:29
[01:05:24] +   --> $DIR/issue-23589.rs:2:24
[01:05:24] 12    |
[01:05:24] 12    |
[01:05:24] 13 LL |     let v: Vec(&str) = vec!['1', '2'];
[01:05:24] -    |                             ^^^ expected &str, found char
[01:05:24] +    |                        ^^^^^^^^^^^^^^ expected slice, found array of 2 elements
[01:05:24] -    = note: expected type `&str`
[01:05:24] -               found type `char`
[01:05:24] -               found type `char`
[01:05:24] +    = note: expected type `std::boxed::Box<[&str]>`
[01:05:24] +               found type `std::boxed::Box<[char; 2]>`
[01:05:24] 18 
[01:05:24] 19 error: aborting due to 2 previous errors
[01:05:24] 20 
[01:05:24] 
[01:05:24] 
[01:05:24] 
[01:05:24] The actual stderr differed from the expected stderr.
[01:05:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23589/issue-23589.stderr
[01:05:24] To update references, rerun the tests and pass the `--bless` flag
[01:05:24] To only update this specific test, also pass `--test-args issues/issue-23589.rs`
[01:05:24] error: 1 errors occurred comparing output.
[01:05:24] status: exit code: 1
[01:05:24] status: exit code: 1
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23589.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23589/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23589/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] {"message":"parenthesized type parameters may only be used with a `Fn` trait","code":{"code":"E0214","explanation":"\nA generic type was described using parentheses rather than angle brackets.\nFor example:\n\n