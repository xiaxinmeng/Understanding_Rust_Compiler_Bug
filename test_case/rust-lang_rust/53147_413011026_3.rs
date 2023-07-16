\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/E0508-fail.rs","byte_start":604,"byte_end":612,"line_start":18,"line_end":18,"column_start":18,"column_end":26,"is_primary":true,"text":[{"text":"    let _value = array[0];  //[ast]~ ERROR [E0508]","highlight_start":18,"highlight_end":26}],"label":"cannot move out of here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider borrowing here","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/E0508-fail.rs","byte_start":604,"byte_end":612,"line_start":18,"line_end":18,"column_start":18,"column_end":26,"is_primary":true,"text":[{"text":"    let _value = array[0];  //[ast]~ ERROR [E0508]","highlight_start":18,"highlight_end":26}],"label":null,"suggested_replacement":"&array[0]","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0508]: cannot move out of type `[NonCopy; 1]`, a non-copy array\n  --> /checkout/src/test/ui/E0508-fail.rs:18:18\n   |\nLL |     let _value = array[0];  //[ast]~ ERROR [E0508]\n   |                  ^^^^^^^^\n   |                  |\n   |                  cannot move out of here\n   |                  help: consider borrowing here: `&array[0]`\n\n"}
[00:52:46] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:52:46] {"message":"For more information about this error, try `rustc --explain E0508`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0508`.\n"}
[00:52:46] ------------------------------------------
[00:52:46] 
[00:52:46] thread '[ui (nll)] ui/E0508-fail.rs#ast' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:52:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:46] 
[00:52:46] ---- [ui (nll)] ui/E0508.rs stdout ----
[00:52:46] diff of stderr:
[00:52:46] 
[00:52:46] 5    |                  ^^^^^^^^
[00:52:46] 6    |                  |
[00:52:46] 7    |                  cannot move out of here
[00:52:46] -    |                  help: consider using a reference instead: `&array[0]`
[00:52:46] +    |                  help: consider borrowing here: `&array[0]`
[00:52:46] 10 error: aborting due to previous error
[00:52:46] 11 
[00:52:46] 
[00:52:46] 
[00:52:46] 
[00:52:46] The actual stderr differed from the expected stderr.
[00:52:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0508.nll/E0508.nll.stderr
[00:52:46] To update references, rerun the tests and pass the `--bless` flag
[00:52:46] To only update this specific test, also pass `--test-args E0508.rs`
[00:52:46] error: 1 errors occurred comparing output.
[00:52:46] status: exit code: 1
[00:52:46] status: exit code: 1
[00:52:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0508.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0508.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0508.nll/auxiliary" "-A" "unused"
[00:52:46] ------------------------------------------
[00:52:46] 
[00:52:46] ------------------------------------------
[00:52:46] stderr:
[00:52:46] stderr:
[00:52:46] ------------------------------------------
[00:52:46] {"message":"cannot move out of type `[NonCopy; 1]`, a non-copy array","code":{"code":"E0508","explanation":"\nA value was moved out of a non-copy fixed-size array.\n\nExample of erroneous code:\n\n