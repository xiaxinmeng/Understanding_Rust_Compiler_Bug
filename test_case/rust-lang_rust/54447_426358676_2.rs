\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":542,"byte_end":547,"line_start":15,"line_end":15,"column_start":19,"column_end":24,"is_primary":false,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":19,"highlight_end":24}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":524,"byte_end":574,"line_start":15,"line_end":15,"column_start":1,"column_end":51,"is_primary":true,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":1,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: constant evaluation error\n  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:15:1\n   |\nLL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];\n   | ^^^^^^^^^^^^^^^^^^-----^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |                   |\n   |                   attempt to subtract with overflow\n\n"}
[00:46:06] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:46:06] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[00:46:06] ------------------------------------------
[00:46:06] 
[00:46:06] thread '[ui] ui/consts/const-eval/conditional_array_execution.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:46:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:06] 
[00:46:06] ---- [ui] ui/consts/const-eval/promoted_errors.rs stdout ----
[00:46:06] diff of stderr:
[00:46:06] 
[00:46:06] 34 LL |     println!("{}", 1/(false as u32));
[00:46:06] 36 
[00:46:06] + warning: this expression will panic at runtime
[00:46:06] +   --> $DIR/promoted_errors.rs:24:20
[00:46:06] +    |
[00:46:06] +    |
[00:46:06] + LL |     println!("{}", 1/(false as u32));
[00:46:06] +    |                    ^^^^^^^^^^^^^^^^ attempt to divide by zero
[00:46:06] + 
[00:46:06] 37 warning: attempt to divide by zero
[00:46:06] 39    |
[00:46:06] 
[00:46:06] 
[00:46:06] The actual stderr differed from the expected stderr.
[00:46:06] The actual stderr differed from the expected stderr.
[00:46:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/promoted_errors.stderr
[00:46:06] To update references, rerun the tests and pass the `--bless` flag
[00:46:06] To only update this sprrors.rs","byte_start":475,"byte_end":484,"line_start":11,"line_end":11,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![warn(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:17:14\n   |\nLL |     let _x = 0u32 - 1;\n   |              ^^^^^^^^ attempt to subtract with overflow\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:11:9\n   |\nLL | #![warn(const_err)]\n   |         ^^^^^^^^^\n\n"}
[00:46:06] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":633,"byte_end":640,"line_start":19,"line_end":19,"column_start":20,"column_end":27,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(1-1));","highlight_start":20,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:19:20\n   |\nLL |     println!(\"{}\", 1/(1-1));\n   |                    ^^^^^^^\n\n"}
[00:46:06] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":680,"byte_end":687,"line_start":21,"line_end":21,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:21:14\n   |\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^\n\n"}
[00:46:06] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":680,"byte_end":687,"line_start":21,"line_end":21,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":"attempt to divide by zero","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:21:14\n   |\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^ attempt to divide by zero\n\n"}
[00:46:06] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":756,"byte_end":772,"line_start":24,"line_end":24,"column_start":20,"column_end":36,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(false as u32));","highlight_start":20,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:24:20\n   |\nLL |     println!(\"{}\", 1/(false as u32));\n   |                    ^^^^^^^^^^^^^^^^\n\n"}
[00:46:06] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":756,"byte_end":772,"line_start":24,"line_end":24,"column_start":20,"column_end":36,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(false as u32));","highlight_start":20,"highlight_end":36}],"label":"attempt to divide by zero","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:24:20\n   |\nLL |     println!(\"{}\", 1/(false as u32));\n   |                    ^^^^^^^^^^^^^^^^ attempt to divide by zero\n\n"}
[00:46:06] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":812,"byte_end":828,"line_start":26,"line_end":26,"column_start":14,"column_end":30,"is_primary":true,"text":[{"text":"    let _x = 1/(false as u32);","highlight_start":14,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:26:14\n   |\nLL |     let _x = 1/(false as u32);\n   |              ^^^^^^^^^^^^^^^^\n\n"}
[00:46:06] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":812,"byte_end":828,"line_start":26,"line_end":26,"column_start":14,"column_end":30,"is_primary":true,"text":[{"text":"    let _x = 1/(false as u32);","highlight_start":14,"highlight_end":30}],"label":"attempt to divide by zero","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:26:14\n   |\nLL |     let _x = 1/(false as u32);\n   |              ^^^^^^^^^^^^^^^^ attempt to divide by zero\n\n"}
[00:46:06] ------------------------------------------
[00:46:06] 
[00:46:06] thread '[ui] ui/consts/const-eval/promoted_errors.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:46:06] 
[00:46:06] 
[00:46:06] ---- [ui] ui/nll/user-annotations/cast_static_lifetime.rs stdout ----
[00:46:06] 
[00:46:06] error: ui test compiled successfully!
[00:46:06] status: exit code: 0
[00:46:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/cast_static_lifetime.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime/auxiliary" "-A" "unused"
[00:46:06] ------------------------------------------
[00:46:06] 
[00:46:06] ------------------------------------------
[00:46:06] stderr:
---
[00:46:06] 
[00:46:06] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:46:06] 
[00:46:06] 
[00:46:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:06] 
[00:46:06] 
[00:46:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:06] Build completed unsuccessfully in 0:03:07
[00:46:06] Build completed unsuccessfully in 0:03:07
[00:46:06] Makefile:58: recipe for target 'check' failed
[00:46:06] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06d9a55c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
