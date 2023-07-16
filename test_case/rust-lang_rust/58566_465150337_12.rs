\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/ch04-00-understanding-ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-uninit-field-access.rs","byte_start":772,"byte_end":784,"line_start":33,"line_end":33,"column_start":33,"column_end":45,"is_primary":false,"text":[{"text":"    let _moved = (line2.origin, line2.middle);","highlight_start":33,"highlight_end":45}],"label":"value moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-uninit-field-access.rs","byte_start":791,"byte_end":796,"line_start":34,"line_end":34,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    line2.consume(); //[ast]~ ERROR use of partially moved value: `line2` [E0382]","highlight_start":5,"highlight_end":10}],"label":"value used here after partial move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `line2.middle` has type `Point`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `line2`\n  --> /checkout/src/test/ui/borrowck/borrowck-uninit-field-access.rs:34:5\n   |\nLL |     let _moved = (line2.origin, line2.middle);\n   |                                 ------------ value moved here\nLL |     line2.consume(); //[ast]~ ERROR use of partially moved value: `line2` [E0382]\n   |     ^^^^^ value used here after partial move\n   |\n   = note: move occurs because `line2.middle` has type `Point`, which does not implement the `Copy` trait\n\n"}
[01:30:22] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:30:22] {"message":"Some errors occurred: E0381, E0382.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0381, E0382.\n"}
[01:30:22] 
[01:30:22] ------------------------------------------
[01:30:22] 
[01:30:22] thread '[ui (nll)] ui/borrowck/borrowck-uninit-field-access.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:30:22] thread '[ui (nll)] ui/borrowck/borrowck-uninit-field-access.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:30:22] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:30:22] 
[01:30:22] ---- [ui (nll)] ui/moves/moves-based-on-type-cyclic-types-issue-4821.rs stdout ----
[01:30:22] diff of stderr:
[01:30:22] 
[01:30:22] 5    |              ----- value moved here
[01:30:22] 6 ...
[01:30:22] 7 LL |     consume(node) + r //~ ERROR use of partially moved value: `node`
[01:30:22] -    |             ^^^^ value used here after move
[01:30:22] +    |             ^^^^ value used here after partial move
[01:30:22] 9    |
[01:30:22] 10    = note: move occurs because value has type `std::boxed::Box<List>`, which does not implement the `Copy` trait
[01:30:22] 
[01:30:22] 
[01:30:22] The actual stderr differed from the expected stderr.
[01:30:22] The actual stderr differed from the expected stderr.
[01:30:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-cyclic-types-issue-4821.nll/moves-based-on-type-cyclic-types-issue-4821.nll.stderr
[01:30:22] To update references, rerun the tests and pass the `--bless` flag
[01:30:22] To only update this specific test, also pass `--test-args moves/moves-based-on-type-cyclic-types-issue-4821.rs`
[01:30:22] error: 1 errors occurred comparing output.
[01:30:22] status: exit code: 1
[01:30:22] status: exit code: 1
[01:30:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/moves-based-on-type-cyclic-types-issue-4821.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-cyclic-types-issue-4821.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-cyclic-types-issue-4821.nll/auxiliary" "-A" "unused"
[01:30:22] ------------------------------------------
[01:30:22] 
[01:30:22] ------------------------------------------
[01:30:22] stderr:
[01:30:22] stderr:
[01:30:22] ------------------------------------------
[01:30:22] {"message":"use of moved value: `node`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n