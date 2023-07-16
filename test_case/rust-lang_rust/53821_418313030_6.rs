\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs","byte_start":3767,"byte_end":3840,"line_start":102,"line_end":102,"column_start":5,"column_end":78,"is_primary":true,"text":[{"text":"    const STR_F64_UNION: f64 = unsafe { Nonsense { stringy: \"3\" }.float_64 };","highlight_start":5,"highlight_end":78}],"label":"type validation failed: encountered a pointer, but expected the type f64","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: it is undefined behavior to use this value\n  --> /checkout/snsion":null}],"children":[],"rendered":"error: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:108:5\n   |\nLL |     const STR_CHAR_UNION: char = unsafe { Nonsense { stringy: \"3\" }.character };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-----------------------------------^^^\n   |                                           |\n   |                                           a raw memory access tried to access part of a pointer value as raw bytes\n\n"}
[00:46:00] {"message":"aborting due to 29 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 29 previous errors\n\n"}
[00:46:00] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[00:46:00] ------------------------------------------
[00:46:00] 
[00:46:00] thread '[ui] ui/consts/const-eval/const-pointer-values-in-various-types.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:00] 
[00:46:00] ---- [ui] ui/consts/const-eval/issue-52442.rs stdout ----
[00:46:00] diff of stderr:
[00:46:00] 
[00:46:00] 4 LL |     [();  { &loop { break } as *const _ as usize } ]; //~ ERROR unimplemented expression type
[00:46:00] 6 
[00:46:00] - error: aborting due to previous error
[00:46:00] - error: aborting due to previous error
[00:46:00] + error[E0080]: it is undefined behavior to use this value
[00:46:00] +   --> $DIR/issue-52442.rs:12:11
[00:46:00] +    |
[00:46:00] + LL |     [();  { &loop { break } as *const _ as usize } ]; //~ ERROR unimplemented expression type
[00:46:00] +    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected the type usize
[00:46:00] +    |
[00:46:00] +    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[00:46:00] - For more information about this error, try `rustc --explain E0019`.
[00:46:00] + error: aborting due to 2 previous errors
[00:46:00] + 
[00:46:00] + Some errors occurred: E0019, E0080.
[00:46:00] + Some errors occurred: E0019, E0080.
[00:46:00] + For more information about an error, try `rustc --explain E0019`.
[00:46:00] 10 
[00:46:00] 
[00:46:00] 
[00:46:00] The actual stderr differed from the expected stderr.
[00:46:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-52442/issue-52442.stderr
[00:46:00] To update references, rerun the tests and pass the `--bless` flag
[00:46:00] To only update this specific test, also pass `--test-args consts/const-eval/issue-52442.rs`
[00:46:00] error: 1 errors occurred comparing output.
[00:46:00] status: exit code: 1
[00:46:00] status: exit code: 1
[00:46:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-52442.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic"  { &loop { break } as *const _ as usize } ]; //~ ERROR unimplemented expression type","highlight_start":14,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0019]: constant contains unimplemented expression type\n  --> /checkout/src/test/ui/consts/const-eval/issue-52442.rs:12:14\n   |\nLL |     [();  { &loop { break } as *const _ as usize } ]; //~ ERROR unimplemented expression type\n   |              ^^^^^^^^^^^^^^\n\n"}
[00:46:00] {"message":"it is undefined behavior to use this value","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n