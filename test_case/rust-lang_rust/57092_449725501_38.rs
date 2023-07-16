\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/stable/book/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap.rs","byte_start":709,"byte_end":718,"line_start":25,"line_end":25,"column_start":19,"column_end":28,"is_primary":false,"text":[{"text":"      Cycle::Node(ref mut y) => {","highlight_start":19,"highlight_end":28}],"label":"borrow of `x.0` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap.rs","byte_start":739,"byte_end":740,"line_start":26,"line_end":26,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"        y.a = x; //~ ERROR cannot move out of","highlight_start":15,"highlight_end":16}],"label":"move out of `x` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap.rs","byte_start":733,"byte_end":736,"line_start":26,"line_end":26,"column_start":9,"column_end":12,"is_primary":false,"text":[{"text":"        y.a = x; //~ ERROR cannot move out of","highlight_start":9,"highlight_end":12}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0505]: cannot move out of `x` because it is borrowed\n  --> /checkout/src/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap.rs:26:15\n   |\nLL |       Cycle::Node(ref mut y) => {\n   |                   --------- borrow of `x.0` occurs here\nLL |         y.a = x; //~ ERROR cannot move out of\n   |         ---   ^ move out of `x` occurs here\n   |         |\n   |         borrow later used here\n\n"}
[01:20:27] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:20:27] {"message":"For more information about this error, try `rustc --explain E0505`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0505`.\n"}
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] thread '[ui (nll)] ui/borrowck/borrowck-no-cycle-in-exchange-heap.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:27] 
[01:20:27] 
[01:20:27] ---- [ui (nll)] ui/issues/issue-2590.rs stdout ----
[01:20:27] diff of stderr:
[01:20:27] 
[01:20:27] 1 error[E0507]: cannot move out of borrowed content
[01:20:27] -   --> $DIR/issue-2590.rs:22:9
[01:20:27] +   --> $DIR/issue-2590.rs:21:9
[01:20:27] 3    |
[01:20:27] 4 LL |         self.tokens //~ ERROR cannot move out of borrowed content
[01:20:27] 5    |         ^^^^^^^^^^^ cannot move out of borrowed content
[01:20:27] 
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2590.nll/issue-2590.nll.stderr
[01:20:27] To update references, rerun the tests and pass the `--bless` flag
[01:20:27] To only update this specific test, also pass `--test-args issues/issue-2590.rs`
[01:20:27] error: 1 errors occurred comparing output.
[01:20:27] status: exit code: 1
[01:20:27] status: exit code: 1
[01:20:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2590.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2590.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2590.nll/auxiliary" "-A" "unused"
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] ------------------------------------------
[01:20:27] stderr:
[01:20:27] stderr:
[01:20:27] ------------------------------------------
[01:20:27] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n