\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/issue-51117.rs","byte_start":760,"byte_end":763,"line_start":19,"line_end":19,"column_start":14,"column_end":17,"is_primary":false,"text":[{"text":"        Some(baz) => {","highlight_start":14,"highlight_end":17}],"label":"first mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/issue-51117.rs","byte_start":782,"byte_end":785,"line_start":20,"line_end":20,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            bar.take(); //~ ERROR cannot borrow","highlight_start":13,"highlight_end":16}],"label":"second mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/issue-51117.rs","byte_start":835,"byte_end":838,"line_start":21,"line_end":21,"column_start":18,"column_end":21,"is_primary":false,"text":[{"text":"            drop(baz);","highlight_start":18,"highlight_end":21}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting this into a `let`-binding","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/borrowck/issue-51117.rs","byte_start":782,"byte_end":792,"line_start":20,"line_end":20,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"            bar.take(); //~ ERROR cannot borrow","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0499]: cannot borrow `*bar` as mutable more than once at a time\n  --> /checkout/src/test/ui/borrowck/issue-51117.rs:20:13\n   |\nLL |         Some(baz) => {\n   |              --- first mutable borrow occurs here\nLL |             bar.take(); //~ ERROR cannot borrow\n   |             ^^^ second mutable borrow occurs here\nLL |             drop(baz);\n   |                  --- borrow later used here\n   |\nnote: consider extracting this into a `let`-binding\n  --> /checkout/src/test/ui/borrowck/issue-51117.rs:20:13\n   |\nLL |             bar.take(); //~ ERROR cannot borrow\n   |             ^^^^^^^^^^\n\n"}
[00:51:17] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:17] {"message":"For more information about this error, try `rustc --explain E0499`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0499`.\n"}
[00:51:17] ------------------------------------------
[00:51:17] 
[00:51:17] thread '[ui (nll)] ui/borrowck/issue-51117.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:51:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:17] 
[00:51:17] ---- [ui (nll)] ui/borrowck/mut-borrow-in-loop.rs stdout ----
[00:51:17] diff of stderr:
[00:51:17] 
[00:51:17] 9    |
[00:51:17] 10 LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
[00:51:17] 11    |      ^^
[00:51:17] + note: consider extracting this into a `let`-binding
[00:51:17] +   --> $DIR/mut-borrow-in-loop.rs:20:13
[00:51:17] +    |
[00:51:17] + LL |             (self.func)(arg) //~ ERROR cannot borrow
[00:51:17] 12 
[00:51:17] 12 
[00:51:17] 13 error[E0499]: cannot borrow `*arg` as mutable more than once at a time
[00:51:17] 
[00:51:17] 21    |
[00:51:17] 21    |
[00:51:17] 22 LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
[00:51:17] 23    |      ^^
[00:51:17] + note: consider extracting this into a `let`-binding
[00:51:17] +   --> $DIR/mut-borrow-in-loop.rs:26:13
[00:51:17] +    |
[00:51:17] + LL |             (self.func)(arg) //~ ERROR cannot borrow
[00:51:17] 24 
[00:51:17] 24 
[00:51:17] 25 error[E0499]: cannot borrow `*arg` as mutable more than once at a time
[00:51:17] 
[00:51:17] 33    |
[00:51:17] 33    |
[00:51:17] 34 LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
[00:51:17] 35    |      ^^
[00:51:17] + note: consider extracting this into a `let`-binding
[00:51:17] +   --> $DIR/mut-borrow-in-loop.rs:33:13
[00:51:17] +    |
[00:51:17] + LL |             (self.func)(arg) //~ ERROR cannot borrow
[00:51:17] 36 
[00:51:17] 37 error: aborting due to 3 previous errors
[00:51:17] 38 
[00:51:17] 
[00:51:17] 
[00:51:17] 
[00:51:17] The actual stderr differed from the expected stderr.
[00:51:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop.nll/mut-borrow-in-loop.nll.stderr
[00:51:17] To update references, rerun the tests and pass the `--bless` flag
[00:51:17] To only update this specific test, also pass `--test-args borrowck/mut-borrow-in-loop.rs`
[00:51:17] error: 1 errors occurred comparing output.
[00:51:17] status: exit code: 1
[00:51:17] status: exit code: 1
[00:51:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/mut-borrow-in-loop.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop.nll/auxiliary" "-A" "unused"
[00:51:17] ------------------------------------------
[00:51:17] 
[00:51:17] ------------------------------------------
[00:51:17] stderr:
[00:51:17] stderr:
[00:51:17] ------------------------------------------
[00:51:17] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:51:17] {"message":"cannot borrow `*arg` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n