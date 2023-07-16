\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-47971.rs","byte_start":614,"byte_end":616,"line_start":19,"line_end":19,"column_start":19,"column_end":21,"is_primary":true,"text":[{"text":"static T: S = S(g(&T), 0);","highlight_start":19,"highlight_end":21}],"label":"cyclic reference","suggested_replacement":null,"expansion":null}],"children":[{"message":"the cycle begins when const-evaluating `T`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-47971.rs","byte_start":596,"byte_end":622,"line_start":19,"line_end":19,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"static T: S = S(g(&T), 0);","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which then again requires const-evaluating `T`, completing the cycle.","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0391]: cyclic dependency detected\n  --> /checkout/src/test/ui/const-eval/issue-47971.rs:19:19\n   |\nLL | static T: S = S(g(&T), 0);\n   |                   ^^ cyclic reference\n   |\nnote: the cycle begins when const-evaluating `T`...\n  --> /checkout/src/test/ui/const-eval/issue-47971.rs:19:1\n   |\nLL | static T: S = S(g(&T), 0);\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = note: ...which then again requires const-evaluating `T`, completing the cycle.\n\n"}
[00:42:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:42:07] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:42:07] 
[00:42:07] ------------------------------------------
[00:42:07] 
[00:42:07] thread '[ui] ui/const-eval/issue-47971.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2903:9
[00:42:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:07] 
[00:42:07] 
[00:42:07] failures:
[00:42:07]     [ui] ui/const-eval/issue-47971.rs
[00:42:07] 
[00:42:07] test result: FAILED. 1281 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out
[00:42:07] 
