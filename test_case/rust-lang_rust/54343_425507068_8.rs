\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/liveness/liveness-move-in-loop.rs","byte_start":682,"byte_end":683,"line_start":21,"line_end":21,"column_start":25,"column_end":26,"is_primary":true,"text":[{"text":"                    x = y; //~ ERROR use of moved value","highlight_start":25,"highlight_end":26}],"label":"value moved here, in previous iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `y` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `y`\n  --> /checkout/src/test/ui/liveness/liveness-move-in-loop.rs:21:25\n   |\nLL |                     x = y; //~ ERROR use of moved value\n   |                         ^ value moved here, in previous iteration of loop\n   |\n   = note: move occurs because `y` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait\n\n"}
[00:52:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:52:30] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[00:52:30] ------------------------------------------
[00:52:30] 
[00:52:30] thread '[ui (nll)] ui/liveness/liveness-move-in-loop.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:52:30] 
[00:52:30] 
[00:52:30] ---- [ui (nll)] ui/moves/move-in-guard-2.rs stdout ----
[00:52:30] diff of stderr:
[00:52:30] 
[00:52:30] 2   --> $DIR/move-in-guard-2.rs:20:24
[00:52:30] 3    |
[00:52:30] 4 LL |         (_, 2) if take(x) => (), //~ ERROR use of moved value: `x`
[00:52:30] -    |                        ^ value moved here in previous iteration of loop
[00:52:30] +    |                        ^ value moved here, in previous iteration of loop
[00:52:30] 6    |
[00:52:30] 7    = note: move occurs because `x` has type `std::boxed::Box<i32>`, which does not implement the `Copy` trait
[00:52:30] 
[00:52:30] 
[00:52:30] The actual stderr differed from the expected stderr.
[00:52:30] The actual stderr differed from the expected stderr.
[00:52:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-in-guard-2.nll/move-in-guard-2.nll.stderr
[00:52:30] To update references, rerun the tests and pass the `--bless` flag
[00:52:30] To only update this specific test, also pass `--test-args moves/move-in-guard-2.rs`
[00:52:30] error: 1 errors occurred comparing output.
[00:52:30] status: exit code: 1
[00:52:30] status: exit code: 1
[00:52:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/move-in-guard-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-in-guard-2.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-in-guard-2.nll/auxiliary" "-A" "unused"
[00:52:30] ------------------------------------------
[00:52:30] 
[00:52:30] ------------------------------------------
[00:52:30] stderr:
[00:52:30] stderr:
[00:52:30] ------------------------------------------
[00:52:30] {"message":"use of moved value: `x`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n