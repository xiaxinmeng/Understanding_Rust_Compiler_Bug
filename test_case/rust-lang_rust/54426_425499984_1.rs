\n\nTo fix this error, you can place thr-dynamic -C rpath
[00:48:10] 
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] thread '[ui] ui/borrowck/borrowck-access-permissions.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] thread '[ui] ui/borrowck/borrowck-access-permissions.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] 
[00:48:10] ---- [ui] ui/borrowck/borrowck-assign-comp.rs#mir stdout ----
[00:48:10] 
[00:48:10] error in revision `mir`: Error: expected failure status (Some(1)) but received status Some(101).
[00:48:10] status: exit code: 101
[00:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-assign-comp.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-comp.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-comp.mir/auxiliary" "-A" "unused"
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
[00:48:10] thread '[ui] ui/borrowck/borrowck-assign-comp.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] thread '[ui] ui/borrowck/borrowck-assign-comp.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] 
[00:48:10] ---- [ui] ui/borrowck/borrowck-closures-mut-and-imm.rs#mir stdout ----
[00:48:10] 
[00:48:10] error in revision `mir`: Error: expected failure status (Some(1)) but received status Some(101).
[00:48:10] status: exit code: 101
[00:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-mut-and-imm.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-mut-and-imm.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-mut-and-imm.mir/auxiliary" "-A" "unused"
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] ------------------------------------------
[00:48:10] stderr:
[00:48:10] stderr:
[00:48:10] ------------------------------------------
[00:48:10] {"message":"cannot borrow `x` as immutable because it is also borrowed as mutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n