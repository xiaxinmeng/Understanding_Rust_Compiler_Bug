\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-21950.rs","byte_start":91,"byte_end":94,"line_start":7,"line_end":7,"column_start":14,"column_end":17,"is_primary":true,"text":[{"text":"            &Add;","highlight_start":14,"highlight_end":17}],"label":"associated type `Output` must be specified","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0191]: the value of the associated type `Output` (from the trait `std::ops::Add`) must be specified\n  --> /checkout/src/test/ui/issues/issue-21950.rs:7:14\n   |\nLL |             &Add;\n   |              ^^^ associated type `Output` must be specified\n\n"}
[01:15:17] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:15:17] {"message":"Some errors occurred: E0191, E0393.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0191, E0393.\n"}
[01:15:17] 
[01:15:17] ------------------------------------------
[01:15:17] 
[01:15:17] thread '[ui] ui/issues/issue-21950.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:15:17] thread '[ui] ui/issues/issue-21950.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:15:17] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:17] 
[01:15:17] ---- [ui] ui/issues/issue-22560.rs stdout ----
[01:15:17] diff of stderr:
[01:15:17] 
[01:15:17] - error[E0393]: the type parameter `RHS` must be explicitly specified
[01:15:17] + error[E0393]: the type parameter `Rhs` must be explicitly specified
[01:15:17] 3    |
[01:15:17] 4 LL | type Test = Add +
[01:15:17] 
[01:15:17] -    |             ^^^ missing reference to `RHS`
[01:15:17] -    |             ^^^ missing reference to `RHS`
[01:15:17] +    |             ^^^ missing reference to `Rhs`
[01:15:17] 6    |
[01:15:17] 7    = note: because of the default `Self` reference, type parameters must be specified on object types
[01:15:17] 
[01:15:17] 
[01:15:17] - error[E0393]: the type parameter `RHS` must be explicitly specified
[01:15:17] + error[E0393]: the type parameter `Rhs` must be explicitly specified
[01:15:17] 11    |
[01:15:17] 12 LL |             Sub;
[01:15:17] 
[01:15:17] -    |             ^^^ missing reference to `RHS`
[01:15:17] -    |             ^^^ missing reference to `RHS`
[01:15:17] +    |             ^^^ missing reference to `Rhs`
[01:15:17] 14    |
[01:15:17] 15    = note: because of the default `Self` reference, type parameters must be specified on object types
[01:15:17] 
[01:15:17] 
[01:15:17] The actual stderr differed from the expected stderr.
[01:15:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/issue-22560.stderr
[01:15:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/issue-22560.stderr
[01:15:17] To update references, rerun the tests and pass the `--bless` flag
[01:15:17] To only update this specific test, also pass `--test-args issues/issue-22560.rs`
[01:15:17] error: 1 errors occurred comparing output.
[01:15:17] status: exit code: 1
[01:15:17] status: exit code: 1
[01:15:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22560.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/auxiliary" "-A" "unused"
[01:15:17] ------------------------------------------
[01:15:17] 
[01:15:17] ------------------------------------------
[01:15:17] stderr:
[01:15:17] stderr:
[01:15:17] ------------------------------------------
[01:15:17] {"message":"the type parameter `Rhs` must be explicitly specified","code":{"code":"E0393","explanation":"\nA type parameter which references `Self` in its default value was not specified.\nExample of erroneous code:\n\n