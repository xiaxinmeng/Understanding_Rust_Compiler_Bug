\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-loan-in-overloaded-op.rs","byte_start":800,"byte_end":801,"line_start":31,"line_end":31,"column_start":15,"column_end":16,"is_primary":false,"text":[{"text":"    let _y = {x} + x.clone(); // the `{x}` forces a move to occur","highlight_start":15,"highlight_end":16}],"label":"value moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-loan-in-overloaded-op.rs","byte_start":805,"byte_end":806,"line_start":31,"line_end":31,"column_start":20,"column_end":21,"is_primary":true,"text":[{"text":"    let _y = {x} + x.clone(); // the `{x}` forces a move to occur","highlight_start":20,"highlight_end":21}],"label":"value borrowed here after move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `x` has type `Foo`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: borrow of moved value: `x`\n  --> /checkout/src/test/ui/borrowck/borrowck-loan-in-overloaded-op.rs:31:20\n   |\nLL |     let _y = {x} + x.clone(); // the `{x}` forces a move to occur\n   |               -    ^ value borrowed here after move\n   |               |\n   |               value moved here\n   |\n   = note: move occurs because `x` has type `Foo`, which does not implement the `Copy` trait\n\n"}
[01:20:27] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:20:27] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] thread '[ui (nll)] ui/borrowck/borrowck-loan-in-overloaded-op.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:27] 
[01:20:27] 
[01:20:27] ---- [ui (nll)] ui/borrowck/borrowck-loan-rcvr.rs stdout ----
[01:20:27] diff of stderr:
[01:20:27] 
[01:20:27] 1 error[E0502]: cannot borrow `p` as mutable because it is also borrowed as immutable
[01:20:27] -   --> $DIR/borrowck-loan-rcvr.rs:34:14
[01:20:27] +   --> $DIR/borrowck-loan-rcvr.rs:33:14
[01:20:27] 3    |
[01:20:27] 4 LL |     p.blockm(|| { //~ ERROR cannot borrow `p` as mutable
[01:20:27] 5    |     - ------ ^^ mutable borrow occurs here
[01:20:27] 
[01:20:27] 10    |         - second borrow occurs due to use of `p` in closure
[01:20:27] 12 error[E0502]: cannot borrow `p` as immutable because it is also borrowed as mutable
[01:20:27] -   --> $DIR/borrowck-loan-rcvr.rs:45:5
[01:20:27] +   --> $DIR/borrowck-loan-rcvr.rs:44:5
[01:20:27] 14    |
[01:20:27] 14    |
[01:20:27] 15 LL |     let l = &mut p;
[01:20:27] 16    |             ------ mutable borrow occurs here
[01:20:27] 
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-rcvr.nll/borrowck-loan-rcvr.nll.stderr
[01:20:27] To update references, rerun the tests and pass the `--bless` flag
[01:20:27] To only update this specific test, also pass `--test-args borrowck/borrowck-loan-rcvr.rs`
[01:20:27] error: 1 errors occurred comparing output.
[01:20:27] status: exit code: 1
[01:20:27] status: exit code: 1
[01:20:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-loan-rcvr.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-rcvr.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-rcvr.nll/auxiliary" "-A" "unused"
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] ------------------------------------------
[01:20:27] stderr:
[01:20:27] stderr:
[01:20:27] ------------------------------------------
[01:20:27] {"message":"cannot borrow `p` as mutable because it is also borrowed as immutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n