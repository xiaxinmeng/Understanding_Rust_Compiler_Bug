\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-52126-assign-op-invariance.rs","byte_start":975,"byte_end":979,"line_start":34,"line_end":34,"column_start":28,"column_end":32,"is_primary":true,"text":[{"text":"        let v: Vec<&str> = line.split_whitespace().collect();","highlight_start":28,"highlight_end":32}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-52126-assign-op-invariance.rs","byte_start":1544,"byte_end":1545,"line_start":48,"line_end":48,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    }","highlight_start":5,"highlight_end":6}],"label":"`line` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-52126-assign-op-invariance.rs","byte_start":1391,"byte_end":1394,"line_start":45,"line_end":45,"column_start":9,"column_end":12,"is_primary":false,"text":[{"text":"        acc += cnt2;","highlight_start":9,"highlight_end":12}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `line` does not live long enough\n  --> /checkout/src/test/ui/issues/issue-52126-assign-op-invariance.rs:34:28\n   |\nLL |         let v: Vec<&str> = line.split_whitespace().collect();\n   |                            ^^^^ borrowed value does not live long enough\n...\nLL |         acc += cnt2;\n   |         --- borrow later used here\n...\nLL |     }\n   |     - `line` dropped here while still borrowed\n\n"}
[01:30:33] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] 
[01:30:33] thread '[ui (nll)] ui/issues/issue-52126-assign-op-invariance.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:30:33] 
[01:30:33] ---- [ui (nll)] ui/span/regions-escape-loop-via-variable.rs stdout ----
[01:30:33] diff of stderr:
[01:30:33] 
[01:30:33] 2   --> $DIR/regions-escape-loop-via-variable.rs:11:13
[01:30:33] 3    |
[01:30:33] 4 LL |         let x = 1 + *p;
[01:30:33] -    |                     -- borrow used here, in later iteration of loop
[01:30:33] +    |                     -- borrow later used here
[01:30:33] 6 LL |         p = &x;
[01:30:33] 7    |             ^^ borrowed value does not live long enough
[01:30:33] 
[01:30:33] 
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-variable.nll/regions-escape-loop-via-variable.nll.stderr
[01:30:33] To update references, rerun the tests and pass the `--bless` flag
[01:30:33] To only update this specific test, also pass `--test-args span/regions-escape-loop-via-variable.rs`
[01:30:33] error: 1 errors occurred comparing output.
[01:30:33] status: exit code: 1
[01:30:33] status: exit code: 1
[01:30:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/regions-escape-loop-via-variable.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-variable.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-variable.nll/auxiliary" "-A" "unused"
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] stderr:
[01:30:33] stderr:
[01:30:33] ------------------------------------------
[01:30:33] {"message":"`x` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n