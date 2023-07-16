\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-creating-enums.rs","byte_start":1113,"byte_end":1132,"line_start":38,"line_end":38,"column_start":16,"column_end":35,"is_primary":true,"text":[{"text":"        return &Ast::Add(m_x, m_y);  //~ ERROR borrowed value does not live long enough","highlight_start":16,"highlight_end":35}],"label":"returns a reference to data owned by the current function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/regions/regions-creating-enums.rs","byte_start":1114,"byte_end":1132,"line_start":38,"line_end":38,"column_start":17,"column_end":35,"is_primary":false,"text":[{"text":"        return &Ast::Add(m_x, m_y);  //~ ERROR borrowed value does not live long enough","highlight_start":17,"highlight_end":35}],"label":"temporary value created here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0515]: cannot return reference to temporary value\n  --> /checkout/src/test/ui/regions/regions-creating-enums.rs:38:16\n   |\nLL |         return &Ast::Add(m_x, m_y);  //~ ERROR borrowed value does not live long enough\n   |                ^------------------\n   |                ||\n   |                |temporary value created here\n   |                returns a reference to data owned by the current function\n\n"}
[01:20:28] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:20:28] {"message":"For more information about this error, try `rustc --explain E0515`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0515`.\n"}
[01:20:28] ------------------------------------------
[01:20:28] 
[01:20:28] thread '[ui (nll)] ui/regions/regions-creating-enums.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:28] 
[01:20:28] 
[01:20:28] ---- [ui (nll)] ui/regions/regions-infer-borrow-scope-too-big.rs stdout ----
[01:20:28] diff of stderr:
[01:20:28] 
[01:20:28] 1 error[E0515]: cannot return value referencing local data `*p`
[01:20:28] +   --> $DIR/regions-infer-borrow-scope-too-big.rs:23:12
[01:20:28] 3    |
[01:20:28] 3    |
[01:20:28] 4 LL |     let xc = x_coord(&*p); //~ ERROR `*p` does not live long enough
[01:20:28] 5    |                      --- `*p` is borrowed here
[01:20:28] 
[01:20:28] The actual stderr differed from the expected stderr.
[01:20:28] The actual stderr differed from the expected stderr.
[01:20:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-borrow-scope-too-big.nll/regions-infer-borrow-scope-too-big.nll.stderr
[01:20:28] To update references, rerun the tests and pass the `--bless` flag
[01:20:28] To only update this specific test, also pass `--test-args regions/regions-infer-borrow-scope-too-big.rs`
[01:20:28] error: 1 errors occurred comparing output.
[01:20:28] status: exit code: 1
[01:20:28] status: exit code: 1
[01:20:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-borrow-scope-too-big.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-borrow-scope-too-big.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-borrow-scope-too-big.nll/auxiliary" "-A" "unused"
[01:20:28] ------------------------------------------
[01:20:28] 
[01:20:28] ------------------------------------------
[01:20:28] stderr:
[01:20:28] stderr:
[01:20:28] ------------------------------------------
[01:20:28] {"message":"cannot return value referencing local data `*p`","code":{"code":"E0515","explanation":"\nCannot return value that references local variable\n\nLocal variables, function parameters and temporaries are all dropped before the\nend of the function body. So a reference to them cannot be returned.\n\n