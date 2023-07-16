\n\nIf you don't know the basics of Rust, you can go look to the Rust Book to get\nstarted: https://doc.rust-lang.org/book/\n"},"level":"error","spans":[],"children":[{"message":"consider adding a `main` function to `/checkout/src/test/ui/tuple-struct-fields/test.rs`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0601]: `main` function not found in crate `test`\n   |\n   = note: consider adding a `main` function to `/checkout/src/test/ui/tuple-struct-fields/test.rs`\n\n"}
[00:44:44] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:44:44] {"message":"Some errors occurred: E0412, E0601.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0412, E0601.\n"}
[00:44:44] {"message":"For more information about an error, try `rustc --explain E0412`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0412`.\n"}
[00:44:44] ------------------------------------------
[00:44:44] 
[00:44:44] thread '[ui] ui/tuple-struct-fields/test.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:44:44] 
[00:44:44] 
[00:44:44] ---- [ui] ui/tuple-struct-fields/test3.rs stdout ----
[00:44:44] normalized stderr:
[00:44:44] error: expected one of `)` or `,`, found `(`
[00:44:44]    |
[00:44:44]    |
[00:44:44] LL |         struct S3(pub($t) ());
[00:44:44]    |                           ^ expected one of `)` or `,` here
[00:44:44] ...
[00:44:44] LL |     define_struct! { foo }
[00:44:44] 
[00:44:44] error: aborting due to previous error
[00:44:44] 
[00:44:44] 
[00:44:44] 
[00:44:44] 
[00:44:44] 
[00:44:44] The actual stderr differed from the expected stderr.
[00:44:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple-struct-fields/test3/test3.stderr
[00:44:44] To update references, rerun the tests and pass the `--bless` flag
[00:44:44] To only update this specific test, also pass `--test-args tuple-struct-fields/test3.rs`
[00:44:44] error: 1 errors occurred comparing output.
[00:44:44] status: exit code: 101
[00:44:44] status: exit code: 101
[00:44:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tuple-struct-fields/test3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple-struct-fields/test3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple-struct-fields/test3/auxiliary" "-A" "unused"
[00:44:44] ------------------------------------------
[00:44:44] 
[00:44:44] ------------------------------------------
[00:44:44] stderr:
[00:44:44] stderr:
[00:44:44] ------------------------------------------
[00:44:44] {"message":"expected one of `)` or `,`, found `(`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/tuple-struct-fields/test3.rs","byte_start":603,"byte_end":604,"line_start":15,"line_end":15,"column_start":27,"column_end":28,"is_primary":true,"text":[{"text":"        struct S3(pub($t) ());","highlight_start":27,"highlight_end":28}],"label":"expected one of `)` or `,` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/tuple-struct-fields/test3.rs","byte_start":688,"byte_end":710,"line_start":21,"line_end":21,"column_start":5,"column_end":27,"is_primary":false,"text":[{"text":"    define_struct! { foo }","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"define_struct!","def_site_span":{"file_name":"/checkout/src/test/ui/tuple-struct-fields/test3.rs","byte_start":467,"byted/bootstrap/debug/bootstrap test
[00:44:44] Build completed unsuccessfully in 0:02:36
[00:44:44] Makefile:58: recipe for target 'check' failed
[00:44:44] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03735339
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
