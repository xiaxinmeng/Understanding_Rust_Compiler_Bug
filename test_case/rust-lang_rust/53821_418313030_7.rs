\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/issue-52442.rs","byte_start":489,"byte_end":529,"line_start":12,"line_end":12,"column_start":11,"column_end":51,"is_primary":true,"text":[{"text":"    [();  { &loop { break } as *const _ as usize } ]; //~ ERROR unimplemented expression type","highlight_start":11,"highlight_end":51}],"label":"type validation failed: encountered a pointer, but expected the type usize","suggested_replacement":0] The actual stderr differed from the expected stderr.
[00:46:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/ref_to_int_match.stderr
[00:46:00] To update references, rerun the tests and pass the `--bless` flag
[00:46:00] To only update this specific test, also pass `--test-args consts/const-eval/ref_to_int_match.rs`
[00:46:00] error: 1 errors occurred comparing output.
[00:46:00] status: exit code: 1
[00:46:00] status: exit code: 1
[00:46:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/auxiliary" "-A" "unused"
[00:46:00] ------------------------------------------
[00:46:00] 
[00:46:00] ------------------------------------------
[00:46:00] stderr:
[00:46:00] stderr:
[00:46:00] ------------------------------------------
[00:46:00] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:46:00] {"message":"it is undefined behavior to use this value","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n