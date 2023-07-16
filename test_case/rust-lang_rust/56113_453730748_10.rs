\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-mut-borrow-linear-errors.rs","byte_start":599,"byte_end":605,"line_start":17,"line_end":17,"column_start":30,"column_end":36,"is_primary":true,"text":[{"text":"            _ => { addr.push(&mut x); } //[ast]~ ERROR [E0499]","highlight_start":30,"highlight_end":36}],"label":"mutable borrow starts here in previous iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0499]: cannot borrow `x` as mutable more than once at a time\n  --> /checkout/src/test/ui/borrowck/borrowck-mut-borrow-linear-errors.rs:17:30\n   |\nLL |             _ => { addr.push(&mut x); } //[ast]~ ERROR [E0499]\n   |                              ^^^^^^ mutable borrow starts here in previous iteration of loop\n\n"}
[01:30:33] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] thread '[ui (nll)] ui/borrowck/borrowck-mut-borrow-linear-errors.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:30:33] thread '[ui (nll)] ui/borrowck/borrowck-mut-borrow-linear-errors.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:30:33] 
[01:30:33] ---- [ui (nll)] ui/issues/issue-52126-assign-op-invariance.rs stdout ----
[01:30:33] diff of stderr:
[01:30:33] 
[01:30:33] 5    |                            ^^^^ borrowed value does not live long enough
[01:30:33] 6 ...
[01:30:33] 7 LL |         acc += cnt2;
[01:30:33] -    |         --- borrow used here, in later iteration of loop
[01:30:33] 9 ...
[01:30:33] 10 LL |     }
[01:30:33] 10 LL |     }
[01:30:33] 11    |     - `line` dropped here while still borrowed
[01:30:33] 
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52126-assign-op-invariance.nll/issue-52126-assign-op-invariance.nll.stderr
[01:30:33] To update references, rerun the tests and pass the `--bless` flag
[01:30:33] To only update this specific test, also pass `--test-args issues/issue-52126-assign-op-invariance.rs`
[01:30:33] error: 1 errors occurred comparing output.
[01:30:33] status: exit code: 1
[01:30:33] status: exit code: 1
[01:30:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52126-assign-op-invariance.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52126-assign-op-invariance.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52126-assign-op-invariance.nll/auxiliary" "-A" "unused"
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] stderr:
[01:30:33] stderr:
[01:30:33] ------------------------------------------
[01:30:33] {"message":"`line` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n