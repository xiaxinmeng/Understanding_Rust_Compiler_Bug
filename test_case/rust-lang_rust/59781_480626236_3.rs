\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-36163.rs","byte_start":54,"byte_end":55,"line_start":4,"line_end":4,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    B = A, //~ ERROR E0391","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which requires processing `A`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-36163.rs","byte_start":17,"byte_end":32,"line_start":1,"line_end":1,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"const A: isize = Foo::B as isize;","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires processing `Foo::B::{{constant}}#0`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing `Foo::B::{{constant}}#0`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-36163.rs","byte_start":54,"byte_end":55,"line_start":4,"line_end":4,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    B = A, //~ ERROR E0391","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when processing `Foo::B::{{constant}}#0`\n  --> /checkout/src/test/ui/issues/issue-36163.rs:4:9\n   |\nLL |     B = A, //~ ERROR E0391\n   |         ^\n   |\nnote: ...which requires processing `A`...\n  --> /checkout/src/test/ui/issues/issue-36163.rs:1:18\n   |\nLL | const A: isize = Foo::B as isize;\n   |                  ^^^^^^^^^^^^^^^\n   = note: ...which again requires processing `Foo::B::{{constant}}#0`, completing the cycle\nnote: cycle used when processing `Foo::B::{{constant}}#0`\n  --> /checkout/src/test/ui/issues/issue-36163.rs:4:9\n   |\nLL |     B = A, //~ ERROR E0391\n   |         ^\n\n"}
[01:08:38] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[01:08:38] 
[01:08:38] ------------------------------------------
[01:08:38] 
[01:08:38] 
[01:08:38] thread '[ui] ui/issues/issue-36163.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:08:38] 
[01:08:38] ---- [ui] ui/issues/issue-51714.rs stdout ----
[01:08:38] diff of stderr:
[01:08:38] 
[01:08:38] 22 LL |     [(); return while let Some(n) = Some(0) {}];
[01:08:38] 24 
[01:08:38] - warning: irrefutable while-let pattern
[01:08:38] -   --> $DIR/issue-51714.rs:11:17
[01:08:38] -    |
[01:08:38] -    |
[01:08:38] - LL |     [(); return while let Some(n) = Some(0) {}];
[01:08:38] -    |
[01:08:38] -    = note: #[warn(irrefutable_let_patterns)] on by default
[01:08:38] - 
[01:08:38] 33 error: aborting due to 4 previous errors
[01:08:38] 33 error: aborting due to 4 previous errors
[01:08:38] 34 
[01:08:38] 35 For more information about this error, try `rustc --explain E0572`.
[01:08:38] 
[01:08:38] 
[01:08:38] The actual stderr differed from the expected stderr.
[01:08:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51714/issue-51714.stderr
[01:08:38] To update references, rerun the tests and pass the `--bless` flag
[01:08:38] To only update this specific test, also pass `--test-args issues/issue-51714.rs`
[01:08:38] error: 1 errors occurred comparing output.
[01:08:38] status: exit code: 1
[01:08:38] status: exit code: 1
[01:08:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51714.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51714/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51714/auxiliary" "-A" "unused"
[01:08:38] ------------------------------------------
[01:08:38] 
[01:08:38] ------------------------------------------
[01:08:38] stderr:
[01:08:38] stderr:
[01:08:38] ------------------------------------------
[01:08:38] {"message":"return statement outside of function body","code":{"code":"E0572","explanation":"\nA return statement was found outside of a function body.\n\nErroneous code example:\n\n