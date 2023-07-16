\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-assign-comp.rs","byte_start":1558,"byte_end":1562,"line_start":45,"line_end":45,"column_start":13,"column_end":17,"is_primary":false,"text":[{"text":"    let q = &p.y;","highlight_start":13,"highlight_end":17}],"label":"borrow of `p.y` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-assign-comp.rs","byte_start":1568,"byte_end":1575,"line_start":46,"line_end":46,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    p.y = 5; //[ast]~ ERROR cannot assign to `p.y`","highlight_start":5,"highlight_end":12}],"label":"assignment to borrowed `p.y` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-assign-comp.rs","byte_start":1694,"byte_end":1696,"line_start":48,"line_end":48,"column_start":5,"column_end":7,"is_primary":false,"text":[{"text":"    *q;","highlight_start":5,"highlight_end":7}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0506]: cannot assign to `p.y` because it is borrowed\n  --> /checkout/src/test/ui/borrowck/borrowck-assign-comp.rs:46:5\n   |\nLL |     let q = &p.y;\n   |             ---- borrow of `p.y` occurs here\nLL |     p.y = 5; //[ast]~ ERROR cannot assign to `p.y`\n   |     ^^^^^^^ assignment to borrowed `p.y` occurs here\nLL |              //[mir]~^ ERROR cannot assign to `p.y` because it is borrowed\nLL |     *q;\n   |     -- borrow later used here\n\n"}
[01:20:27] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:20:27] {"message":"For more information about this error, try `rustc --explain E0506`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0506`.\n"}
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] thread '[ui (nll)] ui/borrowck/borrowck-assign-comp.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:27] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:505:22
[01:20:27] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:505:22
[01:20:27] 
[01:20:27] ---- [ui (nll)] ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs stdout ----
[01:20:27] diff of stderr:
[01:20:27] make: *** [check] Error 1
[01:20:27] 
[01:20:27] 1 error[E0716]: temporary value dropped while borrowed
[01:20:27] +   --> $DIR/borrowck-borrowed-uniq-rvalue-2.rs:30:20
[01:20:27] 3    |
[01:20:27] 3    |
[01:20:27] - LL |     let x = defer(&vec!["Goodbye", "world!"]);
[01:20:27] + LL |     let x = defer(&vec!["Goodbye", "world!"]); //~ ERROR borrowed value does not live long enough
[01:20:27] 5    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
[01:20:27] 6    |                    |
[01:20:27] 7    |                    creates a temporary which is freed while still in use
[01:20:27] 
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.nll/borrowck-borrowed-uniq-rvalue-2.nll.stderr
[01:20:27] To update references, rerun the tests and pass the `--bless` flag
[01:20:27] To only update this specific test, also pass `--test-args borrowck/borrowck-borrowed-uniq-rvalue-2.rs`
[01:20:27] error: 1 errors occurred comparing output.
[01:20:27] status: exit code: 1
[01:20:27] status: exit code: 1
[01:20:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.nll/auxiliary" "-A" "unused"
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] ------------------------------------------
[01:20:27] stderr:
[01:20:27] stderr:
[01:20:27] ------------------------------------------
[01:20:27] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n