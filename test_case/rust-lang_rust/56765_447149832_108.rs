\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-45199.rs","byte_start":655,"byte_end":656,"line_start":18,"line_end":18,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    b = Box::new(1);    //[ast]~ NOTE first assignment","highlight_start":5,"highlight_end":6}],"label":"first assignment to `b`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-45199.rs","byte_start":766,"byte_end":767,"line_start":20,"line_end":20,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    b = Box::new(2);    //[ast]~ ERROR cannot assign twice to immutable variable","highlight_start":5,"highlight_end":6}],"label":"cannot assign twice to immutable variable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"make this binding mutable","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-45199.rs","byte_start":560,"byte_end":561,"line_start":15,"line_end":15,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let b: Box<isize>;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"mut b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0384]: cannot assign twice to immutable variable `b`\n  --> /checkout/src/test/ui/issues/issue-45199.rs:20:5\n   |\nLL |     let b: Box<isize>;\n   |         - help: make this binding mutable: `mut b`\n...\nLL |     b = Box::new(1);    //[ast]~ NOTE first assignment\n   |     - first assignment to `b`\nLL |                         //[mir]~^ NOTE first assignment\nLL |     b = Box::new(2);    //[ast]~ ERROR cannot assign twice to immutable variable\n   |     ^ cannot assign twice to immutable variable\n\n"}
[00:49:31] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:31] {"message":"For more information about this error, try `rustc --explain E0384`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0384`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/issues/issue-45199.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/issues/issue-46604.rs#mir stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] - error[E0017]: references in statics may only refer to immutable values
[00:49:31] -    |
[00:49:31] -    |
[00:49:31] - LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];   //[ast]~ ERROR E0017
[00:49:31] -    |                         ^^^^^^^^^^^^^^^^^^^^ statics require immutable values
[00:49:31] - 
[00:49:31] 7 error[E0594]: cannot assign to `buf[..]`, as `buf` is an immutable static item
[00:49:31] 9    |
[00:49:31] 
[00:49:31] 
[00:49:31] 10 LL |     buf[0]=2;                                   //[ast]~ ERROR E0389
[00:49:31] + 
[00:49:31] + 
[00:49:31] + error[E0017]: references in statics may only refer to immutable values
[00:49:31] +   --> $DIR/issue-46604.rs:14:25
[00:49:31] +    |
[00:49:31] + LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];   //[ast]~ ERROR E0017
[00:49:31] +    |                         ^^^^^^^^^^^^^^^^^^^^ statics require immutable values
[00:49:31] 13 error: aborting due to 2 previous errors
[00:49:31] 14 
[00:49:31] 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604.mir/issue-46604.mir.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args issues/issue-46604.rs`
[00:49:31] 
[00:49:31] error in revision `mir`: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
