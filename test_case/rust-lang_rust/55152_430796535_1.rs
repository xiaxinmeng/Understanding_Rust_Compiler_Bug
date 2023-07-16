\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":646,"byte_end":649,"line_start":19,"line_end":19,"column_start":20,"column_end":23,"is_primary":true,"text":[{"text":"    println!(\"{}\", FOO);","highlight_start":20,"highlight_end":23}],"label":"referenced constant has errors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: could not evaluate constant\n  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:19:20\n   |\nLL |     println!(\"{}\", FOO);\n   |                    ^^^ referenced constant has errors\n\n"}
[00:46:51] {"message":"constant evaluation error","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly eval           attempt to subtract with overflow\n\n"}
[00:46:51] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:46:51] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[00:46:51] ------------------------------------------
[00:46:51] 
[00:46:51] thread '[ui] ui/consts/const-eval/conditional_array_execution.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:46:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:51] 
[00:46:51] ---- [ui] ui/consts/const-eval/promoted_errors.rs stdout ----
[00:46:51] diff of stderr:
[00:46:51] 
[00:46:51] 34 LL |     println!("{}", 1/(false as u32));
[00:46:51] 36 
[00:46:51] + warning: this expression will panic at runtime
[00:46:51] +   --> $DIR/promoted_errors.rs:24:20
[00:46:51] +    |
[00:46:51] +    |
[00:46:51] + LL |     println!("{}", 1/(false as u32));
[00:46:51] +    |                    ^^^^^^^^^^^^^^^^ attempt to divide by zero
[00:46:51] + 
[00:46:51] 37 warning: attempt to divide by zero
[00:46:51] 39    |
[00:46:51] 
[00:46:51] 
[00:46:51] The actual stderr differed from the expected stderr.
[00:46:51] The actual stderr differed from the expected stderr.
[00:46:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors/promoted_errors.stderr
[00:46:51] To uion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":475,"byte_end":484,"line_start":11,"line_end":11,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![warn(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:17:14\n   |\nLL |     let _x = 0u32 - 1;\n   |              ^^^^^^^^ attempt to subtract with overflow\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:11:9\n   |\nLL | #![warn(const_err)]\n   |         ^^^^^^^^^\n\n"}
[00:46:51] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":633,"byte_end":640,"line_start":19,"line_end":19,"column_start":20,"column_end":27,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(1-1));","highlight_start":20,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:19:20\n   |\nLL |     println!(\"{}\", 1/(1-1));\n   |                    ^^^^^^^\n\n"}
[00:46:51] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":680,"byte_end":687,"line_start":21,"line_end":21,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:21:14\n   |\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^\n\n"}
[00:46:51] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":680,"byte_end":687,"line_start":21,"line_end":21,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":"attempt to divide by zero","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:21:14\n   |\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^ attempt to divide by zero\n\n"}
[00:46:51] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":756,"byte_end":772,"line_start":24,"line_end":24,"column_start":20,"column_end":36,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(false as u32));","highlight_start":20,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:24:20\n   |\nLL |     println!(\"{}\", 1/(false as u32));\n   |                    ^^^^^^^^^^^^^^^^\n\n"}
[00:46:51] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":756,"byte_end":772,"line_start":24,"line_end":24,"column_start":20,"column_end":36,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(false as u32));","highlight_start":20,"highlight_end":36}],"label":"attempt to divide by zero","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:24:20\n   |\nLL |     println!(\"{}\", 1/(false as u32));\n   |                    ^^^^^^^^^^^^^^^^ attempt to divide by zero\n\n"}
[00:46:51] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_errors.rs","byte_start":812,"byte_end":828,"line_start":26,"line_end":26,"column_start":14,"column_end":30,"is_primary":true,"text":[{"text":"t: 'a`...
[00:46:51] - error: aborting due to previous error
[00:46:51] - error: aborting due to previous error
[00:46:51] + error[E0309]: the associated type `<T as MyTrait<'a>>::Output` may not live long enough
[00:46:51] +   --> $DIR/projection-where-clause-env-wrong-bound.rs:17:5
[00:46:51] +    |
[00:46:51] + LL |     bar::<T::Output>() //~ ERROR may not live long enough
[00:46:51] +    |
[00:46:51] +    |
[00:46:51] +    = help: consider adding an explicit lifetime bound `<T as MyTrait<'a>>::Output: 'a`...
[00:46:51] + error: aborting due to 2 previous errors
[00:46:51] 10 
[00:46:51] 11 For more information about this error, try `rustc --explain E0309`.
[00:46:51] 12 
[00:46:51] 12 
[00:46:51] 
[00:46:51] 
[00:46:51] The actual stderr differed from the expected stderr.
[00:46:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-where-clause-env-wrong-bound/projection-where-clause-env-wrong-bound.stderr
[00:46:51] To update references, rerun the tests and pass the `--bless` flag
[00:46:51] To only update this specific test, also pass `--test-args nll/ty-outlives/projection-where-clause-env-wrong-bound.rs`
[00:46:51] error: 1 errors occurred comparing output.
[00:46:51] status: exit code: 1
[00:46:51] status: exit code: 1
[00:46:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-where-clause-env-wrong-bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-where-clause-env-wrong-bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-where-clause-env-wrong-bound/auxiliary" "-A" "unused"
[00:46:51] ------------------------------------------
[00:46:51] 
[00:46:51] ------------------------------------------
[00:46:51] stderr:
[00:46:51] stderr:
[00:46:51] ------------------------------------------
[00:46:51] {"message":"the associated type `<T as MyTrait<'_>>::Output` may not live long enough","code":{"code":"E0309","explanation":"\nThe type definition contains some field whose type\nrequires an outlives annotation. Outlives annotations\n(e.g., `T: 'a`) are used to guarantee that all the data in T is valid\nfor at least the lifetime `'a`. This scenario most commonly\narises when the type contains an associated type reference\nlike `<T as SomeTrait<'a>>::Output`, as shown in this example:\n\n