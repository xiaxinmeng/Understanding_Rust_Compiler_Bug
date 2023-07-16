\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-52126-assign-op-invariance.rs","byte_start":1442,"byte_end":1446,"line_start":44,"line_end":44,"column_start":28,"column_end":32,"is_primary":true,"text":[{"text":"        let v: Vec<&str> = line.split_whitespace().collect();","highlight_start":28,"highlight_end":32}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-52126-assign-op-invariance.rs","byte_start":2011,"byte_end":2012,"line_start":58,"line_end":58,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    }","highlight_start":5,"highlight_end":6}],"label":"`line` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-52126-assign-op-invariance.rs","byte_start":1858,"byte_end":1861,"line_start":55,"line_end":55,"column_start":9,"column_end":12,"is_primary":false,"text":[{"text":"        acc += cnt2;","highlight_start":9,"highlight_end":12}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `line` does not live long enough\n  --> /checkout/src/test/ui/issues/issue-52126-assign-op-invariance.rs:44:28\n   |\nLL |         let v: Vec<&str> = line.split_whitespace().collect();\n   |                            ^^^^ borrowed value does not live long enough\n...\nLL |         acc += cnt2;\n   |         --- borrow later used here\n...\nLL |     }\n   |     - `line` dropped here while still borrowed\n\n"}
[00:56:19] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:56:19] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:56:19] ------------------------------------------
[00:56:19] 
[00:56:19] thread '[ui (nll)] ui/issues/issue-52126-assign-op-invariance.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:56:19] 
[00:56:19] 
[00:56:19] ---- [ui (nll)] ui/rfc-2005-default-binding-mode/borrowck-issue-49631.rs stdout ----
[00:56:19] diff of stderr:
[00:56:19] 
[00:56:19] 7    |         ^^^^^^^^^^^^ mutable borrow occurs here
[00:56:19] 8 LL |         //~^ ERROR cannot borrow `foo` as mutable
[00:56:19] 9 LL |         println!("foo={:?}", *string);
[00:56:19] -    |                              ------- immutable borrow used here, in later iteration of loop
[00:56:19] 11 
[00:56:19] 12 error: aborting due to previous error
[00:56:19] 13 
[00:56:19] 
[00:56:19] 
[00:56:19] 
[00:56:19] The actual stderr differed from the expected stderr.
[00:56:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/borrowck-issue-49631.nll/borrowck-issue-49631.nll.stderr
[00:56:19] To update references, rerun the tests and pass the `--bless` flag
[00:56:19] To only update this specific test, also pass `--test-args rfc-2005-default-binding-mode/borrowck-issue-49631.rs`
[00:56:19] error: 1 errors occurred comparing output.
[00:56:19] status: exit code: 1
[00:56:19] status: exit code: 1
[00:56:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/borrowck-issue-49631.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/borrowck-issue-49631.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/borrowck-issue-49631.nll/auxiliary" "-A" "unused"
[00:56:19] ------------------------------------------
[00:56:19] 
[00:56:19] ------------------------------------------
[00:56:19] stderr:
[00:56:19] stderr:
[00:56:19] ------------------------------------------
[00:56:19] {"message":"cannot borrow `foo` as mutable because it is also borrowed as immutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n