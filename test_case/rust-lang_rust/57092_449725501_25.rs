\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-init-in-fru.rs","byte_start":634,"byte_end":668,"line_start":22,"line_end":22,"column_start":5,"column_end":39,"is_primary":true,"text":[{"text":"    origin = Point { x: 10, ..origin };","highlight_start":5,"highlight_end":39}],"label":"use of possibly uninitialized `origin.y`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0381]: use of possibly uninitialized variable: `origin`\n  --> /checkout/src/test/ui/borrowck/borrowck-init-in-fru.rs:22:5\n   |\nLL |     origin = Point { x: 10, ..origin };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly uninitialized `origin.y`\n\n"}
[01:20:27] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:20:27] {"message":"For more information about this error, try `rustc --explain E0381`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0381`.\n"}
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] thread '[ui (nll)] ui/borrowck/borrowck-init-in-fru.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:27] 
[01:20:27] 
[01:20:27] ---- [ui (nll)] ui/borrowck/borrowck-loan-in-overloaded-op.rs stdout ----
[01:20:27] diff of stderr:
[01:20:27] 
[01:20:27] 6    |               |
[01:20:27] 7    |               value moved here
[01:20:27] 8    |
[01:20:27] -    = note: move occurs because `x` has type `foo`, which does not implement the `Copy` trait
[01:20:27] +    = note: move occurs because `x` has type `Foo`, which does not implement the `Copy` trait
[01:20:27] 11 error: aborting due to previous error
[01:20:27] 12 
[01:20:27] 
[01:20:27] 
[01:20:27] 
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-in-overloaded-op.nll/borrowck-loan-in-overloaded-op.nll.stderr
[01:20:27] To update references, rerun the tests and pass the `--bless` flag
[01:20:27] To only update this specific test, also pass `--test-args borrowck/borrowck-loan-in-overloaded-op.rs`
[01:20:27] error: 1 errors occurred comparing output.
[01:20:27] status: exit code: 1
[01:20:27] status: exit code: 1
[01:20:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-loan-in-overloaded-op.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-in-overloaded-op.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-in-overloaded-op.nll/auxiliary" "-A" "unused"
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] ------------------------------------------
[01:20:27] stderr:
[01:20:27] stderr:
[01:20:27] ------------------------------------------
[01:20:27] {"message":"borrow of moved value: `x`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n