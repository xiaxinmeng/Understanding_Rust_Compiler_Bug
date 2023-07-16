\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/promote_const_let.rs","byte_start":141,"byte_end":214,"line_start":6,"line_end":9,"column_start":28,"column_end":6,"is_primary":true,"text":[{"text":"    let x: &'static u32 = &{ //~ ERROR does not live long enough","highlight_start":28,"highlight_end":65},{"text":"        let y = 42;","highlight_start":1,"highlight_end":20},{"text":"        y","highlight_start":1,"highlight_end":10},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/promote_const_let.rs","byte_start":216,"byte_end":217,"line_start":10,"line_end":10,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/promote_const_let.rs","byte_start":125,"byte_end":137,"line_start":6,"line_end":6,"column_start":12,"column_end":24,"is_primary":false,"text":[{"text":"    let x: &'static u32 = &{ //~ ERROR does not live long enough","highlight_start":12,"highlight_end":24}],"label":"type annotation requires that borrow lasts for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/promote_const_let.rs:6:28\n   |\nLL |       let x: &'static u32 = &{ //~ ERROR does not live long enough\n   |  ____________------------____^\n   | |            |\n   | |            type annotation requires that borrow lasts for `'static`\nLL | |         let y = 42;\nLL | |         y\nLL | |     };\n   | |_____^ creates a temporary which is freed while still in use\nLL |   }\n   |   - temporary value is freed at the end of this statement\n\n"}
[01:31:44] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:31:45] {"message":"Some errors occurred: E0597, E0716.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0597, E0716.\n"}
[01:31:45] 
[01:31:45] ------------------------------------------
[01:31:45] 
[01:31:45] thread '[ui (nll)] ui/consts/promote_const_let.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:31:45] thread '[ui (nll)] ui/consts/promote_const_let.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:31:45] 
[01:31:45] ---- [ui (nll)] ui/issues/issue-18118.rs stdout ----
[01:31:45] diff of stderr:
[01:31:45] 
[01:31:45] - error[E0658]: let bindings in constants are unstable (see issue #48821)
[01:31:45] -   --> $DIR/issue-18118.rs:5:17
[01:31:45] -    |
[01:31:45] - LL |         let p = 3;
[01:31:45] -    |
[01:31:45] -    = help: add #![feature(const_let)] to the crate attributes to enable
[01:31:45] - 
[01:31:45] - error[E0658]: statements in constants are unstable (see issue #48821)
[01:31:45] - error[E0658]: statements in constants are unstable (see issue #48821)
[01:31:45] -   --> $DIR/issue-18118.rs:5:17
[01:31:45] -    |
[01:31:45] - LL |         let p = 3;
[01:31:45] -    |
[01:31:45] -    = help: add #![feature(const_let)] to the crate attributes to enable
[01:31:45] - 
[01:31:45] - error[E0658]: let bindings in constants are unstable (see issue #48821)
[01:31:45] - error[E0658]: let bindings in constants are unstable (see issue #48821)
[01:31:45] -   --> $DIR/issue-18118.rs:8:9
[01:31:45] -    |
[01:31:45] - LL |         &p //~ ERROR `p` does not live long enough
[01:31:45] -    |
[01:31:45] -    = help: add #![feature(const_let)] to the crate attributes to enable
[01:31:45] - 
[01:31:45] - error[E0658]: let bindings in constants are unstable (see issue #48821)
[01:31:45] - error[E0658]: let bindings in constants are unstable (see issue #48821)
[01:31:45] -   --> $DIR/issue-18118.rs:2:5
[01:31:45] -    |
[01:31:45] - LL | /     const z: &'static isize = {
[01:31:45] - LL | |         //~^ ERROR let bindings in constants are unstable
[01:31:45] - LL | |         //~| ERROR statements in constants are unstable
[01:31:45] - LL | |         let p = 3;
[01:31:45] - ...  |
[01:31:45] - LL | |         //~^ ERROR let bindings in constants are unstable
[01:31:45] - LL | |     };
[01:31:45] -    |
[01:31:45] -    = help: add #![feature(const_let)] to the crate attributes to enable
[01:31:45] - 
[01:31:45] - error[E0658]: statements in constants are unstable (see issue #48821)
[01:31:45] - error[E0658]: statements in constants are unstable (see issue #48821)
[01:31:45] -   --> $DIR/issue-18118.rs:2:5
[01:31:45] -    |
[01:31:45] - LL | /     const z: &'static isize = {
[01:31:45] - LL | |         //~^ ERROR let bindings in constants are unstable
[01:31:45] - LL | |         //~| ERROR statements in constants are unstable
[01:31:45] - LL | |         let p = 3;
[01:31:45] - ...  |
[01:31:45] - LL | |         //~^ ERROR let bindings in constants are unstable
[01:31:45] - LL | |     };
[01:31:45] -    |
[01:31:45] -    = help: add #![feature(const_let)] to the crate attributes to enable
[01:31:45] - 
[01:31:45] - 
[01:31:45] 53 error[E0597]: `p` does not live long enough
[01:31:45] +   --> $DIR/issue-18118.rs:4:9
[01:31:45] 55    |
[01:31:45] 55    |
[01:31:45] 56 LL |         &p //~ ERROR `p` does not live long enough
[01:31:45] 
[01:31:45] 58    |         |
[01:31:45] 58    |         |
[01:31:45] 59    |         borrowed value does not live long enough
[01:31:45] 60    |         using this value as a constant requires that `p` is borrowed for `'static`
[01:31:45] - LL |         //~^ ERROR let bindings in constants are unstable
[01:31:45] 62 LL |     };
[01:31:45] 63    |     - `p` dropped here while still borrowed
[01:31:45] 
[01:31:45] - error: aborting due to 6 previous errors
[01:31:45] + error: aborting due to previous error
[01:31:45] 66 
[01:31:45] 66 
[01:31:45] - Some errors occurred: E0597, E0658.
[01:31:45] - For more information about an error, try `rustc --explain E0597`.
[01:31:45] + For more information about this error, try `rustc --explain E0597`.
[01:31:45] 69 
[01:31:45] 
[01:31:45] 
[01:31:45] The actual stderr differed from the expected stderr.
[01:31:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118.nll/issue-18118.nll.stderr
[01:31:45] To update references, rerun the tests and pass the `--bless` flag
[01:31:45] To only update this specific test, also pass `--test-args issues/issue-18118.rs`
[01:31:45] error: 1 errors occurred comparing output.
[01:31:45] status: exit code: 1
[01:31:45] status: exit code: 1
[01:31:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18118.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118.nll/auxiliary" "-A" "unused"
[01:31:45] ------------------------------------------
[01:31:45] 
[01:31:45] ------------------------------------------
[01:31:45] stderr:
[01:31:45] stderr:
[01:31:45] ------------------------------------------
[01:31:45] {"message":"`p` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n