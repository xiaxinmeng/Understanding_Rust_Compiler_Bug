\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-uninit-ref-chain.rs","byte_start":1495,"byte_end":1499,"line_start":46,"line_end":46,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    let _b = &a.y; //[ast]~ ERROR use of possibly uninitialized variable: `a.y` [E0381]","highlight_start":14,"highlight_end":18}],"label":"use of possibly uninitialized `a.y`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0381]: borrow of possibly uninitialized variable: `a.y`\n  --> /checkout/src/test/ui/borrowck/borrowck-uninit-ref-chain.rs:46:14\n   |\nLL |     let _b = &a.y; //[ast]~ ERROR use of possibly uninitialized variable: `a.y` [E0381]\n   |              ^^^^ use of possibly uninitialized `a.y`\n\n"}
[00:51:50] {"message":"cannot assign to `a.x` when `a` is not initialized","code":{"code":"E0718","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-uninit-ref-chain.rs","byte_start":1650,"byte_end":1659,"line_start":50,"line_end":50,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    a.x = &&0;","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0718]: cannot assign to `a.x` when `a` is not initialized\n  --> /checkout/src/test/ui/borrowck/borrowck-uninit-ref-cowck-use-in-index-lvalue.rs:20:5
[00:51:50] +    |
[00:51:50] + LL |     w[5] = 0; //[ast]~ ERROR use of possibly uninitialized variable: `*w` [E0381]
[00:51:50] 14 
[00:51:50] - For more information about this error, try `rustc --explain E0381`.
[00:51:50] + error: aborting due to 4 previous errors
[00:51:50] + 
[00:51:50] + 
[00:51:50] + Some errors occurred: E0381, E0718.
[00:51:50] + For more information about an error, try `rustc --explain E0381`.
[00:51:50] 16 
[00:51:50] 
[00:51:50] 
[00:51:50] The actual stderr differed from the expected stderr.
[00:51:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-use-in-index-lvalue.mir/borrowck-use-in-index-lvalue.mir.stderr
[00:51:50] To update references, rerun the tests and pass the `--bless` flag
[00:51:50] To only update this specific test, also pass `--test-args borrowck/borrowck-use-in-index-lvalue.rs`
[00:51:50] 
[00:51:50] error in revision `mir`: 1 errors occurred comparing output.
[00:51:50] status: exit code: 1
[00:51:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-use-in-index-lvalue.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-use-in-index-lvalue.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-use-inull},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-use-in-index-lvalue.rs","byte_start":569,"byte_end":577,"line_start":16,"line_end":16,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    w[5] = 0; //[ast]~ ERROR use of possibly uninitialized variable: `*w` [E0381]","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0718]: cannot assign to `w[..]` when `w` is not initialized\n  --> /checkout/src/test/ui/borrowck/borrowck-use-in-index-lvalue.rs:16:5\n   |\nLL |     w[5] = 0; //[ast]~ ERROR use of possibly uninitialized variable: `*w` [E0381]\n   |     ^^^^^^^^\n\n"}
[00:51:50] {"message":"use of possibly uninitialized variable: `*w`","code":{"code":"E0381","explanation":"\nIt is not allowed to use or capture an uninitialized variable. For example:\n\n