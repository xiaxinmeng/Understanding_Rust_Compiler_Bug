\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-14021.rs","byte_start":654,"byte_end":669,"line_start":19,"line_end":19,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"use serialize::json;","highlight_start":5,"highlight_end":20}],"label":"no `json` in the root","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0432]: unresolved import `serialize::json`\n  --> /checkout/src/test/run-pass-fulldeps/issue-14021.rs:19:5\n   |\nLL | use serialize::json;\n   |     ^^^^^^^^^^^^^^^ no `json` in the root\n\n"}
[01:00:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:10] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] thread '[run-pass] run-pass-fulldeps/issue-14021.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:00:10] 
[01:00:10] 
[01:00:10] ---- [run-pass] run-pass-fulldeps/issue-15924.rs stdout ----
[01:00:10] 
[01:00:10] error: test compilation failed although it shouldn't!
[01:00:10] status: exit code: 1
[01:00:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/issue-15924.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-15924/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-15924/auxiliary"
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] ------------------------------------------
[01:00:10] stderr:
[01:00:10] stderr:
[01:00:10] ------------------------------------------
[01:00:10] {"message":"unresolved import `serialize::json`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n