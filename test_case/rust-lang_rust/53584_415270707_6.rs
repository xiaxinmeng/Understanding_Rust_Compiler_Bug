\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/constructor-lifetime-args.rs","byte_start":1432,"byte_end":1439,"line_start":34,"line_end":34,"column_start":30,"column_end":37,"is_primary":true,"text":[{"text":"    E::V::<'static, 'static, 'static>(&0);","highlight_start":30,"highlight_end":37}],"label":"unexpected lifetime argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of lifetime arguments: expected 2, found 3\n  --> /checkout/src/test/ui/constructor-lifetime-args.rs:34:30\n   |\nLL |     E::V::<'static, 'static, 'static>(&0);\n   |                              ^^^^^^^ unexpected lifetime argument\n\n"}
[00:46:53] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:46:53] {"message":"For more information about this error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0107`.\n"}
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] thread '[ui] ui/constructor-lifetime-args.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:53] 
[00:46:53] 
[00:46:53] ---- [ui] ui/error-codes/E0088.rs stdout ----
[00:46:53] normalized stderr:
[00:46:53] error[E0107]: wrong number of lifetime arguments: expected 0, found 1
[00:46:53]   --> $DIR/E0088.rs:15:9
[00:46:53]    |
[00:46:53] LL |     f::<'static>(); //~ ERROR E0107
[00:46:53]    |         ^^^^^^^ unexpected lifetime argument
[00:46:53] error[E0107]: wrong number of lifetime arguments: expected 1, found 2
[00:46:53]   --> $DIR/E0088.rs:16:18
[00:46:53]    |
[00:46:53]    |
[00:46:53] LL |     g::<'static, 'static>(); //~ ERROR E0107
[00:46:53]    |                  ^^^^^^^ unexpected lifetime argument
[00:46:53] error: aborting due to 2 previous errors
[00:46:53] 
[00:46:53] For more information about this error, try `rustc --explain E0107`.
[00:46:53] 
[00:46:53] 
[00:46:53] 
[00:46:53] 
[00:46:53] The actual stderr differed from the expected stderr.
[00:46:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0088/E0088.stderr
[00:46:53] To update references, rerun the tests and pass the `--bless` flag
[00:46:53] To only update this specific test, also pass `--test-args error-codes/E0088.rs`
[00:46:53] error: 1 errors occurred comparing output.
[00:46:53] status: exit code: 1
[00:46:53] status: exit code: 1
[00:46:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0088.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0088/a" "-Crpath" "-O" "-Zunstableber of type arguments:\n                                  //        expected 1, found 0\nstruct Baz<S, T> { x: Foo<S, T> } // error: wrong number of type arugemtns:\n                                  //        expected 1, found 2\n