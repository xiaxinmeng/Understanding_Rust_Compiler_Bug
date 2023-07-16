\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-45697-1.rs","byte_start":935,"byte_end":936,"line_start":29,"line_end":29,"column_start":40,"column_end":41,"is_primary":false,"text":[{"text":"        let z = copy_borrowed_ptr(&mut y);","highlight_start":40,"highlight_end":41}],"label":"borrow of `*y.pointer` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-45697-1.rs","byte_start":947,"byte_end":962,"line_start":30,"line_end":30,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"        *y.pointer += 1;","highlight_start":9,"highlight_end":24}],"label":"assignment to borrowed `*y.pointer` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0506]: cannot assign to `*y.pointer` because it is borrowed (Ast)\n  --> /checkout/src/test/ui/issue-45697-1.rs:30:9\n   |\nLL |         let z = copy_borrowed_ptr(&mut y);\n   |                                        - borrow of `*y.pointer` occurs here\nLL |         *y.pointer += 1;\n   |         ^^^^^^^^^^^^^^^ assignment to borrowed `*y.pointer` occurs here\n\n"}
[00:42:59] thread 'main' panicked at 'index out of bounds: the len is flag
[00:42:59] To only update this specific test, also pass `--test-args issue-45697.rs`
[00:42:59] error: 1 errors occurred comparing output.
[00:42:59] status: exit code: 101
[00:42:59] status: exit code: 101
[00:42:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-45697.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-45697/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "emit-end-regions" "-Z" "borrowck=compare" "-C" "overflow-checks=off" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-45697/auxiliary" "-A" "unused"
[00:42:59] ------------------------------------------
[00:42:59] 
[00:42:59] ------------------------------------------
[00:42:59] stderr:
[00:42:59] stderr:
[00:42:59] ------------------------------------------
[00:42:59] {"message":"cannot assign to `*y.pointer` because it is borrowed (Ast)","code":{"code":"E0506","explanation":"\nThis error occurs when an attempt is made to assign to a borrowed value.\n\nExample of erroneous code:\n\n