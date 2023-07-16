\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/book/first-edition/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-move-out-of-overloaded-deref.rs","byte_start":514,"byte_end":540,"line_start":14,"line_end":14,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"    let _x = *Rc::new(\"hi\".to_string());","highlight_start":14,"highlight_end:52:46] 
[00:52:46] thread '[ui (nll)] ui/borrowck/borrowck-move-out-of-overloaded-deref.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:52:46] ---- [ui (nll)] ui/error-codes/E0509.rs stdout ----
[00:52:46] diff of stderr:
[00:52:46] 
[00:52:46] 5    |                       ^^^^^^^^^^^^^^^^^
[00:52:46] 5    |                       ^^^^^^^^^^^^^^^^^
[00:52:46] 6    |                       |
[00:52:46] 7    |                       cannot move out of here
[00:52:46] -    |                       help: consider using a reference instead: `&drop_struct.fancy`
[00:52:46] +    |                       help: consider borrowing here: `&drop_struct.fancy`
[00:52:46] 10 error: aborting due to previous error
[00:52:46] 11 
[00:52:46] 
[00:52:46] 
[00:52:46] 
[00:52:46] The actual stderr differed from the expected stderr.
[00:52:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0509.nll/E0509.nll.stderr
[00:52:46] To update references, rerun the tests and pass the `--bless` flag
[00:52:46] To only update this specific test, also pass `--test-args error-codes/E0509.rs`
[00:52:46] error: 1 errors occurred comparing output.
[00:52:46] status: exit code: 1
[00:52:46] status: exit code: 1
[00:52:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0509.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0509.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0509.nll/auxiliary" "-A" "unused"
[00:52:46] ------------------------------------------
[00:52:46] 
[00:52:46] ------------------------------------------
[00:52:46] stderr:
[00:52:46] stderr:
[00:52:46] ------------------------------------------
[00:52:46] {"message":"cannot move out of type `DropStruct`, which implements the `Drop` trait","code":{"code":"E0509","explanation":"\nThis error occurs when an attempt is made to move out of a value whose type\nimplements the `Drop` trait.\n\nExample of erroneous code:\n\n