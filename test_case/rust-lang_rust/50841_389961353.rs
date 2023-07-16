plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:44:29] 
[00:44:29] running 1426 tests
[00:44:33] .....................................................................................i..............
[00:44:39] .............F.......F.F...F........i...............................................................
[00:44:46] ....................................................................................................
[00:44:49] ....................................................................................................
[00:44:52] ....................................................................................................
[00:44:57] ....................................................................................................
---
[00:45:33] failures:
[00:45:33] 
[00:45:33] ---- [ui] ui/const-eval/conditional_array_execution.rs stdout ----
[00:45:33]  
[00:45:33] error: test compilation failed although it shouldn't!
[00:45:33] status: exit code: 101
[00:45:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/conditional_array_execution.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/conditional_array_execution/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/conditional_array_execution/auxiliary" "-A" "unused"
[00:45:33] ------------------------------------------
[00:45:33] 
[00:45:33] ------------------------------------------
[00:45:33] stderr:
[00:45:33] stderr:
[00:45:33] ------------------------------------------
[00:45:33] {"message":"attempt to subtract with overflow","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/conditional_array_execution.rs","byte_start":538,"byte_end":543,"line_start":15,"line_end":15,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: attempt to subtract with overflow\n  --> /checkout/src/test/ui/const-eval/conditional_array_execution.rs:15:19\n   |\nLL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];\n   |                   ^^^^^\n   |\n   = note: #[warn(const_err)] on by default\n\n"}
[00:45:33] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/conditional_array_execution.rs","byte_start":520,"byte_end":570,"line_start":15,"line_end":15,"column_start":1,"column_end":51,"is_primary":true,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":1,"highlight_end":51}],"label":"attempt to subtract with overflow","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: this constant cannot be used\n  --> /checkout/src/test/ui/const-eval/conditional_array_execution.rs:15:1\n   |\nLL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to subtract with overflow\n\n"}
[00:45:33] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/conditional_array_execution.rs","byte_start":686,"byte_end":689,"line_start":20,"line_end":20,"column_start":20,"column_end":23,"is_primary":true,"text":[{"text":"    println!(\"{}\", FOO);","highlight_start":20,"highlight_end":23}],"label":"referenced constant has errors","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/conditional_array_execution.rs:20:20\n   |\nLL |     println!(\"{}\", FOO);\n   |                    ^^^ referenced constant has errors\n\n"}
[00:45:33] {"message":"librustc_codegen_llvm/mir/operand.rs:333: use of _14 before def","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_codegen_llvm/mir/operand.rs:333: use of _14 before def\n\n"}
[00:45:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:33] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:33] 
[00:45:33] note: the compiler unexpectedly panicked. this is a bug.
[00:45:33] 
[00:45:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:45:33] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:45:33] 
[00:45:33] 
[00:45:33] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:45:33] 
[00:45:33] ------------------------------------------
[00:45:33] 
[00:45:33] thread '[ui] ui/const-eval/conditional_array_execution.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3014:9
[00:45:33] thread '[ui] ui/const-eval/conditional_array_execution.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3014:9
[00:45:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:33] 
[00:45:33] ---- [ui] ui/const-eval/issue-43197.rs stdout ----
[00:45:33]  
[00:45:33] error: test compilation failed although it shouldn't!
[00:45:33] status: exit code: 101
[00:45:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/issue-43197.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/issue-43197/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/issue-43197/auxiliary" "-A" "unused"
[00:45:33] ------------------------------------------
[00:45:33] 
[00:45:33] ------------------------------------------
[00:45:33] stderr:
[00:45:33] stderr:
[00:45:33] ------------------------------------------
[00:45:33] {"message":"attempt to subtract with overflow","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-43197.rs","byte_start":577,"byte_end":580,"line_start":20,"line_end":20,"column_start":20,"column_end":23,"is_primary":true,"text":[{"text":"    const X: u32 = 0-1;","highlight_start":20,"highlight_end":23}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: attempt to subtract with overflow\n  --> /checkout/src/test/ui/const-eval/issue-43197.rs:20:20\n   |\nLL |     const X: u32 = 0-1;\n   |                    ^^^\n   |\n   = note: #[warn(const_err)] on by default\n\n"}
[00:45:33] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-43197.rs","byte_start":562,"byte_end":581,"line_start":20,"line_end":20,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    const X: u32 = 0-1;","highlight_start":5,"highlight_end":24}],"label":"attempt to subtract with overflow","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: this constant cannot be used\n  --> /checkout/src/test/ui/const-eval/issue-43197.rs:20:5\n   |\nLL |     const X: u32 = 0-1;\n   |     ^^^^^^^^^^^^^^^^^^^ attempt to subtract with overflow\n\n"}
[00:45:33] {"message":"attempt to subtract with overflow","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-43197.rs","byte_start":696,"byte_end":699,"line_start":23,"line_end":23,"column_start":24,"column_end":27,"is_primary":true,"text":[{"text":"    const Y: u32 = foo(0-1);","highlight_start":24,"highlight_end":27}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: attempt to subtract with overflow\n  --> /checkout/src/test/ui/const-eval/issue-43197.rs:23:24\n   |\nLL |     const Y: u32 = foo(0-1);\n   |                        ^^^\n\n"}
[00:45:33] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-43197.rs","byte_start":677,"byte_end":701,"line_start":23,"line_end":23,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    const Y: u32 = foo(0-1);","highlight_start":5,"highlight_end":29}],"label":"attempt to subtract with overflow","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: this constant cannot be used\n  --> /checkout/src/test/ui/const-eval/issue-43197.rs:23:5\n   |\nLL |     const Y: u32 = foo(0-1);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to subtract with overflow\n\n"}
[00:45:33] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-43197.rs","byte_start":815,"byte_end":816,"line_start":26,"line_end":26,"column_start":23,"column_end":24,"is_primary":true,"text":[{"text":"    println!(\"{} {}\", X, Y);","highlight_start":23,"highlight_end":24}],"label":"referenced constant has errors","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/issue-43197.rs:26:23\n   |\nLL |     println!(\"{} {}\", X, Y);\n   |                       ^ referenced constant has errors\n\n"}
[00:45:33] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-43197.rs","byte_start":818,"byte_end":819,"line_start":26,"line_end":26,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"    println!(\"{} {}\", X, Y);","highlight_start":26,"highlight_end":27}],"label":"referenced constant has errors","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/issue-43197.rs:26:26\n   |\nLL |     println!(\"{} {}\", X, Y);\n   |                          ^ referenced constant has errors\n\n"}
[00:45:33] {"message":"librustc_codegen_llvm/mir/operand.rs:333: use of _15 before def","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_codegen_llvm/mir/operand.rs:333: use of _15 before def\n\n"}
[00:45:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:33] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:33] 
[00:45:33] note: the compiler unexpectedly panicked. this is a bug.
[00:45:33] 
[00:45:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:45:33] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:45:33] 
[00:45:33] 
[00:45:33] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:45:33] 
[00:45:33] ------------------------------------------
[00:45:33] 
[00:45:33] thread '[ui] ui/const-eval/issue-43197.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3014:9
[00:45:33] thread '[ui] ui/const-eval/issue-43197.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3014:9
[00:45:33] 
[00:45:33] ---- [ui] ui/const-eval/issue-44578.rs stdout ----
[00:45:33]  
[00:45:33] error: test compilation failed although it shouldn't!
[00:45:33] status: exit code: 101
[00:45:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/issue-44578.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/issue-44578/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/issue-44578/auxiliary" "-A" "unused"
[00:45:33] ------------------------------------------
[00:45:33] 
[00:45:33] ------------------------------------------
[00:45:33] stderr:
[00:45:33] stderr:
[00:45:33] ------------------------------------------
[00:45:33] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-44578.rs","byte_start":801,"byte_end":827,"line_start":35,"line_end":35,"column_start":20,"column_end":46,"is_primary":true,"text":[{"text":"    println!(\"{}\", <Bar<u16, u8> as Foo>::AMT); //~ WARN const_err","highlight_start":20,"highlight_end":46}],"label":"referenced constant has errors","suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/issue-44578.rs:35:20\n   |\nLL |     println!(\"{}\", <Bar<u16, u8> as Foo>::AMT); //~ WARN const_err\n   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors\n   |\n   = note: #[warn(const_err)] on by default\n\n"}
[00:45:33] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-44578.rs","byte_start":801,"byte_end":827,"line_start":35,"line_end":35,"column_start":20,"column_end":46,"is_primary":true,"text":[{"text":"    println!(\"{}\", <Bar<u16, u8> as Foo>::AMT); //~ WARN const_err","highlight_start":20,"highlight_end":46}],"label":"referenced constant has errors","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/issue-44578.rs:35:20\n   |\nLL |     println!(\"{}\", <Bar<u16, u8> as Foo>::AMT); //~ WARN const_err\n   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors\n\n"}
[00:45:33] {"message":"librustc_codegen_llvm/mir/operand.rs:333: use of _14 before def","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_codegen_llvm/mir/operand.rs:333: use of _14 before def\n\n"}
[00:45:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:33] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:33] 
[00:45:33] note: the compiler unexpectedly panicked. this is a bug.
[00:45:33] 
[00:45:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:45:33] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:45:33] 
[00:45:33] 
[00:45:33] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:45:33] 
[00:45:33] ------------------------------------------
[00:45:33] 
[00:45:33] thread '[ui] ui/const-eval/issue-44578.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3014:9
[00:45:33] thread '[ui] ui/const-eval/issue-44578.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3014:9
[00:45:33] 
[00:45:33] ---- [ui] ui/const-eval/promoted_errors.rs stdout ----
[00:45:33]  
[00:45:33] error: test compilation failed although it shouldn't!
[00:45:33] status: exit code: 101
[00:45:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/promoted_errors.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/promoted_errors/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/promoted_errors/auxiliary" "-A" "unused"
[00:45:33] ------------------------------------------
[00:45:33] 
[00:45:33] ------------------------------------------
[00:45:33] stderr:
[00:45:33] stderr:
[00:45:33] ------------------------------------------
[00:45:33] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":559,"byte_end":567,"line_start":15,"line_end":15,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"    let _x = 0u32 - 1;","highlight_start":14,"highlight_end":22}],"label":"attempt to subtract with overflow","suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:15:14\n   |\nLL |     let _x = 0u32 - 1;\n   |              ^^^^^^^^ attempt to subtract with overflow\n   |\n   = note: #[warn(const_err)] on by default\n\n"}
[00:45:33] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":612,"byte_end":619,"line_start":17,"line_end":17,"column_start":20,"column_end":27,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(1-1));","highlight_start":20,"highlight_end":27}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:17:20\n   |\nLL |     println!(\"{}\", 1/(1-1));\n   |                    ^^^^^^^\n\n"}
[00:45:33] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":612,"byte_end":619,"line_start":17,"line_end":17,"column_start":20,"column_end":27,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(1-1));","highlight_start":20,"highlight_end":27}],"label":"attempt to divide by zero","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:17:20\n   |\nLL |     println!(\"{}\", 1/(1-1));\n   |                    ^^^^^^^ attempt to divide by zero\n\n"}
[00:45:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:45:33] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":683,"byte_end":690,"line_start":20,"line_end":20,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:20:14\n   |\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^\n\n"}
[00:45:33] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":683,"byte_end":690,"line_start":20,"line_end":20,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":"attempt to divide by zero","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:20:14\n   |\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^ attempt to divide by zero\n\n"}
[00:45:33] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":759,"byte_end":775,"line_start":23,"line_end":23,"column_start":20,"column_end":36,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(false as u32));","highlight_start":20,"highlight_end":36}],"label":"attempt to divide by zero","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:23:20\n   |\nLL |     println!(\"{}\", 1/(false as u32));\n   |                    ^^^^^^^^^^^^^^^^ attempt to divide by zero\n\n"}
[00:45:33] {"message":"librustc_codegen_llvm/mir/operand.rs:333: use of _39 before def","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_codegen_llvm/mir/operand.rs:333: use of _39 before def\n\n"}
[00:45:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:33] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:33] 
[00:45:33] note: the compiler unexpectedly panicked. this is a bug.
[00:45:33] 
[00:45:33] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:45:33] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:45:33] 
[00:45:33] 
[00:45:33] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:45:33] 
[00:45:33] ------------------------------------------
[00:45:33] 
[00:45:33] thread '[ui] ui/const-eval/promoted_errors.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3014:9
---
[00:45:33] test result: FAILED. 1415 passed; 4 failed; 7 ignored; 0 measured; 0 filtered out
[00:45:33] 
[00:45:33] 
[00:45:33] 
[00:45:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:33] 
[00:45:33] 
[00:45:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:33] Build completed unsuccessfully in 0:02:20
[00:45:33] Build completed unsuccessfully in 0:02:20
[00:45:33] Makefile:58: recipe for target 'check' failed
[00:45:33] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b21f80f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
