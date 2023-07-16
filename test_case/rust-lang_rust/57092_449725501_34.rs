\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-loan-rcvr.rs","byte_start":1151,"byte_end":1152,"line_start":44,"line_end":44,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    p.impurem(); //~ ERROR cannot borrow","highlight_start":5,"highlight_end":6}],"label":"immutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-loan-rcvr.rs","byte_start":1139,"byte_end":1145,"line_start":43,"line_end":43,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let l = &mut p;","highlight_start":13,"highlight_end":19}],"label":"mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-loan-rcvr.rs","byte_start":1193,"byte_end":1201,"line_start":46,"line_end":46,"column_start":5,"column_end":13,"is_primary":false,"text":[{"text":"    l.x += 1;","highlight_start":5,"highlight_end":13}],"label":"mutable borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `p` as immutable because it is also borrowed as mutable\n  --> /checkout/src/test/ui/borrowck/borrowck-loan-rcvr.rs:44:5\n   |\nLL |     let l = &mut p;\n   |             ------ mutable borrow occurs here\nLL |     p.impurem(); //~ ERROR cannot borrow\n   |     ^ immutable borrow occurs here\nLL | \nLL |     l.x += 1;\n   |     -------- mutable borrow later used here\n\n"}
[01:20:27] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:20:27] {"message":"For more information about this error, try `rustc --explain E0502`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0502`.\n"}
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] thread '[ui (nll)] ui/borrowck/borrowck-loan-rcvr.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:27] 
[01:20:27] 
[01:20:27] ---- [ui (nll)] ui/borrowck/borrowck-no-cycle-in-exchange-heap.rs stdout ----
[01:20:27] diff of stderr:
[01:20:27] 
[01:20:27] 1 error[E0505]: cannot move out of `x` because it is borrowed
[01:20:27] 2   --> $DIR/borrowck-no-cycle-in-exchange-heap.rs:26:15
[01:20:27] 3    |
[01:20:27] - LL |       cycle::node(ref mut y) => {
[01:20:27] + LL |       Cycle::Node(ref mut y) => {
[01:20:27] 5    |                   --------- borrow of `x.0` occurs here
[01:20:27] 6 LL |         y.a = x; //~ ERROR cannot move out of
[01:20:27] 7    |         ---   ^ move out of `x` occurs here
[01:20:27] 
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap.nll/borrowck-no-cycle-in-exchange-heap.nll.stderr
[01:20:27] To update references, rerun the tests and pass the `--bless` flag
[01:20:27] To only update this specific test, also pass `--test-args borrowck/borrowck-no-cycle-in-exchange-heap.rs`
[01:20:27] error: 1 errors occurred comparing output.
[01:20:27] status: exit code: 1
[01:20:27] status: exit code: 1
[01:20:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap.nll/auxiliary" "-A" "unused"
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] ------------------------------------------
[01:20:27] stderr:
[01:20:27] stderr:
[01:20:27] ------------------------------------------
[01:20:27] {"message":"cannot move out of `x` because it is borrowed","code":{"code":"E0505","explanation":"\nA value was moved out while it was still borrowed.\n\nErroneous code example:\n\n