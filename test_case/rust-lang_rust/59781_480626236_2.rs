\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-23302-2.rs","byte_start":100,"byte_end":113,"line_start":4,"line_end":4,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"    A = Y::B as isize, //~ ERROR E0391","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires processing `Y::A::{{constant}}#0`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing `Y::A::{{constant}}#0`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-23302-2.rs","byte_start":100,"byte_end":113,"line_start":4,"line_end":4,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"    A = Y::B as isize, //~ ERROR E0391","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when processing `Y::A::{{constant}}#0`\n  --> /checkout/src/test/ui/issues/issue-23302-2.rs:4:9\n   |\nLL |     A = Y::B as isize, //~ ERROR E0391\n   |         ^^^^^^^^^^^^^\n   |\n   = note: ...which again requires processing `Y::A::{{constant}}#0`, completing the cycle\nnote: cycle used when processing `Y::A::{{constant}}#0`\n  --> /checkout/src/test/ui/issues/issue-23302-2.rs:4:9\n   |\nLL |     A = Y::B as isize, //~ ERROR E0391\n   |         ^^^^^^^^^^^^^\n\n"}
[01:08:38] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[01:08:38] 
[01:08:38] ------------------------------------------
[01:08:38] 
[01:08:38] 
[01:08:38] thread '[ui] ui/issues/issue-23302-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:08:38] 
[01:08:38] ---- [ui] ui/issues/issue-36163.rs stdout ----
[01:08:38] diff of stderr:
[01:08:38] 
[01:08:38] 10 LL | const A: isize = Foo::B as isize;
[01:08:38] 11    |                  ^^^^^^^^^^^^^^^
[01:08:38] 12    = note: ...which again requires processing `Foo::B::{{constant}}#0`, completing the cycle
[01:08:38] - note: cycle used when const-evaluating `Foo::B::{{constant}}#0`
[01:08:38] + note: cycle used when processing `Foo::B::{{constant}}#0`
[01:08:38] 15    |
[01:08:38] 16 LL |     B = A,
[01:08:38] 
[01:08:38] 
[01:08:38] 
[01:08:38] The actual stderr differed from the expected stderr.
[01:08:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36163/issue-36163.stderr
[01:08:38] To update references, rerun the tests and pass the `--bless` flag
[01:08:38] To only update this specific test, also pass `--test-args issues/issue-36163.rs`
[01:08:38] error: 1 errors occurred comparing output.
[01:08:38] status: exit code: 1
[01:08:38] status: exit code: 1
[01:08:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-36163.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36163/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36163/auxiliary" "-A" "unused"
[01:08:38] ------------------------------------------
[01:08:38] 
[01:08:38] ------------------------------------------
[01:08:38] stderr:
[01:08:38] stderr:
[01:08:38] ------------------------------------------
[01:08:38] {"message":"cycle detected when processing `Foo::B::{{constant}}#0`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n