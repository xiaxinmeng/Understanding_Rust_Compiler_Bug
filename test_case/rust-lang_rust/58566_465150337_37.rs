\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/ch04-00-understanding-ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/ref-suggestion.rs","byte_start":224,"byte_end":225,"line_start":13,"line_end":13,"column_start":15,"column_end":16,"is_primary":false,"text":[{"text":"        (Some(y), ()) => {},","highlight_start":15,"highlight_end":16}],"label":"value moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/ref-suggestion.rs","byte_start":266,"byte_end":267,"line_start":16,"line_end":16,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    x; //~ ERROR use of partially moved value","highlight_start":5,"highlight_end":6}],"label":"value used here after partial move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because value has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `x`\n  --> /checkout/src/test/ui/ref-suggestion.rs:16:5\n   |\nLL |         (Some(y), ()) => {},\n   |               - value moved here\n...\nLL |     x; //~ ERROR use of partially moved value\n   |     ^ value used here after partial move\n   |\n   = note: move occurs because value has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait\n\n"}
[01:30:22] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[01:30:22] 
[01:30:22] ------------------------------------------
[01:30:22] 
[01:30:22] 
[01:30:22] thread '[ui (nll)] ui/ref-suggestion.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:30:22] 
[01:30:22] ---- [ui (nll)] ui/unsized-locals/borrow-after-move.rs stdout ----
[01:30:22] diff of stderr:
[01:30:22] 
[01:30:22] 5    |                 -- value moved here
[01:30:22] 6 LL |         drop_unsized(y);
[01:30:22] 7 LL |         println!("{}", &x);
[01:30:22] -    |                        ^^ value borrowed here after move
[01:30:22] +    |                        ^^ value borrowed here after partial move
[01:30:22] 9    |
[01:30:22] 10    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
[01:30:22] 
[01:30:22] 
[01:30:22] 27    |                 -- value moved here
[01:30:22] 28 LL |         y.foo();
[01:30:22] 29 LL |         println!("{}", &x);
[01:30:22] -    |                        ^^ value borrowed here after move
[01:30:22] +    |                        ^^ value borrowed here after partial move
[01:30:22] 31    |
[01:30:22] 32    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
[01:30:22] 
[01:30:22] 48 LL |         x.foo();
[01:30:22] 48 LL |         x.foo();
[01:30:22] 49    |         - value moved here
[01:30:22] 50 LL |         println!("{}", &x);
[01:30:22] -    |                        ^^ value borrowed here after move
[01:30:22] +    |                        ^^ value borrowed here after partial move
[01:30:22] 52    |
[01:30:22] 53    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
[01:30:22] 
[01:30:22] 
[01:30:22] The actual stderr differed from the expected stderr.
[01:30:22] The actual stderr differed from the expected stderr.
[01:30:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move.nll/borrow-after-move.nll.stderr
[01:30:22] To update references, rerun the tests and pass the `--bless` flag
[01:30:22] To only update this specific test, also pass `--test-args unsized-locals/borrow-after-move.rs`
[01:30:22] error: 1 errors occurred comparing output.
[01:30:22] status: exit code: 1
[01:30:22] status: exit code: 1
[01:30:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/borrow-after-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move.nll/auxiliary" "-A" "unused"
[01:30:22] ------------------------------------------
[01:30:22] 
[01:30:22] ------------------------------------------
[01:30:22] stderr:
[01:30:22] stderr:
[01:30:22] ------------------------------------------
[01:30:22] {"message":"borrow of moved value: `x`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n