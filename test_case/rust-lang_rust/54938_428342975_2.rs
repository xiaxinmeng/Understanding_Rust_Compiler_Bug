\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":542,"byte_end":547,"line_start":15,"line_end":15,"column_start":19,"column_end":24,"is_primary":false,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":19,"highlight_end":24}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":524,"byte_end":574,"lin:42] +   --> $DIR/promoted_errors.rs:24:20
[00:55:42] +    |
[00:55:42] + LL |     println!("{}", 1/(false as u32));
[00:55:42] +    |                    ^^^^^^^^^^^^^^^^ attempt to divide by zero
[00:55:42] + 
[00:55:42] 37 warning: attempt to divide by zero
[00:55:42] 39    |
[00:55:42] 
[00:55:42] 
[00:55:42] The actual stderr differed from the expected stderr.
[00:55:42] The actual stderr differed from the expected stderr.
[00:55:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/promoted_errors.stderr
[00:55:42] To update references, rerun the tests and pass the `--bless` flag
[00:55:42] To only update this specific test, also pass `--test-args consts/const-eval/promoted_errors.rs`
[00:55:42] error: 1 errors occurred comparing output.
[00:55:42] status: exit code: 0
[00:55:42] status: exit code: 0
[00:55:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/auxiliary" "-A" "unused"
[00:55:42] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:55:42] ------------------------------------------
[00:55:42] 
[00:55:42] ------------------------------------------
[00:55:42] stderr:
[00:55:42] stderr:
[00:55:42] ------------------------------------------
[00:55:42] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":580,"byte_end":588,"line_start":17,"line_end":17,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"    let _x = 0u32 - 1;","highlight_start":14,"highlight_end":22}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":475,"byte_end":484,"line_start":11,"line_end":11,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![warn(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:17:14\n   |\nLL |     let _x = 0u32 - 1;\n   |              ^^^^^^^^ attempt to subtract with overflow\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:11:9\n   |\nLL | #![warn(const_err)]\n   |         ^^^^^^^^^\n\n"}
[00:55:42] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":633,"byte_end":640,"line_start":19,"line_end":19,"column_start":20,"column_end":27,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(1-1));","highlight_start":20,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:19:20\n   |\nLL |     println!(\"{}\", 1/(1-1));\n   |                    ^^^^^^^\n\n"}
[00:55:42] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":680,"byte_end":687,"line_start":21,"line_end":21,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:21:14\n   |\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^\n\n"}
[00:55:42] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":680,"byte_end":687,"line_start":21,"line_end":21,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":"attempt to divide by zero","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:21:14\n   |\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^ attempt to divide by zero\n\n"}
[00:55:42] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":756,"byte_end":772,"line_start":24,"line_end":24,"column_start":20,"column_end":36,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(false as u32));","highlight_start":20,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:24:20\n   |\nLL |     println!(\"{}\", 1/(false as u32));\n   |                    ^^^^^^^^^^^^^^^^\n\n"}
[00:55:42] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":756,"byte_end":772,"line_start":24,"line_end":24,"column_start":20,"column_end":36,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(false as u32));","highlight_start":20,"highlight_end":36}],"label":"attempt to divide by zero","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[] |\nLL |     let _x = 1/(false as u32);\n   |              ^^^^^^^^^^^^^^^^ attempt to divide by zero\n\n"}
[00:55:42] ------------------------------------------
[00:55:42] 
[00:55:42] thread '[ui] ui/consts/const-eval/promoted_errors.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:55:42] 
[00:55:42] 
[00:55:42] ---- [ui] ui/nll/user-annotations/cast_static_lifetime.rs stdout ----
[00:55:42] 
[00:55:42] error: ui test compiled successfully!
[00:55:42] status: exit code: 0
[00:55:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/cast_static_lifetime.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime/auxiliary" "-A" "unused"
[00:55:42] ------------------------------------------
[00:55:42] 
[00:55:42] ------------------------------------------
[00:55:42] stderr:
---
[00:55:42] 
[00:55:42] failures:
[00:55:42]     [ui] ui/consts/const-eval/conditebug/bootstrap test
[00:55:42] Build completed unsuccessfully in 0:04:00
[00:55:42] make: *** [check] Error 1
[00:55:42] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24d1583a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:098b245d:start=1539117543898765643,finish=1539117543912926418,duration=14160775
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0115ff3a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0df22160
$ dmesg | grep -i kill
