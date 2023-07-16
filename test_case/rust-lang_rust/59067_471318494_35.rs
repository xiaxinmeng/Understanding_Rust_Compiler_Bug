\n"},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/generator/yield-in-args.rs","byte_start":114,"byte_end":119,"line_start":8,"line_end":8,"column_start":17,"column_end":22,"is_primary":false,"text":[{"text":"        foo(&b, yield); //~ ERROR","highlight_start":17,"highlight_end":22}],"label":"possible yield occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/yield-in-args.rs","byte_start":110,"byte_end":112,"line_start":8,"line_end":8,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        foo(&b, yield); //~ ERROR","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this error has been downgraded to a warning for backwards compatibility with previous releases","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"this represents potential undefined behavior in your code and this warning will become a hard error in the future","code":null,"level":"warning","spans":[],"children":[],"rendered":null}],"rendered":"warning[E0626]: borrow may still be in use when generator yields\n  --> /checkout/src/test/ui/generator/yield-in-args.rs:8:13\n   |\nLL |         foo(&b, yield); //~ ERROR\n   |             ^^  ----- possible yield occurs here\n   |\n   = warning: this error has been downgraded to a warning for backwards compatibility with previous releases\n   = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future\n\n"}
[01:38:31] ------------------------------------------
[01:38:31] 
[01:38:31] thread '[ui (nll)] ui/generator/yield-in-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3319:9
[01:38:31] 
[01:38:31] 
[01:38:31] ---- [ui (nll)] ui/issues/issue-27592.rs stdout ----
[01:38:31] 
[01:38:31] error: ui test compiled successfully!
[01:38:31] status: exit code: 0
[01:38:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27592.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27592.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27592.nll/auxiliary" "-A" "unused"
[01:38:31] ------------------------------------------
[01:38:31] 
[01:38:31] ------------------------------------------
[01:38:31] stderr:
[01:38:31] stderr:
[01:38:31] ------------------------------------------
[01:38:31] {"message":"cannot return value referencing temporary value","code":{"code":"E0515","explanation":"\nCannot return value that references local variable\n\nLocal variables, function parameters and temporaries are all dropped before the\nend of the function body. So a reference to them cannot be returned.\n\n