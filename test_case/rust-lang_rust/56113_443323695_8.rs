\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-mut-borrow-linear-errors.rs","byte_start":1066,"byte_end":1072,"line_start":27,"line_end":27,"column_start":30,"column_end":36,"is_primary":true,"text":[{"text":"            _ => { addr.push(&mut x); } //[ast]~ ERROR [E0499]","highlight_start":30,"highlight_end":36}],"label":"mutable borrow starts here in previous iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0499]: cannot borrow `x` as mutable more than once at a time\n  --> /checkout/src/test/ui/borrowck/borrowck-mut-borrow-linear-errors.rs:27:30\n   |\nLL |             _ => { addr.push(&mut x); } //[ast]~ ERROR [E0499]\n   |                              ^^^^^^ mutable borrow starts here in previous iteration of loop\n\n"}
[00:56:19] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:56:19] {"message":"For more information about this error, try `rustc --explain E0499`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0499`.\n"}
[00:56:19] ------------------------------------------
[00:56:19] 
[00:56:19] thread '[ui (nll)] ui/borrowck/borrowck-mut-borrow-linear-errors.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:56:19] 
[00:56:19] 
[00:56:19] ---- [ui (nll)] ui/borrowck/mut-borrow-outside-loop.rs stdout ----
[00:56:19] diff of stderr:
[00:56:19] 
[00:56:19] 17    |                            ^^^^^^^^^^^^^^^ second mutable borrow occurs here
[00:56:19] 18 LL |         inner_second.use_mut();
[00:56:19] 19 LL |         inner_first.use_mut();
[00:56:19] -    |         ----------- first borrow used here, in later iteration of loop
[00:56:19] +    |         ----------- first borrow later used here
[00:56:19] 22 error: aborting due to 2 previous errors
[00:56:19] 23 
[00:56:19] 
[00:56:19] 
[00:56:19] 
[00:56:19] The actual stderr differed from the expected stderr.
[00:56:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-outside-loop.nll/mut-borrow-outside-loop.nll.stderr
[00:56:19] To update references, rerun the tests and pass the `--bless` flag
[00:56:19] To only update this specific test, also pass `--test-args borrowck/mut-borrow-outside-loop.rs`
[00:56:19] error: 1 errors occurred comparing output.
[00:56:19] status: exit code: 1
[00:56:19] status: exit code: 1
[00:56:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/mut-borrow-outside-loop.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-outside-loop.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-outside-loop.nll/auxiliary" "-A" "unused"
[00:56:19] ------------------------------------------
[00:56:19] 
[00:56:19] ------------------------------------------
[00:56:19] stderr:
[00:56:19] stderr:
[00:56:19] ------------------------------------------
[00:56:19] {"message":"cannot borrow `void` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n