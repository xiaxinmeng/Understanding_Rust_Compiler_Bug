\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/index_out_of_bound.rs","byte_start":485,"byte_end":490,"line_start":11,"line_end":11,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"static FOO: i32 = [][0];","highlight_start":19,"highlight_end":24}],"label":"index out of bounds: the len is 0 but the index is 0","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0080]: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/index_out_of_bound.rs:11:19\n   |\nLL | static FOO: i32 = [][0];\n   |                   ^^^^^ index out of bounds: the len is 0 but the index is 0\n\n"}
[01:01:49] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:01:49] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[01:01:49] ------------------------------------------
[01:01:49] 
[01:01:49] thread '[ui] ui/const-eval/index_out_of_bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[01:01:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:49] 
[01:01:49] ---- [ui] ui/error-codes/E0080.rs stdout ----
[01:01:49]  diff of stderr:
[01:01:49] 
[01:01:49] 26 LL |     Y = (1 / 0) //~ ERROR E0080
[01:01:49] 27    |         ^^^^^^^ attempt to divide with overflow
[01:01:49] - error[E0080]: constant evaluation error
[01:01:49] -   --> $DIR/E0080.rs:14:9
[01:01:49] -    |
[01:01:49] -    |
[01:01:49] - LL |     Y = (1 / 0) //~ ERROR E0080
[01:01:49] -    |         ^^^^^^^ attempt to divide by zero
[01:01:49] + error: internal compiler error: librustc_mir/interpret/terminator/mod.rs:166: impossible case reached
[01:01:49] 35 error: aborting due to 3 previous errors
[01:01:49] 36 
[01:01:49] 
[01:01:49] 
[01:01:49] 
[01:01:49] The actual stderr differed from the expected stderr.
[01:01:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0080.stderr
[01:01:49] To update references, run this command from build directory:
[01:01:49] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'error-codes/E0080.rs'
[01:01:49] error: 1 errors occurred comparing output.
[01:01:49] status: exit code: 101
[01:01:49] status: exit code: 101
[01:01:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0080.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0080.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunsn    X = (1 << 500),\n    Y = (1 / 0)\n}\n