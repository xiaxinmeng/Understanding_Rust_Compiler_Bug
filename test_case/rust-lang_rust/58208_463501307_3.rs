\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/hygiene/no_implicit_prelude.rs","byte_start":223,"byte_end":228,"line_start":12,"line_end":12,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"        ().clone() //~ ERROR no method named `clone` found","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/hygiene/no_implicit_prelude.rs","byte_start":93,"byte_end":105,"line_start":5,"line_end":5,"column_start":14,"column_end":26,"is_primary":false,"text":[{"text":"    fn f() { ::bar::m!(); }","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"::bar::m!","def_site_span":{"file_name":"/checkout/src/test/ui/hygiene/no_implicit_prelude.rs","byte_start":148,"byte_end":276,"line_start":10,"line_end":13,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    pub macro m() {","highlight_start":5,"highlight_end":20},{"text":"        Vec::new(); //~ ERROR failed to resolve","highlight_start":1,"highlight_end":48},{"text":"        ().clone() //~ ERROR no method named `clone` found","highlight_start":1,"highlight_end":59},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"items from traits can only be used if the trait is in scope","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"the following trait is implemented but not in scope, perhaps add a `use` for it:\n`use std::clone::Clone;`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0599]: no method named `clone` found for type `()` in the current scope\n  --> /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:12:12\n   |\nLL |     fn f() { ::bar::m!(); }\n   |              ------------ in this macro invocation\n...\nLL |         ().clone() //~ ERROR no method named `clone` found\n   |            ^^^^^\n   |\n   = help: items from traits can only be used if the trait is in scope\n   = note: the following trait is implemented but not in scope, perhaps add a `use` for it:\n           `use std::clone::Clone;`\n\n"}
[01:02:26] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:02:26] {"message":"Some errors occurred: E0433, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0433, E0599.\n"}
[01:02:26] 
[01:02:26] ------------------------------------------
[01:02:26] 
[01:02:26] thread '[ui] ui/hygiene/no_implicit_prelude.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:26] thread '[ui] ui/hygiene/no_implicit_prelude.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:26] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:02:26] 
[01:02:26] ---- [ui] ui/tag-that-dare-not-speak-its-name.rs stdout ----
[01:02:26] diff of stderr:
[01:02:26] 
[01:02:26] + error: cannot find macro `panic!` in this scope
[01:02:26] +    |
[01:02:26] + LL |     panic!();
[01:02:26] +    |     ^^^^^^^^^
[01:02:26] +    |
[01:02:26] +    |
[01:02:26] +    = help: have you added the `#[macro_use]` on the module/import?
[01:02:26] + 
[01:02:26] 1 error[E0308]: mismatched types
[01:02:26] +   --> $DIR/tag-that-dare-not-speak-its-name.rs:6:28
[01:02:26] +    |
[01:02:26] +    |
[01:02:26] + LL | fn last<T>(v: Vec<&T> ) -> std::option::Option<T> {
[01:02:26] +    |    ----                    ^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found ()
[01:02:26] +    |    |
[01:02:26] +    |    this function's body doesn't return
[01:02:26] + LL |     panic!();
[01:02:26] +    |             - help: consider removing this semicolon
[01:02:26] +    = note: expected type `std::option::Option<T>`
[01:02:26] +               found type `()`
[01:02:26] + 
[01:02:26] + error[E0308]: mismatched types
[01:02:26] + error[E0308]: mismatched types
[01:02:26] 2   --> $DIR/tag-that-dare-not-speak-its-name.rs:12:20
[01:02:26] 3    |
[01:02:26] 4 LL |     let x : char = last(y);
[01:02:26] 7    = note: expected type `char`
[01:02:26] 8               found type `std::option::Option<_>`
[01:02:26] 9 
[01:02:26] - error: aborting due to previous error
[01:02:26] - error: aborting due to previous error
[01:02:26] + error: aborting due to 3 previous errors
[01:02:26] 11 
[01:02:26] 12 For more information about this error, try `rustc --explain E0308`.
[01:02:26] 13 
[01:02:26] 
[01:02:26] 
[01:02:26] The actual stderr differed from the expected stderr.
[01:02:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-that-dare-not-speak-its-name/tag-that-dare-not-speak-its-name.stderr
[01:02:26] To update references, rerun the tests and pass the `--bless` flag
[01:02:26] To only update this specific test, also pass `--test-args tag-that-dare-not-speak-its-name.rs`
[01:02:26] error: 1 errors occurred comparing output.
[01:02:26] status: exit code: 1
[01:02:26] status: exit code: 1
[01:02:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-that-dare-not-speak-its-name/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-that-dare-not-speak-its-name/auxiliary" "-A" "unused"
[01:02:26] ------------------------------------------
[01:02:26] 
[01:02:26] ------------------------------------------
[01:02:26] stderr:
[01:02:26] stderr:
[01:02:26] ------------------------------------------
[01:02:26] {"message":"cannot find macro `panic!` in this scope","code":null,"level":"error","spans":[{"file_name":"<::std::macros::panic macros>","byte_start":12,"byte_end":17,"line_start":1,"line_end":1,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"(  ) => ( { panic ! ( \"explicit panic\" ) } ) ; ( $ msg : expr ) => (","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs","byte_start":115,"byte_end":124,"line_start":7,"line_end":7,"column_start":5,"column_end":14,"is_primary":false,"text":[{"text":"    panic!();","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"<::std::macros::panic macros>","byte_start":0,"byte_end":427,"line_start":1,"line_end":10,"column_start":1,"column_end":68,"is_primary":false,"text":[{"text":"(  ) => ( { panic ! ( \"explicit panic\" ) } ) ; ( $ msg : expr ) => (","highlight_start":1,"highlight_end":69},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"$ crate :: rt :: begin_panic (","highlight_start":1,"highlight_end":31},{"text":"$ msg , & ( file ! (  ) , line ! (  ) , __rust_unstable_column ! (  ) ) ) } )","highlight_start":1,"highlight_end":78},{"text":"; ( $ msg : expr , ) => ( { panic ! ( $ msg ) } ) ; (","highlight_start":1,"highlight_end":54},{"text":"$ fmt : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":41},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"$ crate :: rt :: begin_panic_fmt (","highlight_start":1,"highlight_end":35},{"text":"& format_args ! ( $ fmt , $ ( $ arg ) + ) , & (","highlight_start":1,"highlight_end":48},{"text":"file ! (  ) , line ! (  ) , __rust_unstable_column ! (  ) ) ) } ) ;","highlight_start":1,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"have you added the `#[macro_use]` on the module/import?","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `panic!` in this scope\n  --> /checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs:7:5\n   |\nLL |     panic!();\n   |     ^^^^^^^^^\n   |\n   = help: have you added the `#[macro_use]` on the module/import?\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:02:26] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n