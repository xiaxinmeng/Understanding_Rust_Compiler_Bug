
[00:51:04] failures:
[00:51:04] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:476:22
[00:51:04] 
[00:51:04] ---- [ui] ui/lint/type-overflow.rs stdout ----
[00:51:04] 	diff of stderr:
[00:51:04] 
[00:51:04] 54	LL |     let fail: isize = 0x8000_0000_0000_0000; //~WARNING literal out of range for isize
[00:51:04] 55	   |                       ^^^^^^^^^^^^^^^^^^^^^
[00:51:04] 56	   |
[00:51:04] -	   = note: the literal `0x8000_0000_0000_0000` (decimal `9223372036854775808`) does not fit into an `isize` and will become `-9223372036854775808isize`
[00:51:04] +	   = note: the literal `0x8000_0000_0000_0000` (decimal `9223372036854775808`) does not fit into an `isize` and will become `0isize`
[00:51:04] 58	
[00:51:04] 59	warning: literal out of range for i8
[00:51:04] 60	  --> $DIR/type-overflow.rs:34:17
[00:51:04] 
[00:51:04] 
[00:51:04] The actual stderr differed from the expected stderr.
[00:51:04] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow.stderr
[00:51:04] To update references, run this command from build directory:
[00:51:04] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'lint/type-overflow.rs'
[00:51:04] 
[00:51:04] error: 1 errors occurred comparing output.
[00:51:04] status: exit code: 0
[00:51:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/type-overflow.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=wasm32-unknown-unknown" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow.stage2-wasm32-unknown-unknown.wasm" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow.stage2-wasm32-unknown-unknown.aux" "-A" "unused"
[00:51:04] stdout:
[00:51:04] ------------------------------------------
[00:51:04] 
[00:51:04] ------------------------------------------
[00:51:04] stderr:
[00:51:04] ------------------------------------------
[00:51:04] warning: literal out of range for i8
[00:51:04]   --> /checkout/src/test/ui/lint/type-overflow.rs:16:17
[00:51:04]    |
[00:51:04] LL |     let error = 255i8; //~WARNING literal out of range for i8
[00:51:04]    |                 ^^^^^
[00:51:04]    |
[00:51:04]    = note: #[warn(overflowing_literals)] on by default
[00:51:04] 
[00:51:04] warning: literal out of range for i8
[00:51:04]   --> /checkout/src/test/ui/lint/type-overflow.rs:21:16
[00:51:04]    |
[00:51:04] LL |     let fail = 0b1000_0001i8; //~WARNING literal out of range for i8
[00:51:04]    |                ^^^^^^^^^^^^^ help: consider using `u8` instead: `0b1000_0001u8`
[00:51:04]    |
[00:51:04]    = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `-127i8`
[00:51:04] 
[00:51:04] warning: literal out of range for i64
[00:51:04]   --> /checkout/src/test/ui/lint/type-overflow.rs:23:16
[00:51:04]    |
[00:51:04] LL |     let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64
[00:51:04]    |                ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x8000_0000_0000_0000u64`
[00:51:04]    |
[00:51:04]    = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `-9223372036854775808i64`
[00:51:04] 
[00:51:04] warning: literal out of range for u32
[00:51:04]   --> /checkout/src/test/ui/lint/type-overflow.rs:25:16
[00:51:04]    |
[00:51:04] LL |     let fail = 0x1_FFFF_FFFFu32; //~WARNING literal out of range for u32
[00:51:04]    |                ^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x1_FFFF_FFFFu64`
[00:51:04]    |
[00:51:04]    = note: the literal `0x1_FFFF_FFFFu32` (decimal `8589934591`) does not fit into an `u32` and will become `4294967295u32`
[00:51:04] 
[00:51:04] warning: literal out of range for i128
[00:51:04]   --> /checkout/src/test/ui/lint/type-overflow.rs:27:22
[00:51:04]    |
[00:51:04] LL |     let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;
[00:51:04]    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:51:04]    |
[00:51:04]    = note: the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into an `i128` and will become `-170141183460469231731687303715884105728i128`
[00:51:04]    = help: consider using `u128` instead
[00:51:04] 
[00:51:04] warning: literal out of range for i32
[00:51:04]   --> /checkout/src/test/ui/lint/type-overflow.rs:30:16
[00:51:04]    |
[00:51:04] LL |     let fail = 0x8FFF_FFFF_FFFF_FFFE; //~WARNING literal out of range for i32
[00:51:04]    |                ^^^^^^^^^^^^^^^^^^^^^
[00:51:04]    |
[00:51:04]    = note: the literal `0x8FFF_FFFF_FFFF_FFFE` (decimal `10376293541461622782`) does not fit into an `i32` and will become `-2i32`
[00:51:04]    = help: consider using `i128` instead
[00:51:04] 
[00:51:04] warning: literal out of range for isize
[00:51:04]   --> /checkout/src/test/ui/lint/type-overflow.rs:32:23
[00:51:04]    |
[00:51:04] LL |     let fail: isize = 0x8000_0000_0000_0000; //~WARNING literal out of range for isize
[00:51:04]    |                       ^^^^^^^^^^^^^^^^^^^^^
[00:51:04]    |
[00:51:04]    = note: the literal `0x8000_0000_0000_0000` (decimal `9223372036854775808`) does not fit into an `isize` and will become `0isize`
[00:51:04] 
[00:51:04] warning: literal out of range for i8
[00:51:04]   --> /checkout/src/test/ui/lint/type-overflow.rs:34:17
[00:51:04]    |
[00:51:04] LL |     let fail = -0b1111_1111i8; //~WARNING literal out of range for i8
[00:51:04]    |                 ^^^^^^^^^^^^^ help: consider using `i16` instead: `0b1111_1111i16`
[00:51:04]    |
[00:51:04]    = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `-1i8`
[00:51:04] 
[00:51:04] 
[00:51:04] ------------------------------------------
[00:51:04] 
[00:51:04] thread '[ui] ui/lint/type-overflow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2893:9
[00:51:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:04] 
[00:51:04] 
[00:51:04] failures:
[00:51:04]     [ui] ui/lint/type-overflow.rs
[00:51:04] 
[00:51:04] test result: FAILED. 1242 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out
[00:51:04] 
[00:51:04] 
[00:51:04] 
[00:51:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:04] expected success, got: exit code: 101
[00:51:04] 
[00:51:04] 
[00:51:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore src/libstd_unicode/
[00:51:04] Build completed unsuccessfully in 0:47:59
