plain
[01:08:47] 
[01:08:47] ---- [ui] ui/consts/const_arg_local.rs stdout ----
[01:08:47] diff of stderr:
[01:08:47] 
[01:08:47] - error: argument 3 is required to be a constant
[01:08:47] + error[E0412]: cannot find type `__m128i` in this scope
[01:08:47] +    |
[01:08:47] +    |
[01:08:47] + LL | unsafe fn pclmul(a: __m128i, b: __m128i) -> __m128i {
[01:08:47] + 
[01:08:47] + 
[01:08:47] + error[E0412]: cannot find type `__m128i` in this scope
[01:08:47] +    |
[01:08:47] +    |
[01:08:47] + LL | unsafe fn pclmul(a: __m128i, b: __m128i) -> __m128i {
[01:08:47] + 
[01:08:47] + 
[01:08:47] + error[E0412]: cannot find type `__m128i` in this scope
[01:08:47] +    |
[01:08:47] +    |
[01:08:47] + LL | unsafe fn pclmul(a: __m128i, b: __m128i) -> __m128i {
[01:08:47] + 
[01:08:47] + 
[01:08:47] + error[E0425]: cannot find function `_mm_clmulepi64_si128` in this scope
[01:08:47] 3    |
[01:08:47] 3    |
[01:08:47] 4 LL |     _mm_clmulepi64_si128(a, b, imm8)
[01:08:47] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:08:47] +    |     ^^^^^^^^^^^^^^^^^^^^ not found in this scope
[01:08:47] 6 
[01:08:47] - error: aborting due to previous error
---
[01:08:47] 
[01:08:47] 
[01:08:47] The actual stderr differed from the expected stderr.
[01:08:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_local/const_arg_local.stderr
[01:08:47] To update references, rerun the tests and pass the `--bless` flag
[01:08:47] To only update this specific test, also pass `--test-args consts/const_arg_local.rs`
[01:08:47] error: 1 errors occurred comparing output.
[01:08:47] status: exit code: 1
[01:08:47] status: exit code: 1
[01:08:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_arg_local.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_local/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_local/auxiliary" "-A" "unused"
[01:08:47] ------------------------------------------
[01:08:47] 
[01:08:47] ------------------------------------------
[01:08:47] stderr:
[01:08:47] stderr:
[01:08:47] ------------------------------------------
[01:08:47] {"message":"cannot find type `__m128i` in this scope","code":{"code":"E0412","explanation":"\nThe type name used is not in scope.\n\nErroneous code examples:\n\n