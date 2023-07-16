\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/mut-borrow-outside-loop.rs","byte_start":823,"byte_end":838,"line_start":24,"line_end":24,"column_start":27,"column_end":42,"is_primary":false,"text":[{"text":"        let inner_first = &mut inner_void;","highlight_start":27,"highlight_end":42}],"label":"first mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/mut-borrow-outside-loop.rs","byte_start":867,"byte_end":882,"line_start":25,"line_end":25,"column_start":28,"column_end":43,"is_primary":true,"text":[{"text":"        let inner_second = &mut inner_void; //~ ERROR cannot borrow","highlight_start":28,"highlight_end":43}],"label":"second mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/mut-borrow-outside-loop.rs","byte_start":948,"byte_end":959,"line_start":27,"line_end":27,"column_start":9,"column_endowed value does not live long enough
[00:56:19] 6 ...
[00:56:19] 7 LL |         acc += cnt2;
[00:56:19] -    |         --- borrow used here, in later iteration of loop
[00:56:19] 9 ...
[00:56:19] 9 ...
[00:56:19] 10 LL |     }
[00:56:19] 11    |     - `line` dropped here while still borrowed
[00:56:19] 
[00:56:19] The actual stderr differed from the expected stderr.
[00:56:19] The actual stderr differed from the expected stderr.
[00:56:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52126-assign-op-invariance.nll/issue-52126-assign-op-invariance.nll.stderr
[00:56:19] To update references, rerun the tests and pass the `--bless` flag
[00:56:19] To only update this specific test, also pass `--test-args issues/issue-52126-assign-op-invariance.rs`
[00:56:19] error: 1 errors occurred comparing output.
[00:56:19] status: exit code: 1
[00:56:19] status: exit code: 1
[00:56:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52126-assign-op-invariance.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52126-assign-op-invariance.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52126-assign-op-invariance.nll/auxiliary" "-A" "unused"
[00:56:19] ------------------------------------------
[00:56:19] 
[00:56:19] ------------------------------------------
[00:56:19] stderr:
[00:56:19] stderr:
[00:56:19] ------------------------------------------
[00:56:19] {"message":"`line` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n