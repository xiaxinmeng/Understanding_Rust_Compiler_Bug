\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-45199.rs","byte_start":766,"byte_end":781,"line_start":20,"line_end":20,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    b = Box::new(2);    //[ast]~ ERROR cannot assign twice to immutable variable","highlight_start":5,"highlight_end":20}],"label":"cannot assign twice to immutable variable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-45199.rs","byte_start":655,"byte_end":670,"line_start":18,"line_end":18,"column_start":5,"column_end":20,"is_primary":false,"text":[{"text":"    b = Box::new(1);    //[ast]~ NOTE first assignment","highlight_start":5,"highlight_end":20}],"label":"first assignment to `b`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0384]: cannot assign twice to immutable variable `b`\n  --> /checkout/src/test/ui/issues/issue-45199.rs:20:5\n   |\nLL |     b = Box::new(1);    //[ast]~ NOTE first assignment\n   |     --------------- first assignment to `b`\nLL |                         //[mir]~^ NOTE first assignment\nLL |     b = Box::new(2);    //[ast]~ ERROR cannot assign twice to immutable variable\n   |     ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable\n\n"}
[00:49:31] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:31] {"message":"For more information about this error, try `rustc --explain E0384`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0384`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/issues/issue-45199.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/issues/issue-45199.rs#mir stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] 1 error[E0384]: cannot assign twice to immutable variable `b`
[00:49:31] -    |
[00:49:31] -    |
[00:49:31] - LL |     let b: Box<isize>;
[00:49:31] -    |         - help: make this binding mutable: `mut b`
[00:49:31] - ...
[00:49:31] - LL |     b = Box::new(1);    //[ast]~ NOTE first assignment
[00:49:31] -    |     - first assignment to `b`
[00:49:31] - LL |                         //[mir]~^ NOTE first assignment
[00:49:31] - LL |     b = Box::new(2);    //[ast]~ ERROR cannot assign twice to immutable variable
[00:49:31] -    |     ^ cannot assign twice to immutable variable
[00:49:31] - 
[00:49:31] - error[E0384]: cannot assign twice to immutable variable `b`
[00:49:31] 15    |
[00:49:31] 15    |
[00:49:31] 16 LL |     let b = Box::new(1);    //[ast]~ NOTE first assignment
[00:49:31] 30 ...
[00:49:31] 30 ...
[00:49:31] 31 LL |     b = Box::new(2);            //[ast]~ ERROR cannot assign twice to immutable variable
[00:49:31] 32    |     ^ cannot assign to immutable argument
[00:49:31] + 
[00:49:31] + error[E0384]: cannot assign twice to immutable variable `b`
[00:49:31] +   --> $DIR/issue-45199.rs:20:5
[00:49:31] +    |
[00:49:31] + LL |     let b: Box<isize>;
[00:49:31] +    |         - help: make this binding mutable: `mut b`
[00:49:31] + ...
[00:49:31] + LL |     b = Box::new(1);    //[ast]~ NOTE first assignment
[00:49:31] +    |     - first assignment to `b`
[00:49:31] + LL |                         //[mir]~^ NOTE first assignment
[00:49:31] + LL |     b = Box::new(2);    //[ast]~ ERROR cannot assign twice to immutable variable
[00:49:31] +    |     ^ cannot assign twice to immutable variable
[00:49:31] 34 error: aborting due to 3 previous errors
[00:49:31] 35 
[00:49:31] 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45199.mir/issue-45199.mir.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args issues/issue-45199.rs`
[00:49:31] 
[00:49:31] error in revision `mir`: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45199.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45199.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45199.mir/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"cannot assign twice to immutable variable `b`","code":{"code":"E0384","explanation":"\nThis error occurs when an attempt is made to reassign an immutable variable.\nFor example:\n\n