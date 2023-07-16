\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/hrtb/hrtb-higher-ranker-supertraits.rs","byte_start":770,"byte_end":790,"line_start":28,"line_end":28,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    want_foo_f/hrtb/hrtb-higher-ranker-supertraits.rs:31:1\n   |\nLL | / fn want_foo_for_any_tcx<F>(f: &F)\nLL | |     where F : for<'tcx> Foo<'tcx>\nLL | | {\nLL | |     want_foo_for_some_tcx(f);\nLL | |     want_foo_for_any_tcx(f);\nLL | | }\n   | |_^\n\n"}
[01:00:57] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:00:57] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[01:00:57] ------------------------------------------
[01:00:57] 
[01:00:57] thread '[ui] ui/hrtb/hrtb-higher-ranker-supertraits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:00:57] 
[01:00:57] 
[01:00:57] ---- [ui] ui/immut-function-arguments.rs#ast stdout ----
[01:00:57] diff of stderr:
[01:00:57] 
[01:00:57] - error[E0594]: cannot assign to immutable `Box` content `*y`
[01:00:57] -   --> $DIR/immut-function-arguments.rs:15:5
[01:00:57] -    |
[01:00:57] - LL | fn f(y: Box<isize>) {
[01:00:57] -    |      - help: make this binding mutable: `mut y`
[01:00:57] - LL |     *y = 5; //[ast]~ ERROR cannot assign
[01:00:57] -    |     ^^^^^^ cannot borrow as mutable
[01:00:57] - 
[01:00:57] 9 error[E0594]: cannot assign to immutable `Box` content `*q`
[01:00:57] 10   --> $DIR/immut-function-arguments.rs:20:35
[01:00:57] 
[01:00:57] 13    |                  -                ^^^^^^ cannot borrow as mutable
[01:00:57] 13    |                  -                ^^^^^^ cannot borrow as mutable
[01:00:57] 14    |    lifetime mismatch","highlight_start":24,"highlight_end":31}],"label":"this parameter and the return type are declared with different lifetimes...","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/in-band-lifetimes/mismatched.rs","byte_start":637,"byte_end":644,"line_start":16,"line_end":16,"column_start":36,"column_end":43,"is_primary":false,"text":[{"text":"fn foo2(x: &'a u32, y: &'b u32) -> &'a u32 { y } //~ ERROR lifetime mismatch","highlight_start":36,"highlight_end":43}],"label":"","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/in-band-lifetimes/mismatched.rs","byte_start":647,"byte_end":648,"line_start":16,"line_end":16,"column_start":46,"column_end":47,"is_primary":true,"text":[{"text":"fn foo2(x: &'a u32, y: &'b u32) -> &'a u32 { y } //~ ERROR lifetime mismatch","highlight_start":46,"highlight_end":47}],"label":"...but data from `y` is returned here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0623]: lifetime mismatch\n  --> /checkout/src/test/ui/in-band-lifetimes/mismatched.rs:16:46\n   |\nLL | fn foo2(x: &'a u32, y: &'b u32) -> &'a u32 { y } //~ ERROR lifetime mismatch\n   |                        -------     -------   ^ ...but data from `y` is returned here\n   |                        |\n   |                        this parameter and the return type are declared with different lifetimes...\n\n"}
[01:00:57] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"X {\n    Entry,\n}\n\nX::Entry(); // error: expected function, found `X::Entry`\n\n// Or even simpler:\nlet x = 0i32;\nx(); // error: expected function, found `i32`\n