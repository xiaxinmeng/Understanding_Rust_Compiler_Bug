\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0443.rs","byte_start":695,"byte_end":745,"line_start":20,"line_end":20,"column_start":5,"column_end":55,"is_primary":true,"text":[{"text":"    fn x86_mm_adds_epi16(x: i16x8, y: i16x8) -> i64x8; //~ ERROR E0443","highlight_start":5,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`\n  --> /checkout/src/test/ui/error-codes/E0443.rs:20:5\n   |\nLL |     fn x86_mm_adds_epi16(x: i16x8, y: i16x8) -> i64x8; //~ ERROR E0443\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:59:01] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:59:01] {"message":"For more information about this error, try `rustc --explain E0441`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0441`.\n"}
[00:59:01] ------------------------------------------
[00:59:01] 
[00:59:01] thread '[ui] ui/error-codes/E0443.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:59:01] 
[00:59:01] 
[00:59:01] ---- [ui] ui/error-codes/E0444.rs stdout ----
[00:59:01] diff of stderr:
[00:59:01] 
[00:59:01] - error[E0444]: platform-specific intrinsic has invalid number of arguments: found 3, expected 1
[00:59:01] + error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_movemask_pd`
[00:59:01] 2   --> $DIR/E0444.rs:18:5
[00:59:01] 3    |
[00:59:01] 4 LL |     fn x86_mm_movemask_pd(x: f64x2, y: f64x2, z: f64x2) -> i32; //~ ERROR E0444
[00:59:01] 6 
[00:59:01] 7 error: aborting due to previous error
[00:59:01] 8 
[00:59:01] - For more information about this error, try `rustc --explain E0444`.
[00:59:01] - For more information about this error, try `rustc --explain E0444`.
[00:59:01] + For more information about this error, try `rustc --explain E0441`.
[00:59:01] 10 
[00:59:01] 
[00:59:01] 
[00:59:01] The actual stderr differed from the expected stderr.
[00:59:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0444/E0444.stderr
[00:59:01] To update references, rerun the tests and pass the `--bless` flag
[00:59:01] To only update this specific test, also pass `--test-args error-codes/E0444.rs`
[00:59:01] error: 1 errors occurred comparing output.
[00:59:01] status: exit code: 1
[00:59:01] status: exit code: 1
[00:59:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0444.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0444/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0444/auxiliary" "-A" "unused"
[cognized platform-specific intrinsic function: `x86_mm_movemask_pd`\n  --> /checkout/src/test/ui/error-codes/E0444.rs:18:5\n   |\nLL |     fn x86_mm_movemask_pd(x: f64x2, y: f64x2, z: f64x2) -> i32; //~ ERROR E0444\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:59:01] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:59:01] {"message":"For more information about this error, try `rustc --explain E0441`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0441`.\n"}
[00:59:01] ------------------------------------------
[00:59:01] 
[00:59:01] thread '[ui] ui/error-codes/E0444.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:59:01] 
[00:59:01] 
[00:59:01] ---- [ui] ui/intrinsic-invalid-number-of-arguments.rs stdout ----
[00:59:01] diff of stderr:
[00:59:01] 
[00:59:01] - error[E0444]: platform-specific intrinsic has invalid number of arguments: found 3, expected 1
[00:59:01] + error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_movemask_pd`
[00:59:01] 3    |
[00:59:01] 3    |
[00:59:01] 4 LL |     fn x86_mm_movemask_pd(x: f64x2, y: f64x2, z: f64x2) -> i32; //~ platform-specific intrinsic
[00:59:01] 6 
[00:59:01] 7 error: aborting due to previous error
[00:59:01] 8 
[00:59:01] - For more information about this error, try `rustc --explain E0444`.
[00:59:01] - For more information about this error, try `rustc --explain E0444`.
[00:59:01] + For more information about thi41\n#![feature(repr_simd)]\n#![feature(platform_intrinsics)]\n\n#[repr(simd)]\nstruct i16x8(i16, i16, i16, i16, i16, i16, i16, i16);\n\nextern \"platform-intrinsic\" {\n    fn x86_mm_adds_ep16(x: i16x8, y: i16x8) -> i16x8;\n    // error: unrecognized platform-specific intrinsic function\n}\n