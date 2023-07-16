\n#![featur^^^^^^^
[00:59:01] 42 
[00:59:01] - error[E0442]: intrinsic argument 2 has wrong type: found vector with length 4, expected length 8
[00:59:01] -    |
[00:59:01] -    |
[00:59:01] - LL |     fn x86_mm_adds_epi16(x: i8x16, y: i32x4) -> i64x2;
[00:59:01] - 
[00:59:01] - 
[00:59:01] - error[E0442]: intrinsic return value has wrong type: found vector with length 2, expected length 8
[00:59:01] -    |
[00:59:01] -    |
[00:59:01] - LL |     fn x86_mm_adds_epi16(x: i8x16, y: i32x4) -> i64x2;
[00:59:01] - 
[00:59:01] - 
[00:59:01] - error[E0442]: intrinsic argument 1 has wrong type: found `i32`, expected `f32`
[00:59:01] + error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_max_ps`
[00:59:01] 57    |
[00:59:01] 57    |
[00:59:01] 58 LL |     fn x86_mm_max_ps(x: i32x4, y: i32x4) -> i32x4;
[00:59:01] 59    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:01] 60 
[00:59:01] 60 
[00:59:01] - error[E0442]: intrinsic argument 2 has wrong type: found `i32`, expected `f32`
[00:59:01] -    |
[00:59:01] -    |
[00:59:01] - LL |     fn x86_mm_max_ps(x: i32x4, y: i32x4) -> i32x4;
[00:59:01] + error: aborting due to 6 previous errors
[00:59:01] 66 
[00:59:01] 66 
[00:59:01] - error[E0442]: intrinsic return value has wrong type: found `i32`, expected `f32`
[00:59:01] -    |
[00:59:01] -    |
[00:59:01] - LL |     fn x86_mm_max_ps(x: i32x4, y: i32x4) -> i32x4;
[00:59:01] - 
[00:59:01] - error: aborting due to 12 previous errors
[00:59:01] - 
[00:59:01] - For more information about this error, try `rustc --explain E0442`.
[00:59:01] - For more information about this error, try `rustc --explain E0442`.
[00:59:01] + For more information about this error, try `rustc --explain E0441`.
[00:59:01] 76 
[00:59:01] 
[00:59:01] 
[00:59:01] The actual stderr differed from the expected stderr.
[00:59:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-declaration-type/simd-intrinsic-declaration-type.stderr
[00:59:01] To update references, rerun the tests and pass the `--bless` flag
[00:59:01] To only update this specific test, also pass `--test-args simd-intrinsic/simd-intrinsic-declaration-type.rs`
[00:59:01] error: 1 errors occurred comparing output.
[00:59:01] status: exit code: 1
[00:59:01] status: exit code: 1
[00:59:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-declaration-type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-declaration-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-declaration-type/auxiliary" "-A" "unused"
[00:59:01] ------------------------------------------
[00:59:01] 
[00:59:01] ------------------------------------------
[00:59:01] stderr:
[00:59:01] stderr:
[00:59:01] ------------------------------------------
[00:59:01] {"message":"unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`","code":{"code":"E0441","explanation":"\nAn unknown platform-specific intrinsic function was used. Erroneous\ncode example:\n\n