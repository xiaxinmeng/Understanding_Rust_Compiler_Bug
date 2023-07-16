plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:43] 
[00:43:43] running 1540 tests
[00:43:49] .................F..............................................................................Fi..
[00:43:55] .............................................................i...........FF.................F.......
[00:43:59] ........F.............................................F.F...........................................
[00:44:02] ....................................................................................................
[00:44:06] ................................................................................F...................
[00:44:09] ....................................................................................................
[00:44:14] .................................................F................F.........................F.......
[00:44:19] ....................................................................................................
[00:44:25] ...F................................................................F...F...............F...........
[00:44:37] .F...........................i......................................................................
[00:44:42] ...............i....................................................................................
[00:44:42] ...............i....................................................................................
[00:44:47] .............F......F...............................................................................
[00:44:52] ......................F.................F...........................................................
[00:45:00] ...............................................F.......i............................................
[00:45:01] The actual stderr differed from the expected stderr.
[00:45:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/async-fn-multiple-lifetimes.stderr
[00:45:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/async-fn-multiple-lifetimes.stderr
[00:45:01] To update references, rerun the tests and pass the `--bless` flag
[00:45:01] To only update this specific test, also pass `--test-args async-fn-multiple-lifetimes.rs`
[00:45:01] error: 1 errors occurred comparing output.
[00:45:01] status: exit code: 101
[00:45:01] status: exit code: 101
[00:45:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-fn-multiple-lifetimes.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/auxiliary" "-A" "unused"
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] ------------------------------------------
[00:45:01] stderr:
[00:45:01] stderr:
[00:45:01] ------------------------------------------
[00:45:01] {"message":"multiple different lifetimes used in arguments of `async fn`","code":{"code":"E0709","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":629,"byte_end":631,"line_start":17,"line_end":17,"column_start":47,"column_end":49,"is_primary":false,"text":[{"text":"a\n   |\n   = help: `async fn` can only accept borrowed values with identical lifetimes\n\n"}
[00:45:01] {"message":"multiple elided lifetimes used in arguments of `async fn`","code":{"code":"E0707","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":905,"byte_end":905,"line_start":26,"line_end":26,"column_start":39,"column_end":39,"is_primary":false,"text":[{"text":"async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}","highlight_start":39,"highlight_end":39}],"label":"first lifetime here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":913,"byte_end":913,"line_start":26,"line_end":26,"column_start":47,"column_end":47,"is_primary":false,"text":[{"text":"async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}","highlight_start":47,"highlight_end":47}],"label":"different lifetime here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":905,"byte_end":913,"line_start":26,"line_end":26,"column_start":39,"column_end":47,"is_primary":true,"text":[{"text":"async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}","highlight_start":39,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider giving these arguments named lifetimes","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0707]: multiple elided lifetimes used in ais_primary":true,"text":[{"text":"\tbar; //~ ERROR cannot find value `bar`","highlight_start":2,"highlight_end":5}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0425]: cannot find value `bar` in this scope\n  --> /checkout/src/test/ui/codemap_tests/tab.rs:14:2\n   |\nLL |     bar; //~ ERROR cannot find value `bar`\n   |     ^^^ not found in this scope\n\n"}
[00:45:01] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:01] {"message":"For more information about this error, try `rustc --explain E0425`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0425`.\n"}
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] thread '[ui] ui/codemap_tests/tab.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:45:01] 
[00:45:01] 
[00:45:01] ---- [ui] ui/did_you_mean/bad-assoc-pat.rs stdout ----
[00:45:01] diff of stderr:
[00:45:01] 
[00:45:01] 22 LL |         &(u8,)::AssocItem => {}
[00:45:01] 23    |          ^^^^^^^^^^^^^^^^ help: try: `<(u8,)>::AssocItem`
[00:45:01] 24 
[00:45:01] - error[E0599]: no associated item named `AssocItem` found for type `[u8]` in the current scope
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL |         [u8]::AssocItem => {}
[00:45:01] -    |         ^^^^^^^^^^^^^^^ associated item not found in `[u8]`
[00:45:01] + error: aborting d1] status: exit code: 101
[00:45:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-pat/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-pat/auxiliary" "-A" "unused"
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] ------------------------------------------
[00:45:01] stderr:
[00:45:01] stderr:
[00:45:01] ------------------------------------------
[00:45:01] {"message":"missing angle brackets in associated item path","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs","byte_start":503,"byte_end":518,"line_start":13,"line_end":13,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"        [u8]::AssocItem => {}","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs","byte_start":503,"byte_end":518,"line_start":13,"line_end":13,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"        [u8]::AssocItem => {}","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":"<[u8]>::Assocerror","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs","byte_start":899,"byte_end":911,"line_start":19,"line_end":19,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        _::AssocItem => {}","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs","byte_start":899,"byte_end":911,"line_start":19,"line_end":19,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        _::AssocItem => {}","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":"<_>::AssocItem","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: missing angle brackets in associated item path\n  --> /checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs:19:9\n   |\nLL |         _::AssocItem => {}\n   |         ^^^^^^^^^^^^ help: try: `<_>::AssocItem`\n\n"}
[00:45:01] {"message":"missing angle brackets in associated item path","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs","byte_start":1112,"byte_end":1128,"line_start":24,"line_end":24,"column_start":10,"column_end":26,"is_primary":true,"text":[{"text":"        &(u8,)::AssocItem => {}","highlight_start":10,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"/checkout/srocTy`
[00:45:01] + error: aborting due to 7 previous errors
[00:45:01] 50 
[00:45:01] - error[E0223]: ambiguous associated type
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL | type B = [u8]::AssocTy;
[00:45:01] -    |          ^^^^^^^^^^^^^ ambiguous associated type
[00:45:01] -    |
[00:45:01] -    = note: specify the type using the syntax `<[u8] as Trait>::AssocTy`
[00:45:01] - 
[00:45:01] - error[E0223]: ambiguous associated type
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL | type C = (u8)::AssocTy;
[00:45:01] -    |          ^^^^^^^^^^^^^ ambiguous associated type
[00:45:01] -    |
[00:45:01] -    = note: specify the type using the syntax `<u8 as Trait>::AssocTy`
[00:45:01] - 
[00:45:01] - error[E0223]: ambiguous associated type
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL | type D = (u8, u8)::AssocTy;
[00:45:01] -    |          ^^^^^^^^^^^^^^^^^ ambiguous associated type
[00:45:01] -    |
[00:45:01] -    = note: specify the type using the syntax `<(u8, u8) as Trait>::AssocTy`
[00:45:01] - 
[00:45:01] - error[E0121]: the type placeholder `_` is not allowed within types on item signatures
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL | type E = _::AssocTy;
[00:45:01] -    |          ^ not allowed in type signatures
[00:45:01] - 
[00:45:01] - error[E0223]: ambiguous associated type
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL | type F = &'static (u8)::AssocTy;
[00:45:01] -    |                   ^^^^^^^^^^^^^ ambiguous associated type
[00:45:01] -    |
[00:45:01] -    = note: specify the type using the syntax `<u8 as Trait>::AssocTy`
[00:45:01] - 
[00:45:01] - error[E0223]: ambiguous associated type
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL | type G = 'static + (Send)::AssocTy;
[00:45:01] -    |          ^^^^^^^^^^^^^^^^^^^^^^^^^ ambiguous associated type
[00:45:01] -    |
[00:45:01] -    = note: specify the type using the syntax `<(dyn std::marker::Send + 'static) as Trait>::AssocTy`
[00:45:01] - 
[00:45:01] - error[E0223]: ambiguous associated type
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL | type H = Fn(u8) -> (u8)::Output;
[00:45:01] -    |          ^^^^^^^^^^^^^^^^^^^^^^ ambiguous associated type
[00:45:01] -    |
[00:45:01] -    = note: specify the type using the syntax `<(dyn std::ops::Fn(u8) -> u8 + 'static) as Trait>::Output`
[00:45:01] - error: aborting due to 15 previous errors
[00:45:01] - 
[00:45:01] - Some errors occurred: E0121, E0223.
[00:45:01] - For more information about an error, try `rustc --explain E0121`.
[00:45:01] - For more information about an error, try `rustc --explain E0121`.
[00:45:01] 109 
[00:45:01] 
[00:45:01] 
[00:45:01] The actual stderr differed from the expected stderr.
[00:45:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-ty/bad-assoc-ty.stderr
[00:45:01] To update references, rerun the tests and pass the `--bless` flag
[00:45:01] To only update this specific test, also pass `--test-args did_you_mean/bad-assoc-ty.rs`
[00:45:01] error: 1 errors occurred comparing output.
[00:45:01] error: 1 errors occurred comparing output.
[00:45:01] status: expplicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: missing angle brackets in associated item path\n  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:11:10\n   |\nLL | type A = [u8; 4]::AssocTy;\n   |          ^^^^^^^^^^^^^^^^ help: try: `<[u8; 4]>::AssocTy`\n\n"}
[00:45:01] {"message":"missing angle brackets in associated item path","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs","byte_start":599,"byte_end":612,"line_start":15,"line_end":15,"column_start":10,"column_end":23,"is_primary":true,"text":[{"text":"type B = [u8]::AssocTy;","highlight_start":10,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs","byte_start":599,"byte_end":612,"line_start":15,"line_end":15,"column_start":10,"column_end":23,"is_primary":true,"text":[{"text":"type B = [u8]::AssocTy;","highlight_start":10,"highlight_end":23}],"label":null,"suggested_replacement":"<[u8]>::AssocTy","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: missing angle brackets in associated item path\n  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:15:10\n   |\nLL | type B = [u8]::AssocTy;\n   |          ^^^^^^^^^^^^^ help: try: `<[u8]>::AssocTy`\n\n"}
[00:45:01] {"message":"missing angle brackets in associated item path","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_meanhlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs","byte_start":1366,"byte_end":1391,"line_start":37,"line_end":37,"column_start":10,"column_end":35,"is_primary":true,"text":[{"text":"type G = 'static + (Send)::AssocTy;","highlight_start":10,"highlight_end":35}],"label":null,"suggested_replacement":"<'static + Send>::AssocTy","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: missing angle brackets in associated item path\n  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:37:10\n   |\nLL | type G = 'static + (Send)::AssocTy;\n   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `<'static + Send>::AssocTy`\n\n"}
[00:45:01] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] thread '[ui] ui/did_you_mean/bad-assoc-ty.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:45:01] 
[00:45:01] 
[00:45:01] ---- [ui] ui/did_you_mean/issue-40006.rs stdout ----
[00:45:01] diff of stderr:
[00:45:01] 
[00:45:01] 56 LL |     pub hello_method(&self) { //~ ERROR missing
[00:45:01] 57    |        ^ missing `fn`, `type`, or `const`
[00:45:01] 58 
[00:45:01] - error[E0038]: the trait `X` cannot be made into an object
[00:45:01] -   --> $DIR/issue-40006.rs:11:6
[00:45:01] - 45:01] stderr:
[00:45:01] ------------------------------------------
[00:45:01] {"message":"missing `fn`, `type`, or `const` for impl-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":475,"byte_end":539,"line_start":11,"line_end":13,"column_start":9,"column_end":5,"is_primary":true,"text":[{"text":"impl X { //~ ERROR cannot be made into an object","highlight_start":9,"highlight_end":49},{"text":"//~^ ERROR missing","highlight_start":1,"highlight_end":19},{"text":"    Y","highlight_start":1,"highlight_end":5}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for impl-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:11:9\n   |\nLL |   impl X { //~ ERROR cannot be made into an object\n   |  _________^\nLL | | //~^ ERROR missing\nLL | |     Y\n   | |____^ missing `fn`, `type`, or `const`\n\n"}
[00:45:01] {"message":"missing `fn`, `type`, or `const` for trait-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":564,"byte_end":587,"line_start":18,"line_end":19,"column_start":10,"column_end":5,"is_primary":true,"text":[{"text":"trait X { //~ ERROR missing","highlight_start":10,"highlight_end":28},{"text":"    X() {}","highlight_start":1,"highlight_end":5}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error`fn`, `type`, or `const` for trait-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:20:21\n   |\nLL |       fn xxx() { ### } //~ ERROR missing\n   |  _____________________^\nLL | |     //~^ ERROR expected\nLL | |     L = M; //~ ERROR missing\n   | |____^ missing `fn`, `type`, or `const`\n\n"}
[00:45:01] {"message":"missing `fn`, `type`, or `const` for trait-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":667,"byte_end":690,"line_start":22,"line_end":23,"column_start":11,"column_end":5,"is_primary":true,"text":[{"text":"    L = M; //~ ERROR missing","highlight_start":11,"highlight_end":29},{"text":"    Z = { 2 + 3 }; //~ ERROR expected one of","highlight_start":1,"highlight_end":5}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for trait-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:22:11\n   |\nLL |       L = M; //~ ERROR missing\n   |  ___________^\nLL | |     Z = { 2 + 3 }; //~ ERROR expected one of\n   | |____^ missing `fn`, `type`, or `const`\n\n"}
[00:45:01] {"message":"expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `;`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":703,"byte_end":704,"line_start":23,"line_end":23,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"    Z = { 2 + 3 }; //~ ERROR expected one of","highlight_stauggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for impl-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:28:8\n   |\nLL |     pub hello_method(&self) { //~ ERROR missing\n   |        ^ missing `fn`, `type`, or `const`\n\n"}
[00:45:01] {"message":"aborting due to 8 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 8 previous errors\n\n"}
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] thread '[ui] ui/did_you_mean/issue-40006.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:45:01] 
[00:45:01] 
[00:45:01] ---- [ui] ui/did_you_mean/trait-object-reference-without-parens-suggestion.rs stdout ----
[00:45:01] diff of stderr:
[00:45:01] 
[00:45:01] 10 LL |     let _: &'static Copy + 'static; //~ ERROR expected a path
[00:45:01] 11    |            ^^^^^^^^^^^^^^^^^^^^^^^ help: try adding parentheses: `&'static (Copy + 'static)`
[00:45:01] 12 
[00:45:01] - error[E0038]: the trait `std::marker::Copy` cannot be made into an object
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL |     let _: &Copy + 'static; //~ ERROR expected a path
[00:45:01] -    |            ^^^^^ the trait `std::marker::Copy` cannot be made into an object
[00:45:01] -    |
[00:45:01] -    = note: the trait cannot require that `Self : Sized`
[00:45:01] + error: aborting due to 2 previous errors
[00:45:01] - error: aborting due to 3 previous errors
[00:45:01] - 
[00:45:01] - Some errors occurred: E0038, E0178.
[00:45:01] - For more information about an error, try `rustc --explain E0038`.
[00:45:01] - For more information about an error, try `rustc --explain E0038`.
[00:45:01] + For more information about this error, try `rustc --explain E0178`.
[00:45:01] 25 
[00:45:01] 
[00:45:01] 
[00:45:01] The actual stderr differed from the expected stderr.
[00:45:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/trait-object-reference-without-parens-suggestion/trait-object-reference-without-parens-suggestion.stderr
[00:45:01] To update references, rerun the tests and pass the `--bless` flag
[00:45:01] To only update this specific test, also pass `--test-args did_you_mean/trait-object-reference-without-parens-suggestion.rs`
[00:45:01] error: 1 errors occurred comparing output.
[00:45:01] status: exit code: 101
[00:45:01] status: exit code: 101
[00:45:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/trait-object-reference-without-parens-suggestion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/trait-object-reference-without-parens-suggestion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/trait-object-reference-without-parens-suggestion/auxiliary" "-A" "unused"
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] 
[00:45:01] -------------------------ggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0178]: expected a path on the left-hand side of `+`, not `&Copy`\n  --> /checkout/src/test/ui/did_you_mean/trait-object-reference-without-parens-suggestion.rs:12:12\n   |\nLL |     let _: &Copy + 'static; //~ ERROR expected a path\n   |            ^^^^^^^^^^^^^^^ help: try adding parentheses: `&(Copy + 'static)`\n\n"}
[00:45:01] {"message":"expected a path on the left-hand side of `+`, not `&'static Copy`","code":{"code":"E0178","explanation":"\nIn types, the `+` type operator has low precedence, so it is often necessary\nto use parentheses.\n\nFor example:\n\n