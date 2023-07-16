\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-27282-move-match-input-into-guard.rs","byte_start":980,"byte_end":982,"line_start":26,"line_end":26,"column_start":17,"column_end":19,"is_primary":false,"text":[{"text":"        _ if { (|| { let bar = b; *bar = false; })();","highlight_start":17,"highlight_end":19}],"label":"value moved into closure here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-27282-move-match-input-into-guard.rs","byte_start":995,"byte_end":996,"line_start":26,"line_end":26,"column_start":32,"column_end":33,"is_primary":false,"text":[{"text":"        _ if { (|| { let bar = b; *bar = false; })();","highlight_start":32,"highlight_end":33}],"label":"variable moved due to use in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-27282-move-match-input-into-guard.rs","byte_start":1154,"byte_end":1158,"line_start":29,"line_end":29,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"        &mut true => { println!(\"You might think we should get here\"); },","highlight_start":14,"highlight_end":18}],"label":"here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-27282-mutate-before-diverging-arm-1.rs","byte_start":1105,"byte_end":1106,"line_start":29,"line_end":29,"column_start":11,"column_end":12,"is_primary":false,"text":[{"text":"    match x {","highlight_start":11,"highlight_end":12}],"label":"borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-27282-mutate-before-diverging-arm-1.rs","byte_start":1251,"byte_end":1252,"line_start":33,"line_end":33,"column_start":20,"column_end":21,"is_primary":false,"text":[{"text":"            (|| { *x = None; drop(force_fn_once); })();","highlight_start":20,"highlight_end":21}],"label":"second borrow occurs due to use of `x` in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-27282-mutate-before-diverging-arm-1.rs","byte_start":1426,"byte_end":1439,"line_start":37,"line_end":37,"column_start":9,"column_end":22,"is_primary":false,"text":[{"text":"        &mut Some(&a) if { // this binds to garbage if we've corrupted discriminant","highlight_start":9,"highlight_end":22}],"label":"borrow used here in later iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0500]: closure requires unique access to `x` but it is already borrowed\n  --> /checkout/src/test/ui/issue-27282-mutate-before-diverging-arm-1.rs:33:14\n   |\nLL |     match x {\n   |           - borrow occurs here\n...\nLL |             (|| { *x = None; drop(force_fn_once); })();\n   |              ^^    - second borrow occurs due to use of `x` in closure\n   |              |\n   |              closure construction occurs here\n...\nLL |         &mut Some(&a) if { // this binds to garbage if we've corrupted discriminant\n   |         ------------- borrow used here in later iteration of loop\n\n"}
[00:43:19] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:19] {"message":"For more information about this error, try `rustc --explain E0500`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0500`.\n"}
[00:43:19] ------------------------------------------
[00:43:19] 
[00:43:19] thread '[ui] ui/issue-27282-mutate-before-diverging-arm-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:43:19] 
[00:43:19] 
[00:43:19] ---- [ui] ui/issue-27282-mutate-before-diverging-arm-2.rs stdout ----
[00:43:19] diff of stderr:
[00:43:19] 
[00:43:19] 10    |                  closure construction occurs here
[00:43:19] 11 ...
[00:43:19] 12 LL |         &mut Some(&2)
[00:43:19] +    |         ------------- borrow used here in later iteration of loop
[00:43:19] 14 
[00:43:19] 15 error: aborting due to previous error
[00:43:19] 16 
[00:43:19] 16 
[00:43:19] 
[00:43:19] 
[00:43:19] The actual stderr differed from the expected stderr.
[00:43:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-27282-mutate-before-diverging-arm-2/issue-27282-mutate-before-diverging-arm-2.stderr
[00:43:19] To update references, rerun the tests and pass the `--bless` flag
[00:43:19] To only update this specific test, also pass `--test-args issue-27282-mutate-before-diverging-arm-2.rs`
[00:43:19] error: 1 errors occurred comparing output.
[00:43:19] status: exit code: 1
[00:43:19] status: exit code: 1
[00:43:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-27282-mutate-before-diverging-arm-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-27282-mutate-before-diverging-arm-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-27282-mutate-before-diverging-arm-2/auxiliary" "-A" "unused"
[00:43:19] ------------------------------------------
[00:43:19] 
[00:43:19] ------------------------------------------
[00:43:19] stderr:
[00:43:19] stderr:
[00:43:19] ------------------------------------------
[00:43:19] {"message":"closure requires unique access to `x` but it is already borrowed","code":{"code":"E0500","explanation":"\nA borrowed variable was used in another closure. Example of erroneous code:\n\n