\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/issue-41962.rs","byte_start":593,"byte_end":598,"line_start":17,"line_end":17,"column_start":21,"column_end":26,"is_primary":true,"text":[{"text":"        if let Some(thing) = maybe {","highlight_start":21,"highlight_end":26}],"label":"value moved here, in previous iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value (Mir)\n  --> /checkout/src/test/ui/borrowck/issue-41962.rs:17:21\n   |\nLL |         if let Some(thing) = maybe {\n   |                     ^^^^^ value moved here, in previous iteration of loop\n   |\n   = note: move occurs because value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait\n\n"}
[00:48:45] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:48:45] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[00:48:45] ------------------------------------------
[00:48:45] 
[00:48:45] thread '[ui] ui/borrowck/issue-41962.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:45] 
[00:48:45] 
[00:48:45] ---- [ui] ui/nll/closures-in-loops.rs stdout ----
[00:48:45] diff of stderr:
[00:48:45] 
[00:48:45] 4 LL |         || x; //~ ERROR
[00:48:45] 5    |         ^^ - use occurs due to use in closure
[00:48:45] 6    |         |
[00:48:45] -    |         value moved into closure here in previous iteration of loop
[00:48:45] +    |         value moved into closure here, in previous iteration of loop
[00:48:45] 8    |
[00:48:45] 9    = note: move occurs because `x` has type `std::string::String`, which does not implement the `Copy` trait
[00:48:45] 
[00:48:45] 
[00:48:45] The actual stderr differed from the expected stderr.
[00:48:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closures-in-loops/closures-in-loops.stderr
[00:48:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closures-in-loops/closures-in-loops.stderr
[00:48:45] To update references, rerun the tests and pass the `--bless` flag
[00:48:45] To only update this specific test, also pass `--test-args nll/closures-in-loops.rs`
[00:48:45] error: 1 errors occurred comparing output.
[00:48:45] status: exit code: 1
[00:48:45] status: exit code: 1
[00:48:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closures-in-loops.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closures-in-loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closures-in-loops/auxiliary" "-A" "unused"
[00:48:45] ------------------------------------------
[00:48:45] 
[00:48:45] ------------------------------------------
[00:48:45] stderr:
[00:48:45] stderr:
[00:48:45] ------------------------------------------
[00:48:45] {"message":"use of moved value: `x`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n