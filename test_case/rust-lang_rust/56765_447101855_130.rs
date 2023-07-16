\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/yield-while-iterating.rs","byte_start":1798,"byte_end":1799,"line_start":67,"line_end":67,"column_start":20,"column_end":21,"is_primary":true,"text":[{"text":"    println!(\"{}\", x[0]); //~ ERROR","highlight_start":20,"highlight_end":21}],"label":"immutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/yield-while-iterating.rs","byte_start":1710,"byte_end":1712,"line_start":62,"line_end":62,"column_start":17,"column_end":19,"is_primary":false,"text":[{"text":"    let mut b = || {","highlight_start":17,"highlight_end":19}],"label":"mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/yieo 2 previous errors\n\n"}
[01:00:57] {"message":"Some errors occurred: E0502, E0626.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0502, E0626.\n"}
[01:00:57] {"message":"For more information about an error, try `rustc --explain E0502`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0502`.\n"}
[01:00:57] ------------------------------------------
[01:00:57] 
[01:00:57] thread '[ui] ui/generator/yield-while-iterating.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:00:57] 
[01:00:57] 
[01:00:57] ---- [ui] ui/hrtb/hrtb-higher-ranker-supertraits.rs stdout ----
[01:00:57] diff of stderr:
[01:00:57] 
[01:00:57] - error[E0277]: the trait bound `for<'tcx> F: Foo<'tcx>` is not satisfied
[01:00:57] -   --> $DIR/hrtb-higher-ranker-supertraits.rs:28:5
[01:00:57] -    |
[01:00:57] - LL |     want_foo_for_any_tcx(f); //~ ERROR E0277
[01:00:57] -    |     ^^^^^^^^^^^^^^^^^^^^ the trait `for<'tcx> Foo<'tcx>` is not implemented for `F`
[01:00:57] -    |
[01:00:57] -    = help: consider adding a `where for<'tcx> F: Foo<'tcx>` bound
[01:00:57] - note: required by `want_foo_for_any_tcx`
[01:00:57] -   --> $DIR/hrtb-higher-ranker-supertraits.rs:31:1
[01:00:57] -    |
[01:00:57] - LL | / fn want_foo_for_any_tcx<F>(f: &F)
[01:00:57] - LL | |     where F : for<'tcx> Foo<'tcx>
[01:00:57] - LL | | {
[01:00:57] - LL | |     want_foo_for_some_tcx(f);
[01:00:57] - LL | |     want_foo_for_any_tcx(f);
[01:00:57] - LL | | }
[01:00:57] - 
[01:00:57] - 
[01:00:57] 19 error[E0277]: the trait bound `for<'ccx> B: Bar<'ccx>` is not satisfied
[01:00:57] 20   --> $DIR/hrtb-higher-ranker-supertraits.rs:45:5
[01:00:57] 
[01:00:57] 
[01:00:57] 32 LL | |     want_foo_for_some_tcx(b);
[01:00:57] 33 ...  |
[01:00:57] 34 LL | |     want_bar_for_any_ccx(b);
[01:00:57] + LL | | }
[01:00:57] + 
[01:00:57] + 
[01:00:57] + error[E0277]: the trait bound `for<'tcx> F: Foo<'tcx>` is not satisfied
[01:00:57] +   --> $DIR/hrtb-higher-ranker-supertraits.rs:28:5
[01:00:57] +    |
[01:00:57] + LL |     want_foo_for_any_tcx(f); //~ ERROR E0277
[01:00:57] +    |     ^^^^^^^^^^^^^^^^^^^^ the trait `for<'tcx> Foo<'tcx>` is not implemented for `F`
[01:00:57] +    |
[01:00:57] +    = help: consider adding a `where for<'tcx> F: Foo<'tcx>` bound
[01:00:57] + note: required by `want_foo_for_any_tcx`
[01:00:57] +   --> $DIR/hrtb-higher-ranker-supertraits.rs:31:1
[01:00:57] +    |
[01:00:57] + LL | / fn want_foo_for_any_tcx<F>(f: &F)
[01:00:57] + LL | |     where F : for<'tcx> Foo<'tcx>
[01:00:57] + LL | | {
[01:00:57] + LL | |     want_foo_for_some_tcx(f);
[01:00:57] + LL | |     want_foo_for_any_tcx(f);
[01:00:57] 35 LL | | }
[01:00:57] 37 
[01:00:57] 
[01:00:57] 
[01:00:57] The actual stderr differed from the expected stderr.
[01:00:57] The actual stderr differed from the expected stderr.
[01:00:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-higher-ranker-supertraits/hrtb-higher-ranker-supertraits.stderr
[01:00:57] To update references, rerun the tests and pass the `--bless` flag
[01:00:57] To only update this specific test, also pass `--test-args hrtb/hrtb-higher-ranker-supertraits.rsile_name":"/checkout/src/test/ui/hrtb/hrtb-higher-ranker-supertraits.rs","byte_start":1102,"byte_end":1122,"line_start":45,"line_end":45,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    want_bar_for_any_ccx(b); //~ ERROR E0277","highlight_start":5,"highlight_end":25}],"label":"the trait `for<'ccx> Bar<'ccx>` is not implemented for `B`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider adding a `where for<'ccx> B: Bar<'ccx>` bound","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"required by `want_bar_for_any_ccx`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/hrtb/hrtb-higher-ranker-supertraits.rs","byte_start":1146,"byte_end":1336,"line_start":48,"line_end":56,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn want_bar_for_any_ccx<B>(b: &B)","highlight_start":1,"highlight_end":34},{"text":"    where B : for<'ccx> Bar<'ccx>","highlight_start":1,"highlight_end":34},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    want_foo_for_some_tcx(b);","highlight_start":1,"highlight_end":30},{"text":"    want_foo_for_any_tcx(b);","highlight_start":1,"highlight_end":29},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    want_bar_for_some_ccx(b);","highlight_start":1,"highlight_end":30},{"text":"    want_bar_for_any_ccx(b);","highlight_start":1,"highlight_end":29},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `for<'ccx> B: Bar<'ccx>` is not satisfied\n  --> /checkout/src/test/ui/hrtb/hrtb-higher-ranker-supertraits.rs:45:5\n   |\nLL |     want_bar_for_any_ccx(b); //~ ERROR E0277\n   |     ^^^^^^^^^^^^^^^^^^^^ the trait `for<'ccx> Bar<'ccx>` is not implemented for `B`\n   |\n   = help: consider adding a `where for<'ccx> B: Bar<'ccx>` bound\nnote: required by `want_bar_for_any_ccx`\n  --> /checkout/src/test/ui/hrtb/hrtb-higher-ranker-supertraits.rs:48:1\n   |\nLL | / fn want_bar_for_any_ccx<B>(b: &B)\nLL | |     where B : for<'ccx> Bar<'ccx>\nLL | | {\nLL | |     want_foo_for_some_tcx(b);\n...  |\nLL | |     want_bar_for_any_ccx(b);\nLL | | }\n   | |_^\n\n"}
[01:00:57] {"message":"the trait bound `for<'tcx> F: Foo<'tcx>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n