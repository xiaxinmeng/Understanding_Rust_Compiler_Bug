\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/generator-with-nll.rs","byte_start":914,"byte_end":922,"line_start":22,"line_end":22,"column_start":9,"column_end":17,"is_primary":false,"text":[{"text":"        yield ();","highlight_start":9,"highlight_end":17}],"label":"possible yield occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/generator-with-nll.rs","byte_start":675,"byte_end":679,"line_start":19,"line_end":19,"column_start":23,"column_end":27,"is_primary":true,"text":[{"text":"        let _a = &mut true; //~ ERROR borrow may still be in use when generator yields (Ast)","highll,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/generator-with-nll.rs","byte_start":767,"byte_end":771,"line_start":20,"line_end":20,"column_start":22,"column_end":26,"is_primary":true,"text":[{"text":"        let b = &mut true; //~ ERROR borrow may still be in use when generator yields (Ast)","highlight_start":22,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0626]: borrow may still be in use when generator yields (Ast)\n  --> /checkout/src/test/ui/generator/generator-with-nll.rs:20:22\n   |\nLL |         let b = &mut true; //~ ERROR borrow may still be in use when generator yields (Ast)\n   |                      ^^^^\nLL |         //~^ borrow may still be in use when generator yields (Mir)\nLL |         yield ();\n   |         -------- possible yield occurs here\n\n"}
[00:42:59] thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /checkout/src/librustc_data_structures/indexed_vec.rs:505:32
[00:42:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:59] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:42:59] {"message":"For more information about this error, try `rustc --explain E0626`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0626`.\n"}
[00:42:59] error: internal compiler error: unexpected panic
[00:42:59] 
[00:42:59] note: t2:59] - error: aborting due to 4 previous errors
[00:42:59] + error: aborting due to 2 previous errors
[00:42:59] + error: aborting due to 2 previous errors
[00:42:59] 38 
[00:42:59] 39 For more information about this error, try `rustc --explain E0626`.
[00:42:59] 40 
[00:42:59] 
[00:42:59] 
[00:42:59] The actual stderr differed from the expected stderr.
[00:42:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-local-borrowed/yield-while-local-borrowed.stderr
[00:42:59] To update references, rerun the tests and pass the `--bless` flag
[00:42:59] To only update this specific test, also pass `--test-args generator/yield-while-local-borrowed.rs`
[00:42:59] error: 1 errors occurred comparing output.
[00:42:59] status: exit code: 101
[00:42:59] status: exit code: 101
[00:42:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-while-local-borrowed.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-local-borrowed/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-local-borrowed/auxiliary" "-A" "unused"
[00:42:59] ------------------------------------------
[00:42:59] 
[00:42:59] ------------------------------------------
[00:42:59] stderr:
[00:42:59] stderr:
[00:42:59] ------------------------------------------
[00:42:59] {"message":"borrow may still be in use when generator yields (Ast)","code":{"code":"E0626","explanation":"\nThis error occurs because a borrow in a generator persists across a\nyield point.\n\n