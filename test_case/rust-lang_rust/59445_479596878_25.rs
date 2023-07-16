\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-22560.rs","byte_start":66,"byte_end":145,"line_start":5,"line_end":8,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"type Test = Add +","highlight_start":13,"highlight_end":18},{"text":"            //~^ ERROR E0393","highlight_start":1,"highlight_end":29},{"text":"            //~| ERROR E0191","highlight_start":1,"highlight_end":29},{"text":"            Sub;","highlight_start":1,"highlight_end":16}],"label":"associated type `Output` must be specified","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-22560.rs","byte_start":66,"byte_end":145,"line_start":5,"line_end":8,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"type Test = Add +","highlight_start":13,"highlight_end":18},{"text":"            //~^ ERROR E0393","highlight_start":1,"highlight_end":29},{"text":"            //~| ERROR E0191","highlight_start":1,"highlight_end":29},{"text":"            Sub;","highlight_start":1,"highlight_end":16}],"label":"associated type `Output` must be specified","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0191]: the value of the associated types `Output` (from the trait `std::ops::Add`), `Output` (from the trait `std::ops::Sub`) must be specified\n  --> /checkout/src/test/ui/issues/issue-22560.rs:5:13\n   |\nLL |   type Test = Add +\n   |  _____________^\n   | |_____________|\n   | |\nLL | |             //~^ ERROR E0393\nLL | |             //~| ERROR E0191\nLL | |             Sub;\n   | |               ^\n   | |_______________|\n   | |_______________associated type `Output` must be specified\n   |                 associated type `Output` must be specified\n\n"}
[01:15:35] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:15:35] {"message":"Some errors occurred: E0191, E0225, E0393.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0191, E0225, E0393.\n"}
[01:15:35] 
[01:15:35] ------------------------------------------
[01:15:35] 
[01:15:35] thread '[ui] ui/issues/issue-22560.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:15:35] thread '[ui] ui/issues/issue-22560.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:15:35] 
[01:15:35] ---- [ui] ui/issues/issue-32963.rs stdout ----
[01:15:35] diff of stderr:
[01:15:35] 
[01:15:35] 2   --> $DIR/issue-32963.rs:8:25
[01:15:35] 3    |
[01:15:35] 4 LL |     size_of_copy::<Misc+Copy>();
[01:15:35] -    |                         ^^^^ non-auto additional trait
[01:15:35] +    |                    ---- ^^^^ additional non-auto trait
[01:15:35] +    |                    first non-auto trait
[01:15:35] 6 
[01:15:35] 6 
[01:15:35] 7 error[E0277]: the trait bound `dyn Misc: std::marker::Copy` is not satisfied
[01:15:35] 
[01:15:35] 
[01:15:35] The actual stderr differed from the expected stderr.
[01:15:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32963/issue-32963.stderr
[01:15:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32963/issue-32963.stderr
[01:15:35] To update references, rerun the tests and pass the `--bless` flag
[01:15:35] To only update this specific test, also pass `--test-args issues/issue-32963.rs`
[01:15:35] error: 1 errors occurred comparing output.
[01:15:35] status: exit code: 1
[01:15:35] status: exit code: 1
[01:15:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32963.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32963/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32963/auxiliary" "-A" "unused"
[01:15:35] ------------------------------------------
[01:15:35] 
[01:15:35] ------------------------------------------
[01:15:35] stderr:
[01:15:35] stderr:
[01:15:35] ------------------------------------------
[01:15:35] {"message":"only auto traits can be used as additional traits in a trait object","code":{"code":"E0225","explanation":"\nYou attempted to use multiple types as bounds for a closure or trait object.\nRust does not currently support this. A simple example that causes this error:\n\n