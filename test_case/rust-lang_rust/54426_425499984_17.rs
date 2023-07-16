\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs","byte_start":10575,"byte_end":10581,"line_start":306,"line_end":306,"column_start":28,"column_end":34,"is_primary":false,"text":[{"text":"                   let y = &mut x;","highlight_start":28,"highlight_end":34}],"label":"first mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs","byte_start":10602,"byte_end":10608,"line_start":307,"line_end":307,"column_start":20,"column_end":26,"is_primary":true,"text":[{"text":"                   &mut x; //[ast]~ ERROR cannot borrow `**x` as mutable more than once at a time","highlight_start":20,"highlight_end":26}],"label":"second mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs","byte_start":10789,"byte_end":10795,"line_start":309,"line_end":309,"column_start":20,"column_end":26,"is_primary":false,"text":[{"text":"                   *y = 1;","highlight_start":20,"highlight_end":26}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0499]: cannot borrow `_` as mutable more than once at a time\n  --> /checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs:307:20\n   |\nLL |                    let y = &mut x;\n   |                            ------ first mutable borrow occurs here\nLL |                    &mut x; //[ast]~ ERROR cannot borrow `**x` as mutable more than once at a time\n   |                    ^^^^^^ second mutable borrow occurs here\nLL |                    //[mir]~^ ERROR cannot borrow `x` as mutable more than once at a time\nLL |                    *y = 1;\n   |                    ------ borrow later used here\n\n"}
[00:48:10] {"message":"unsatisfied lifetime constraints","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs","byte_start":10475,"byte_end":10477,"line_start":304,"line_end":304,"column_start":12,"column_end":14,"is_primary":false,"text":[{"text":"           || {","highlight_start":12,"highlight_end":14}],"label":"lifetime `'1` represents this closure's body","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs","byte_start":10476,"byte_end":10477,"line_start":304,"line_end":304,"column_start":13,"column_end":14,"is_primary":false,"text":[{"text":"           || {","highlight_start":13,"highlight_end":14}],"label":"return type of closure is [closure@/e information about this error, try `rustc --explain E0499`.\n"}
[00:48:10] note: the compiler unexpectedly panicked. this is a bug.
[00:48:10] 
[00:48:10] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:10] 
[00:48:10] 
[00:48:10] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:48:10] 
[00:48:10] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -C prefer-dynamic -C rpath
[00:48:10] 
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] thread '[ui] ui/borrowck/borrowck-describe-lvalue.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] thread '[ui] ui/borrowck/borrowck-describe-lvalue.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] 
[00:48:10] ---- [ui] ui/borrowck/borrowck-init-in-fru.rs#mir stdout ----
[00:48:10] 
[00:48:10] error in revision `mir`: Error: expected failure status (Some(1)) but received status Some(101).
[00:48:10] status: exit code: 101
[00:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-init-in-fru.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-init-in-fru.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-init-in-fru.mir/auxiliary" "-A" "unused"
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] ------------------------------------------
[00:48:10] stderr:
[00:48:10] stderr:
[00:48:10] ------------------------------------------
[00:48:10] {"message":"librustc_mir/borrow_check/error_reporting.rs:1150: End-user description not implemented for field access on `Int(isize)`","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_mir/borrow_check/error_reporting.rs:1150: End-user description not implemented for field access on `Int(isize)`\n\n"}
[00:48:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:10] note: the compiler unexpectedly panicked. this is a bug.
[00:48:10] 
[00:48:10] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:10] 
[00:48:10] 
[00:48:10] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:48:10] 
[00:48:10] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -C prefer-dynamic -C rpath
[00:48:10] 
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] thread '[ui] ui/borrowck/borrowck-init-in-fru.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] thread '[ui] ui/borrowck/borrowck-init-in-fru.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] 
[00:48:10] ---- [ui] ui/borrowck/borrowck-issue-14498.rs#mir stdout ----
[00:48:10] 
[00:48:10] error in revision `mir`: Error: expectee":"/checkout/src/test/ui/borrowck/borrowck-issue-14498.rs","byte_start":912,"byte_end":914,"line_start":28,"line_end":28,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"    let p = &y;","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":"&mut y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to `p***` which is behind a `&` reference\n  --> /checkout/src/test/ui/borrowck/borrowck-issue-14498.rs:29:5\n   |\nLL |     let p = &y;\n   |             -- help: consider changing this to be a mutable reference: `&mut y`\nLL |     ***p = 2; //[ast]~ ERROR cannot assign to data in a `&` reference\n   |     ^^^^^^^^ `p` is a `&` reference, so the data it refers to cannot be written\n\n"}
[00:48:10] {"message":"cannot assign to `y**` because it is borrowed","code":{"code":"E0506","explanation":"\nThis error occurs when an attempt is made to assign to a borrowed value.\n\nExample of erroneous code:\n\n