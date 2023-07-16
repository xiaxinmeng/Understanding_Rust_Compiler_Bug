\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const_arg_promotable.rs","byte_start":167,"byte_end":187,"line_start":7,"line_end":7,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    _mm_clmulepi64_si128(a, b, *&mut 42) //~ ERROR argument 3 is required to be a constant","highlight_start":5,"highlight_end":25}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0425]: cannot find function `_mm_clmulepi64_si128` in this scope\n  --> /checkout/src/test/ui/consts/const_arg_promotable.rs:7:5\n   |\nLL |     _mm_clmulepi64_si128(a, b, *&mut 42) //~ ERROR argument 3 is required to be a constant\n   |     ^^^^^^^^^^^^^^^^^^^^ not found in this scope\n\n"}
[01:08:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:08:47] {"message":"Some errors occurred: E0412, E0425.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0412, E0425.\n"}
[01:08:47] 
[01:08:47] ------------------------------------------
[01:08:47] 
[01:08:47] thread '[ui] ui/consts/const_arg_promotable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:08:47] thread '[ui] ui/consts/const_arg_promotable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:08:47] 
[01:08:47] ---- [ui] ui/consts/const_arg_wrapper.rs stdout ----
[01:08:47] diff of stderr:
[01:08:47] 
[01:08:47] - error: argument 3 is required to be a constant
[01:08:47] + error[E0412]: cannot find type `__m128i` in this scope
[01:08:47] +   --> $DIR/const_arg_wrapper.rs:6:21
[01:08:47] +    |
[01:08:47] + LL | unsafe fn pclmul(a: __m128i, b: __m128i, imm8: i32) -> __m128i {
[01:08:47] + 
[01:08:47] + 
[01:08:47] + error[E0412]: cannot find type `__m128i` in this scope
[01:08:47] +   --> $DIR/const_arg_wrapper.rs:6:33
[01:08:47] +    |
[01:08:47] + LL | unsafe fn pclmul(a: __m128i, b: __m128i, imm8: i32) -> __m128i {
[01:08:47] + 
[01:08:47] + 
[01:08:47] + error[E0412]: cannot find type `__m128i` in this scope
[01:08:47] +   --> $DIR/const_arg_wrapper.rs:6:56
[01:08:47] +    |
[01:08:47] + LL | unsafe fn pclmul(a: __m128i, b: __m128i, imm8: i32) -> __m128i {
[01:08:47] + 
[01:08:47] + 
[01:08:47] + error[E0425]: cannot find function `_mm_clmulepi64_si128` in this scope
[01:08:47] 2   --> $DIR/const_arg_wrapper.rs:7:5
[01:08:47] 3    |
[01:08:47] 4 LL |     _mm_clmulepi64_si128(a, b, imm8)
[01:08:47] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:08:47] +    |     ^^^^^^^^^^^^^^^^^^^^ not found in this scope
[01:08:47] 6 
[01:08:47] - error: aborting due to previous error
---
[01:08:47] 9 
[01:08:47] 
[01:08:47] 
[01:08:47] The actual stderr differed from the expected stderr.
[01:08:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_wrapper/const_arg_wrapper.stderr
[01:08:47] To update references, rerun the tests and pass the `--bless` flag
[01:08:47] To only update this specific test, also pass `--test-args consts/const_arg_wrapper.rs`
[01:08:47] error: 1 errors occurred comparing output.
[01:08:47] status: exit code: 1
[01:08:47] status: exit code: 1
[01:08:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_arg_wrapper.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_wrapper/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_wrapper/auxiliary" "-A" "unused"
[01:08:47] ------------------------------------------
[01:08:47] 
[01:08:47] ------------------------------------------
[01:08:47] stderr:
[01:08:47] stderr:
[01:08:47] ------------------------------------------
[01:08:47] {"message":"cannot find type `__m128i` in this scope","code":{"code":"E0412","explanation":"\nThe type name used is not in scope.\n\nErroneous code examples:\n\n