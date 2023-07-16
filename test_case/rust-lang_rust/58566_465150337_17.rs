\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/ch04-00-understanding-ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/moves/moves-based-on-type-cyclic-types-issue-4821.rs","byte_start":328,"byte_end":333,"line_start":10,"line_end":10,"column_start":14,"column_end":19,"is_primary":false,"text":[{"text":"        Some(right) => consume(right),","highlight_start":14,"highlight_end":19}],"label":"value moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/moves/moves-based-on-type-cyclic-types-issue-4821.rs","byte_start":391,"byte_end":395,"line_start":13,"line_end":13,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    consume(node) + r //~ ERROR use of partially moved value: `node`","highlight_start":13,"highlight_end":17}],"label":"value used here after partial move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because value has type `std::boxed::Box<List>`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `node`\n  --> /checkout/src/test/ui/moves/moves-based-on-type-cyclic-types-issue-4821.rs:13:13\n   |\nLL |         Some(right) => consume(right),\n   |              ----- value moved here\n...\nLL |     consume(node) + r //~ ERROR use of partially moved value: `node`\n   |             ^^^^ value used here after partial move\n   |\n   = note: move occurs because value has type `std::boxed::Box<List>`, which does not implement the `Copy` trait\n\n"}
[01:30:22] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[01:30:22] 
[01:30:22] ------------------------------------------
[01:30:22] 
[01:30:22] 
[01:30:22] thread '[ui (nll)] ui/moves/moves-based-on-type-cyclic-types-issue-4821.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:30:22] 
[01:30:22] ---- [ui (nll)] ui/moves/moves-based-on-type-match-bindings.rs stdout ----
[01:30:22] diff of stderr:
[01:30:22] 
[01:30:22] 5    |              - value moved here
[01:30:22] 6 ...
[01:30:22] 7 LL |     touch(&x); //~ ERROR use of partially moved value: `x`
[01:30:22] -    |           ^^ value borrowed here after move
[01:30:22] +    |           ^^ value borrowed here after partial move
[01:30:22] 9    |
[01:30:22] 10    = note: move occurs because `x.f` has type `std::string::String`, which does not implement the `Copy` trait
[01:30:22] 
[01:30:22] 
[01:30:22] The actual stderr differed from the expected stderr.
[01:30:22] The actual stderr differed from the expected stderr.
[01:30:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-match-bindings.nll/moves-based-on-type-match-bindings.nll.stderr
[01:30:22] To update references, rerun the tests and pass the `--bless` flag
[01:30:22] To only update this specific test, also pass `--test-args moves/moves-based-on-type-match-bindings.rs`
[01:30:22] error: 1 errors occurred comparing output.
[01:30:22] status: exit code: 1
[01:30:22] status: exit code: 1
[01:30:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/moves-based-on-type-match-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-match-bindings.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-match-bindings.nll/auxiliary" "-A" "unused"
[01:30:22] ------------------------------------------
[01:30:22] 
[01:30:22] ------------------------------------------
[01:30:22] stderr:
[01:30:22] stderr:
[01:30:22] ------------------------------------------
[01:30:22] {"message":"borrow of moved value: `x`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n