\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/ch04-00-understanding-ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/unsized-locals/borrow-after-move.rs","byte_start":747,"byte_end":748,"line_start":38,"line_end":38,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"        x.foo();","highlight_start":9,"highlight_end":10}],"label":"value moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/unsized-locals/borrow-after-move.rs","byte_start":779,"byte_end":781,"line_start":39,"line_end":39,"column_start":24,"column_end":26,"is_primary":true,"text":[{"text":"        println!(\"{}\", &x);","highlight_start":24,"highlight_end":26}],"label":"value borrowed here after partial move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `*x` has type `str`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: borrow of moved value: `x`\n  --> /checkout/src/test/ui/unsized-locals/borrow-after-move.rs:39:24\n   |\nLL |         x.foo();\n   |         - value moved here\nLL |         println!(\"{}\", &x);\n   |                        ^^ value borrowed here after partial move\n   |\n   = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait\n\n"}
[01:30:22] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[01:30:22] 
[01:30:22] ------------------------------------------
[01:30:22] 
[01:30:22] 
[01:30:22] thread '[ui (nll)] ui/unsized-locals/borrow-after-move.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:30:22] 
[01:30:22] ---- [ui (nll)] ui/unsized-locals/double-move.rs stdout ----
[01:30:22] diff of stderr:
[01:30:22] 
[01:30:22] 14 LL |         let _y = *x;
[01:30:22] 15    |                  -- value moved here
[01:30:22] 16 LL |         drop_unsized(x); //~ERROR use of moved value
[01:30:22] -    |                      ^ value used here after move
[01:30:22] +    |                      ^ value used here after partial move
[01:30:22] 18    |
[01:30:22] 19    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
[01:30:22] 
[01:30:22] 
[01:30:22] The actual stderr differed from the expected stderr.
[01:30:22] The actual stderr differed from the expected stderr.
[01:30:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move.nll/double-move.nll.stderr
[01:30:22] To update references, rerun the tests and pass the `--bless` flag
[01:30:22] To only update this specific test, also pass `--test-args unsized-locals/double-move.rs`
[01:30:22] error: 1 errors occurred comparing output.
[01:30:22] status: exit code: 1
[01:30:22] status: exit code: 1
[01:30:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/double-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move.nll/auxiliary" "-A" "unused"
[01:30:22] ------------------------------------------
[01:30:22] 
[01:30:22] ------------------------------------------
[01:30:22] stderr:
[01:30:22] stderr:
[01:30:22] ------------------------------------------
[01:30:22] {"message":"use of moved value: `y`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n