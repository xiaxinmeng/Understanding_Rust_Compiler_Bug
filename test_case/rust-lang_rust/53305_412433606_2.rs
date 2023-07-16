\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/two-phase-multi-mut.rs","byte_start":617,"byte_end":625,"line_start":23,"line_end":23,"column_start":16,"column_end":24,"is_primary":false,"text":[{"text":"    foo.method(&mut foo);","highlight_start":16,"highlight_end":24}],"label":"first mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/two-phase-multi-mut.rs","byte_start":606,"byte_end":626,"line_start":23,"line_end":23,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    foo.method(&mut foo);","highlight_start":5,"highlight_end":25}],"label":"second mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/two-phase-multi-mut.rs","byte_start":606,"byte_end":626,"line_start":23,"line_end":23,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    foo.method(&mut foo);","highlight_start":5,"highlight_end":25}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0499]: cannot borrow `foo` as mutable more than once at a time\n  --> /checkout/src/test/ui/borrowck/two-phase-multi-mut.rs:23:5\n   |\nLL |     foo.method(&mut foo);\n   |     ^^^^^^^^^^^--------^\n   |     |          |\n   |     |          first mutable borrow occurs here\n   |     second mutable borrow occurs here\n   |     borrow later used here\n\n"}
[00:58:42] {"message":"cannot borrow `foo` as mutable morelse,"text":[{"text":"    foo.method(&mut foo);","highlight_start":5,"highlight_end":25}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting this into a `let`-binding","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/borrowck/two-phase-multi-mut.rs","byte_start":606,"byte_end":626,"line_start":23,"line_end":23,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    foo.method(&mut foo);","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0499]: cannot borrow `foo` as mutable more than once at a time\n  --> /checkout/src/test/ui/borrowck/two-phase-multi-mut.rs:23:16\n   |\nLL |     foo.method(&mut foo);\n   |     -----------^^^^^^^^-\n   |     |          |\n   |     |          second mutable borrow occurs here\n   |     first mutable borrow occurs here\n   |     borrow later used here\n   |\nnote: consider extracting this into a `let`-binding\n  --> /checkout/src/test/ui/borrowck/two-phase-multi-mut.rs:23:5\n   |\nLL |     foo.method(&mut foo);\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:58:42] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:58:42] {"message":"For more information about this error, try `rustc --explain E0499`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0499`.\n"}
[00:58:42] ------------------------------------------
[00:58:42] 
[00:58:42] thread '[ui] ui/borrowck/two-phase-multi-mut.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:58:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:42] 
[00:58:42] ---- [ui] ui/issue-47646.rs stdout ----
[00:58:42] diff of stderr:
[00:58:42] 
[00:58:42] 6 ...
[00:58:42] 7 LL |             println!("{:?}", heap); //~ ERROR cannot borrow `heap` as immutable
[00:58:42] +    |
[00:58:42] +    |
[00:58:42] + note: consider extracting this into a `let`-binding
[00:58:42] +   --> $DIR/issue-47646.rs:22:22
[00:58:42] +    |
[00:58:42] + LL |             println!("{:?}", heap); //~ ERROR cannot borrow `heap` as immutable
[00:58:42] 9 
[00:58:42] 10 error: aborting due to previous error
[00:58:42] 11 
[00:58:42] 
[00:58:42] 
[00:58:42] 
[00:58:42] The actual stderr differed from the expected stderr.
[00:58:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-47646/issue-47646.stderr
[00:58:42] To update references, rerun the tests and pass the `--bless` flag
[00:58:42] To only update this specific test, also pass `--test-args issue-47646.rs`
[00:58:42] error: 1 errors occurred comparing output.
[00:58:42] status: exit code: 1
[00:58:42] status: exit code: 1
[00:58:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-47646.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-47646/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-47646/auxiliary" "-A" "unused"
[00:58:42] ------------------------------------------
[00:58:42] 
[00:58:42] ------------------------------------------
[00:58:42] stderr:
[00:58:42] stderr:
[00:58:42] ------------------------------------------
[00:58:42] {"message":"cannot borrow `heap` as immutable because it is also borrowed as mutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n