\n\nNow that the closure has its own copy of the data, there's no need to worry\nabout safety.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-4335.rs","byte_start":593,"byte_end":594,"line_start":16,"line_end":16,"column_start":21,"column_end":22,"is_primary":false,"text":[{"text":"    id(Box::new(|| *v))","highlight_start":21,"highlight_end":22}],"label":"`v` is borrowed here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-4335.rs","byte_start":589,"byte_end":591,"line_start":16,"line_end":16,"column_start":17,"column_end":19,"is_primary":true,"text":[{"text":"    id(Box::new(|| *v))","highlight_start":17,"highlight_end":19}],"label":"may outlive borrowed value `v`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-4335.rs","byte_start":589,"byte_end":591,"line_start":16,"line_end":16,"column_start":17,"column_end":19,"is_primary":true,"text":[{"text":"    id(Box::new(|| *v))","highlight_start":17,"highlight_end":19}],"label":null,"suggested_replacement":"move ||","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function\n  --> /checkout/src/test/ui/issues/issue-4335.rs:16:17\n   |\nLL |     id(Box::new(|| *v))\n   |                 ^^  - `v` is borrowed here\n   |                 |\n   |                 may outlive borrowed value `v`\nhelp: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword\n   |\nLL |     id(Box::new(move || *v))\n   |                 ^^^^^^^\n\n"}
[00:49:31] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:31] {"message":"Some errors occurred: E0373, E0507.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0373, E0507.\n"}
[00:49:31] {"message":"For more information about an error, try `rustc --explain E0373`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0373`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/issues/issue-4335.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/issues/issue-45199.rs#ast stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] 1 error[E0384]: cannot assign twice to immutable variable `b`
[00:49:31] +   --> $DIR/issue-45199.rs:40:5
[00:49:31] 3    |
[00:49:31] 3    |
[00:49:31] - LL |     b = Box::new(1);    //[ast]~ NOTE first assignment
[00:49:31] -    |     --------------- first assignment to `b`
[00:49:31] - LL |                         //[mir]~^ NOTE first assignment
[00:49:31] - LL |     b = Box::new(2);    //[ast]~ ERROR cannot assign twice to immutable variable
[00:49:31] + LL | fn test_args(b: Box<i32>) {  //[ast]~ NOTE first assignment
[00:49:31] +    |              - first assignment to `b`
[00:49:31] + ...
[00:49:31] + LL |     b = Box::new(2);            //[ast]~ ERROR cannot assign twice to immutable variable
[00:49:31] 8    |     ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
[00:49:31] 9 
[00:49:31] 10 error[E0384]: cannot assign twice to immutable variable `b`
[00:49:31] 
[00:49:31] 17    |     ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
[00:49:31] 18 
[00:49:31] 19 error[E0384]: cannot assign twice to immutable variable `b`
[00:49:31] +   --> $DIR/issue-45199.rs:20:5
[00:49:31] 21    |
[00:49:31] 21    |
[00:49:31] - LL | fn test_args(b: Box<i32>) {  //[ast]~ NOTE first assignment
[00:49:31] -    |              - first assignment to `b`
[00:49:31] - ...
[00:49:31] - LL |     b = Box::new(2);            //[ast]~ ERROR cannot assign twice to immutable variable
[00:49:31] + LL |     b = Box::new(1);    //[ast]~ NOTE first assignment
[00:49:31] +    |     --------------- first assignment to `b`
[00:49:31] + LL |                         //[mir]~^ NOTE first assignment
[00:49:31] + LL |     b = Box::new(2);    //[ast]~ ERROR cannot assign twice to immutable variable
[00:49:31] 26    |     ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
[00:49:31] 28 error: aborting due to 3 previous errors
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45199.ast/issue-45199.ast.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args issues/issue-45199.rs`
[00:49:31] 
[00:49:31] error in revision `ast`: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45199.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45199.ast/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45199.ast/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"cannot assign twice to immutable variable `b`","code":{"code":"E0384","explanation":"\nThis error occurs when an attempt is made to reassign an immutable variable.\nFor example:\n\n