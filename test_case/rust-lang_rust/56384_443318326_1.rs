\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":659,"byte_end":662,"line_start":19,"line_end":19,"column_start":20,"column_end":23,"is_primary":true,"text":[{"text":"    println!(\"{}\", FOO);","highlight_start":20,"highlight_end":23}],"label":"referenced constant has errors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: evaluation of constant expression failed\n  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:19:20\n   |\nLL |     println!(\"{}\", FOO);\n   |                    ^^^ referenced constant has errors\n\n"}
[00:51:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:35] {"message":"For more information abou_primary":true,"text":[{"text":"    const Y: u32 = foo(0-1);","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:20:5\n   |\nLL |     const Y: u32 = foo(0-1);\n   |     ^^^^^^^^^^^^^^^^^^^---^^\n   |                        |\n   |                        attempt to subtract with overflow\n\n"}
[00:51:35] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/issue-43197.rs","byte_start":558,"byte_end":561,"line_start":18,"line_end":18,"column_start":20,"column_end":23,"is_primary":false,"text":[{"text":"    const X: u32 = 0-1;","highlight_start":20,"highlight_end":23}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/issue-43197.rs","byte_start":543,"byte_end":562,"line_start":18,"line_end":18,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    const X: u32 = 0-1;","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:18:5\n   |\nLL |     const X: u32 = 0-1;\n   |     ^^^^^^^^^^^^^^^---^\n   |                    |\n   |   ts/const-eval/pub_const_err/auxiliary" "-A" "unused"
[00:51:35] ------------------------------------------
[00:51:35] 
[00:51:35] ------------------------------------------
[00:51:35] stderr:
[00:51:35] stderr:
[00:51:35] ------------------------------------------
[00:51:35] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/pub_const_err.rs","byte_start":547,"byte_end":552,"line_start":16,"line_end":16,"column_start":20,"column_end":25,"is_primary":false,"text":[{"text":"pub const Z: u32 = 0 - 1;","highlight_start":20,"highlight_end":25}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/pub_const_err.rs","byte_start":528,"byte_end":553,"line_start":16,"line_end":16,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"pub const Z: u32 = 0 - 1;","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/pub_const_err.rs","byte_start":491,"byte_end":500,"line_start":12,"line_end":12,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![warn(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-eval/pub_const_err.rs:16:1\n   |\nLL | pub const Z: u32 = 0 - 1;\n   | ^^^^^^^^^^^^^^^^^^^-----^\n   |                    |\n   |                    attempt to subtract with overflow\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/consts/const-eval/pub_const_err.rs:12:9\n   |\nLL | #![warn(const_err)]\n   |         ^^^^^^^^^\n\n"}
[00:51:35] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:51:35] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/pub_const_err.rs","byte_start":547,"byte_end":552,"line_start":16,"line_end":16,"column_start":20,"column_end":25,"is_primary":false,"text":[{"text":"pub const Z: u32 = 0 - 1;","highlight_start":20,"highlight_end":25}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/pub_const_err.rs","byte_start":528,"byte_end":553,"line_start":16,"line_end":16,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"pub const Z: u32 = 0 - 1;","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-eval/pub_const_err.rs:16:1\n   |\nLL | pub const Z: u32 = 0 - 1;\n   | ^^^^^^^^^^^^^^^^^^^-----^\n   |                    |\n   |                    attempt to subtract with overflow\n\n"}
[00:51:35] ------------------------------------------
[00:51:35] 
[00:51:35] thread '[ui] ui/consts/const-eval/pub_const_err.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:51:35] 
[00:51:35] 
[00:51:35] ---- [ui] ui/consts/const-eval/pub_const_err_bin.rs stdout ----
[00:51:35] diff of stderr:
[00:51:35] 
[00:51:35] 12 LL | #![warn(const_err)]
[00:51:35] 14 
[00:51:35] 14 
[00:51:35] + warning: any use of this value will cause an error
[00:51:35] +   --> $DIR/pub_const_err_bin.rs:14:1
[00:51:35] +    |
[00:51:35] + LL | pub const Z: u32 = 0 - 1;
[00:51:35] +    |                    |
[00:51:35] +    |                    attempt to subtract with overflow
[00:51:35] + 
[00:51:35] 15 
[00:51:35] 15 
[00:51:35] 
[00:51:35] 
[00:51:35] The actual stderr differed from the expected stderr.
[00:51:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/pub_const_err_bin/pub_const_err_bin.stderr
[00:51:35] To update references, rerun the tests and pass the `--bless` flag
[00:51:35] To only update this specific test, also pass `--test-args consts/const-eval/pub_const_err_bin.rs`
[00:51:35] error: 1 errors occurred comparing output.
[00:51:35] status: exit code: 0
[00:51:35] status: exit code: 0
[00:51:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/pub_const_err_bin.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-consts/const-len-underflow-separate-spans.rs","byte_start":814,"byte_end":817,"line_start":21,"line_end":21,"column_start":17,"column_end":20,"is_primary":true,"text":[{"text":"    let a: [i8; LEN] = unimplemented!();","highlight_start":17,"highlight_end":20}],"label":"referenced constant has errors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: evaluation of constant value failed\n  --> /checkout/src/test/ui/consts/const-len-underflow-separate-spans.rs:21:17\n   |\nLL |     let a: [i8; LEN] = unimplemented!();\n   |                 ^^^ referenced constant has errors\n\n"}
[00:51:35] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-len-underflow-separate-spans.rs","byte_start":721,"byte_end":730,"line_start":17,"line_end":17,"column_start":20,"column_end":29,"is_primary":false,"text":[{"text":"const LEN: usize = ONE - TWO;","highlight_start":20,"highlight_end":29}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-len-underflow-separate-spans.rs","byte_start":702,"byte_end":731,"line_start":17,"line_end":17,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"const LEN: usize = ONE - TWO;","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-len-underflow-separate-spans.rs:17:1\n   |\nLL | const LEN: usize = ONE - TWO;\n   | ^^^^^^^^^^^^^^^^^^^---------^\n   |                    |\n   |                    attempt to subtract with overflow\n\n"}
[00:51:35] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:51:35] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[00:51:35] ------------------------------------------
[00:51:35] 
[00:51:35] thread '[ui] ui/consts/const-len-underflow-separate-spans.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:51:35] 
[00:51:35] 
[00:51:35] ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
[00:51:35] diff of stderr:
[00:51:35] 
[00:51:35] 10 LL |     intrinsics::size_of::<T>()
[00:51:35] 11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:51:35] 12 note: ...which requires computing layout of `Foo`...
[00:51:35] - note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...
[00:51:35] + note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: Some(DefId(0/0:3 ~ const_size_of_cycle[317d]::Foo[0])) }, value: [u8; _] }`...
[00:51:35] 14 note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}`...
[00:51:35] 15   --> $DIR/const-size_of-c_fail,E0391\ntrait FirstTrait : SecondTrait {\n\n}\n\ntrait SecondTrait : FirstTrait {\n\n}\n