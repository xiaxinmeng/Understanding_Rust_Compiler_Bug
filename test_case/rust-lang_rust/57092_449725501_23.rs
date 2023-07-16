\n"},"level":"error","spans":[{"file_name":"<::alloc::macros::vec macros>","byte_start":115,"byte_end":162,"line_start":3,"line_end":3,"column_start":1,"column_end":48,"is_primary":true,"text":[{"text":"< [ _ ] > :: into_vec ( box [ $ ( $ x ) , * ] ) ) ; ( $ ( $ x : expr , ) * )","highlight_start":1,"highlight_end":48}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs","byte_start":751,"byte_end":776,"line_start":30,"line_end":30,"column_start":20,"column_end":45,"is_primary":false,"text":[{"text":"    let x = defer(&vec![\"Goodbye\", \"world!\"]); //~ ERROR borrowed value does not live long enough","highlight_start":20,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"vec!","def_site_span":{"file_name":"<::alloc::macros::vec macros>","byte_start":0,"byte_end":222,"line_start":1,"line_end":4,"column_start":1,"column_end":31,"is_primary":false,"text":[{"text":"( $ elem : expr ; $ n : expr ) => (","highlight_start":1,"highlight_end":36},{"text":"$ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (","highlight_start":1,"highlight_end":79},{"text":"< [ _ ] > :: into_vec ( box [ $ ( $ x ) , * ] ) ) ; ( $ ( $ x : expr , ) * )","highlight_start":1,"highlight_end":77},{"text":"=> ( vec ! [ $ ( $ x ) , * ] )","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs","byte_start":777,"byte_end":778,"line_start":30,"line_end":30,"column_start":46,"column_end":47,"is_primary":false,"text":[{"text":"    let x = defer(&vec![\"Goodbye\", \"world!\"]); //~ ERROR borrowed value does not live long enough","highlight_start":46,"highlight_end":47}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs","byte_start":834,"byte_end":840,"line_start":31,"line_end":31,"column_start":5,"column_end":11,"is_primary":false,"text":[{"text":"    x.x[0];","highlight_start":5,"highlight_end":11}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a `let` binding to create a longer lived value","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs:30:20\n   |\nLL |     let x = defer(&vec![\"Goodbye\", \"world!\"]); //~ ERROR borrowed value does not live long enough\n   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement\n   |                    |\n   |                    creates a temporary which is freed while still in use\nLL |     x.x[0];\n   |     ------ borrow later used here\n   |\n   = note: consider using a `let` binding to create a longer lived value\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:20:27] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:20:27] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] thread '[ui (nll)] ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:27] 
[01:20:27] 
[01:20:27] ---- [ui (nll)] ui/borrowck/borrowck-init-in-fru.rs#ast stdout ----
[01:20:27] diff of stderr:
[01:20:27] 
[01:20:27] 1 error[E0381]: use of possibly uninitialized variable: `origin`
[01:20:27] 3    |
[01:20:27] 3    |
[01:20:27] - LL |     origin = point {x: 10,.. origin};
[01:20:27] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly uninitialized `origin.y`
[01:20:27] + LL |     origin = Point { x: 10, ..origin };
[01:20:27] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly uninitialized `origin.y`
[01:20:27] 7 error: aborting due to previous error
[01:20:27] 8 
[01:20:27] 
[01:20:27] 
[01:20:27] 
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-init-in-fru.ast.nll/borrowck-init-in-fru.ast.nll.stderr
[01:20:27] To update references, rerun the tests and pass the `--bless` flag
[01:20:27] To only update this specific test, also pass `--test-args borrowck/borrowck-init-in-fru.rs`
[01:20:27] 
[01:20:27] error in revision `ast`: 1 errors occurred comparing output.
[01:20:27] status: exit code: 1
[01:20:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-init-in-fru.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-init-in-fru.ast.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-init-in-fru.ast.nll/auxiliary" "-A" "unused"
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] ------------------------------------------
[01:20:27] stderr:
[01:20:27] stderr:
[01:20:27] ------------------------------------------
[01:20:27] {"message":"use of possibly uninitialized variable: `origin`","code":{"code":"E0381","explanation":"\nIt is not allowed to use or capture an uninitialized variable. For example:\n\n