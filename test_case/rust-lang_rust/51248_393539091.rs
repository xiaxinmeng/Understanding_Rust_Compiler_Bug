plain
[00:45:30] ....................................................................................................
[00:45:35] ....................................................................................................
[00:45:40] ....................................................................................................
[00:45:46] .............................................................................................i......
[00:45:51] ...........FF..FFFFFFFF.F...F................................F........i.............................
[00:46:02] ....................................................................................................
[00:46:08] ....................................................................................................
[00:46:08] ....................................................................................................
[00:46:12] ..i.................iiiiiiiii...................................................
[00:46:12] 
[00:46:12] ---- [ui] ui/nll/closure-requirements/escape-argument-callee.rs stdout ----
[00:46:12] diff of stderr:
[00:46:12] 
[00:46:12] 
[00:46:12] 18    |
[00:46:12] 19    = note: defining type: DefId(0/1:9 ~ escape_argument_callee[317d]::test[0]::{{closure}}[0]) with closure substs [
[00:46:12] 20                i16,
[00:46:12] -                for<'r, 's, 't0> extern "rust-call" fn((&ReLateBound(DebruijnIndex { index: 0 }, BrNamed(crate0:DefIndex(0:0), 'r)) mut &ReLateBound(DebruijnIndex { index: 0 }, BrNamed(crate0:DefIndex(0:0), 's)) i32, &ReLateBound(DebruijnIndex { index: 0 }, BrNamed(crate0:DefIndex(0:0), 't0)) i32))
[00:46:12] +                for<'r, 's, 't0> extern "rust-call" fn((&ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 'r)) mut &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 's)) i32, &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 't0)) i32))
[00:46:12] 23 
[00:46:12] 24 note: No external requirements
[00:46:12] 
[00:46:12] 
[00:46:12] 
[00:46:12] The actual stderr differed from the expected stderr.
[00:46:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee/escape-argument-callee.stderr
[00:46:12] To update references, rerun the tests and pass the `--bless` flag
[00:46:12] To only update this specific test, also pass `--test-args nll/closure-requirements/escape-argument-callee.rs`
[00:46:12] error: 1 errors occurred comparing output.
[00:46:12] status: exit code: 101
[00:46:12] status: exit code: 101
[00:46:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee/auxiliary" "-A" "unused"
[00:46:12] ------------------------------------------
[00:46:12] 
[00:46:12] 
[00:46:12] -------------------------------------nt_callee[317d]::test[0]::{{closure}}[0]), BrAnon(3))` does not outlive free region `ReFree(DefId(0/1:9 ~ escape_argument_callee[317d]::test[0]::{{closure}}[0]), BrAnon(2))`\n  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs:36:45\n   |\nLL |         let mut closure = expect_sig(|p, y| *p = y);\n   |                                             ^^^^^^\n\n"}
[00:46:12] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs","byte_start":1353,"byte_end":1366,"line_start":36,"line_end":36,"column_start":38,"column_end":51,"is_primary":true,"text":[{"text":"        let mut closure = expect_sig(|p, y| *p = y);","highlight_start":38,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:9 ~ escape_argument_callee[317d]::test[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's, 't0> extern \"rust-call\" fn((&ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 'r)) mut &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 's)) i32, &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 't0)) i32))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs:36:38\n   |\nLL |         let mut closure = expect_sig(|p, y| *p = y);\n   |                                      ^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:9 ~ escape_argument_callee[317d]::test[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's, 't0> extern \"rust-call\" fn((&ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 'r)) mut &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 's)) i32, &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 't0)) i32))\n           ]\n\n"}
[00:46:12] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs","byte_start":1241,"byte_end":1527,"line_start":30,"line_end":43,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn test() {","highlight_start":1,"highlight_end":12},{"text":"    let x = 44;","highlight_start":1,"highlight_end":16},{"text":"    let mut p = &x;","highlight_start":1,"highlight_end":20},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    {","highlight_start":1,"highlight_end":6},{"text":"        let y = 22;","highlight_start":1,"highlight_end":20},{"text":"        let mut closure = expect_sig(|p, y| *p = y);","highlight_start":1,"highlight_end":53},{"text":"        //~^ ERROR does not outlive free region","highlight_start":1,"highlight_end":48},{"text":"        //~| WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"        closure(&mut p, &y);","highlight_start":1,"highlight_end":29},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    deref(p);","highlight_start":1,"highlight_end":14},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:3 ~ escape_argument_callee[317d]::test[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs:30:1\n   |\nLL | / fn test() {\nLL | |     let x = 44;\nLL | |     let mut p = &x;\nLL | |\n...  |\nLL | |     deref(p);\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:3 ~ escape_argument_callee[317d]::test[0]) with substs []\n\n"}
[00:46:12] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:12] ------------------------------------------
[00:46:12] 
[00:46:12] thread '[ui] ui/nll/closure-requirements/escape-argument-callee.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:46:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:12] 
[00:46:12] ---- [ui] ui/nll/closure-requirements/escape-argument.rs stdout ----
[00:46:12] diff of stderr:
[00:46:12] 
[00:46:12] 6    |
[00:46:12] 7    = note: defining type: DefId(0/1:9 ~ escape_argument[317d]::test[0]::{{closure}}[0]) with closure substs [
[00:46:12] 8                i16,
[00:46:12] -                for<'r, 's> extern "rust-call" fn((&ReLateBound(DebruijnIndex { index: 0 }, BrNamed(crate0:DefIndex(0:0), 'r)) mut &ReLateBound(DebruijnIndex { index: 0 }, BrNamed(crate0:DefIndex(0:0), 's)) i32, &ReLateBound(DebruijnIndex { index: 0 }, BrNamed(crate0:DefIndex(0:0), 's)) i32))
[00:46:12] +                for<'r, 's> extern "rust-call" fn((&ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 'r)) mut &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 's)) i32, &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 's)) i32))
[00:46:12] 11 
[00:46:12] 12 note: No external requirements
[00:46:12] 
[00:46:12] 
[00:46:12] 
[00:46:12] The actual stderr differed from the expected stderr.
[00:46:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument/escape-argument.stderr
[00:46:12] To update references, rerun the tests and pass the `--bless` flag
[00:46:12] To only update this specific test, also pass `--test-args nll/closure-requirements/escape-argument.rs`
[00:46:12] error: 1 errors occurred comparing output.
[00:46:12] status: exit code: 101
[00:46:12] status: exit code: 101
[00:46:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/escape-argument.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument/auxiliary" "-A" "unused"
[00:46:12] ------------------------------------------
[00:46:12] 
[00:46:12] ------------------------------------------
[00:46:12] stderr:
[00:46:12] stderr:
[00:46:12] ------------------------------------------
[00:46:12] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/escape-argument.rs","byte_start":1148,"byte_end":1161,"line_start":36,"line_end":36,"column_start":38,"column_end":51,"is_primary":true,"text":[{"text":"        let mut closure = expect_sig(|p, y| *p = y);","highlight_start":38,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:9 ~ escape_argument[317d]::test[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's> extern \"rust-call\" fn((&ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 'r)) mut &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 's)) i32, &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 's)) i32))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument.rs:36:38\n   |\nLL |         let mut closure = expect_sig(|p, y| *p = y);\n   |                                      ^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:9 ~ escape_argument[317d]::test[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's> extern \"rust-call\" fn((&ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 'r)) mut &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 's)) i32, &ReLateBound(0, BrNamed(crate0:DefIndex(0:0), 's)) i32))\n           ]\n\n"}
[00:46:12] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/escape-argument.rs","byte_start":1036,"byte_end":1272,"line_start":30,"line_end":42,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn test() {","highlight_start":1,"highlight_end":12},{"text":"    let x = 44;","highlight_start":1,"highlight_end":16},{"text":"    let mut p = &x;","highlight_start":1,"highlight_end":20},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    {","highlight_start":1,"highlight_end":6},{"text":"        let y = 22;","highlight_start":1,"highlight_end":20},{"text":"        let mut closure = expect_sig(|p, y| *p = y);","highlight_start":1,"highlight_end":53},{"text":"        closure(&mut p, &y);","highlight_start":1,"highlight_end":29},{"text":"        //~^ ERROR `y` does not live long enough [E0597]","highlight_start":1,"highlight_end":57},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    deref(p);","highlight_start":1,"highlight_end":14},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:3 ~ escape_argument[317d]::test[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument.rs:30:1\n   |\nLL | / fn test() {\nLL | |     let x = 44;\nLL | |     let mut p = &x;\nLL | |\n...  |\nLL | |     deref(p);\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:3 ~ escape_argument[317d]::test[0]) with substs []\n\n"}
[00:46:12] {"message":"`y` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n