\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/E0508.rs","byte_start":543,"byte_end":551,"line_start":15,"line_end":15,"column_start":18,"column_end":26,"is_primary":true,"text":[{"text":"    let _value = array[0];  //~ ERRrowck/borrowck-move-out-of-overloaded-deref.rs stdout ----
[00:52:46] 
[00:52:46] 5    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:52:46] 6    |              |
[00:52:46] 7    |              cannot move out of borrowed content
[00:52:46] 7    |              cannot move out of borrowed content
[00:52:46] -    |              help: consider using a reference instead: `&*Rc::new("hi".to_string())`
[00:52:46] +    |              help: consider removing the `*`: `Rc::new("hi".to_string())`
[00:52:46] 10 error: aborting due to previous error
[00:52:46] 11 
[00:52:46] 
[00:52:46] 
[00:52:46] 
[00:52:46] The actual stderr differed from the expected stderr.
[00:52:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-of-overloaded-deref.nll/borrowck-move-out-of-overloaded-deref.nll.stderr
[00:52:46] To update references, rerun the tests and pass the `--bless` flag
[00:52:46] To only update this specific test, also pass `--test-args borrowck/borrowck-move-out-of-overloaded-deref.rs`
[00:52:46] error: 1 errors occurred comparing output.
[00:52:46] status: exit code: 1
[00:52:46] status: exit code: 1
[00:52:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-out-of-overloaded-deref.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-of-overloaded-deref.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-of-overloaded-deref.nll/auxiliary" "-A" "unused"
[00:52:46] ------------------------------------------
[00:52:46] 
[00:52:46] ------------------------------------------
[00:52:46] stderr:
[00:52:46] stderr:
[00:52:46] ------------------------------------------
[00:52:46] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n