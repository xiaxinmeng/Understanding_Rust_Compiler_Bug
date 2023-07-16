\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs","byte_start":10880,"byte_end":10881,"line_start":316,"line_end":316,"column_start":22,"column_end":23,"is_primary":false,"text":[{"text":"                drop(x);","highlight_start":22,"highlight_end":23}],"label":"value moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs","byte_start":10905,"byte_end":10906,"line_start":317,"line_end":317,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"                drop(x); //[ast]~ ERROR use of moved value: `x`","highlight_start":22,"highlight_end":23}],"label":"value used here after move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `x`\n  --> /checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs:317:22\n   |\nLL |                 drop(x);\n   |                      - value moved here\nLL |                 drop(x); //[ast]~ ERROR use of moved value: `x`\n   |                      ^ value used here after move\n   |\n   = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait\n\n"}
[01:19:16] {"message":"aborting due to 30 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 30 previous errors\n\n"}
[01:19:16] {"message":"Some errors occurred: E0382, E0499, E0502, E0503.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0382, E0499, E0502, E0503.\n"}
[01:19:16] {"message":"For more information about an error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0382`.\n"}
[01:19:16] ------------------------------------------
[01:19:16] 
[01:19:16] thread '[ui (nll)] ui/borrowck/borrowck-describe-lvalue.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:19:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:19:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:19:16] 
[01:19:16] ---- [ui (nll)] ui/issues/issue-36082.rs#ast stdout ----
[01:19:16] diff of stderr:
[01:19:16] 
[01:19:16] 1 error[E0716]: temporary value dropped while borrowed
[01:19:16] +   --> $DIR/issue-36082.rs:21:19
[01:19:16] 3    |
[01:19:16] 3    |
[01:19:16] 4 LL |     let val: &_ = x.borrow().0;
[01:19:16] 5    |                   ^^^^^^^^^^  - temporary value is freed at the end of this statement
[01:19:16] 
[01:19:16] The actual stderr differed from the expected stderr.
[01:19:16] The actual stderr differed from the expected stderr.
[01:19:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36082.ast.nll/issue-36082.ast.nll.stderr
[01:19:16] To update references, rerun the tests and pass the `--bless` flag
[01:19:16] To only update this specific test, also pass `--test-args issues/issue-36082.rs`
[01:19:16] 
[01:19:16] error in revision `ast`: 1 errors occurred comparing output.
[01:19:16] status: exit code: 1
[01:19:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-36082.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36082.ast.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36082.ast.nll/auxiliary" "-A" "unused"
[01:19:16] ------------------------------------------
[01:19:16] 
[01:19:16] ------------------------------------------
[01:19:16] stderr:
[01:19:16] stderr:
[01:19:16] ------------------------------------------
[01:19:16] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n