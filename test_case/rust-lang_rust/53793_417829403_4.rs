\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-23302-1.rs","byte_start":612,"by~ ERROR E0391","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when processing `Foo::B::{{constant}}`\n  --> /checkout/src/test/ui/issues/issue-36163.rs:14:9\n   |\nLL |     B = A, //~ ERROR E0391\n   |         ^\n   |\nnote: ...which requires processing `A`...\n  --> /checkout/src/test/ui/issues/issue-36163.rs:11:18\n   |\nLL | const A: isize = Foo::B as isize;\n   |                  ^^^^^^^^^^^^^^^\n   = note: ...which again requires processing `Foo::B::{{constant}}`, completing the cycle\nnote: cycle used when const-evaluating `Foo::B::{{constant}}`\n  --> /checkout/src/test/ui/issues/issue-36163.rs:14:9\n   |\nLL |     B = A, //~ ERROR E0391\n   |         ^\n\n"}
[00:47:21] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:21] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:47:21] ------------------------------------------
[00:47:21] 
[00:47:21] thread '[ui] ui/issues/issue-36163.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:21] 
[00:47:21] 
[00:47:21] ---- [ui] ui/issues/issue-44415.rs stdout ----
[00:47:21] diff of stderr:
[00:47:21] 
[00:47:21] 7 LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
[00:47:21] 8    |                  ired lifetime bounds
[00:47:21] +   --> $DIR/regions-enum-not-wf.rs:45:1
[00:47:21] 44    |
[00:47:21] 44    |
[00:47:21] 45 LL | / enum RefDouble<'a, 'b, T> { //~ ERROR 43:1: 46:2: the parameter type `T` may not live long enough [E0309]
[00:47:21] 46 LL | |     RefDoubleVariant1(&'a RequireOutlives<'b, T>)
[00:47:21] 49    | |_^
[00:47:21] 50 
[00:47:21] 50 
[00:47:21] 51 error[E0309]: the parameter type `T` may not live long enough
[00:47:21] +   --> $DIR/regions-enum-not-wf.rs:46:23
[00:47:21] 53    |
[00:47:21] 53    |
[00:47:21] 54 LL | enum RefDouble<'a, 'b, T> { //~ ERROR 43:1: 46:2: the parameter type `T` may not live long enough [E0309]
[00:47:21] 55    |                        - help: consider adding an explicit lifetime bound `T: 'b`...
[00:47:21] 57    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:47:21] 58    |
[00:47:21] 58    |
[00:47:21] 59 note: ...so that the type `T` will meet its required lifetime bounds
[00:47:21] +   --> $DIR/regions-enum-not-wf.rs:46:23
[00:47:21] 61    |
[00:47:21] 61    |
[00:47:21] 62 LL |     RefDoubleVariant1(&'a RequireOutlives<'b, T>)
[00:47:21] 
[00:47:21] 
[00:47:21] The actual stderr differed from the expected stderr.
[00:47:21] The actual stderr differed from the expected stderr.
[00:47:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-enum-not-wf/regions-enum-not-wf.stderr
[00:47:21] To update references, rerun the tests and pass the `--bless` flag
[00:47:21] To only update this specific test, also pass `--test-args regions/regions-enum-not-wf.rs`
[00:47:21] error: 1 errors occurred comparing output.
[00:47:21] status: exit code: 1
[00:47:21] status: exit code: 1
[00:47:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-enum-not-wf.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-enum-not-wf/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-enum-not-wf/auxiliary" "-A" "unused"
[00:47:21] ------------------------------------------
[00:47:21] 
[00:47:21] ------------------------------------------
[00:47:21] stderr:
[00:47:21] stderr:
[00:47:21] ------------------------------------------
[00:47:21] {"message":"the parameter type `T` may not live long enough","code":{"code":"E0309","explanation":"\nTypes in type definitions have lifetimes associated with them that represent\nhow long the data stored within them is guaranteed to be live. This lifetime\nmust be as long as the data needs to be alive, and missing the constraint that\ndenotes this will cause this error.\n\n