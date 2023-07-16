plain
travis_time:end:22dc9a82:start=1544631727719459948,finish=1544631841905408414,duration=114185948466
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:56] .................................................................................................... 2700/5170
[00:49:00] .................................................................................................... 2800/5170
[00:49:03] .................................................................................................... 2900/5170
[00:49:07] .................................................................................................... 3000/5170
[00:49:10] .........................F...........................................................i.............. 3100/5170
[00:49:16] ................................................ii..i..ii........................................... 3300/5170
[00:49:20] .................................................................................................... 3400/5170
[00:49:24] .................................................................................................... 3500/5170
[00:49:27] .................................ii................................................................. 3600/5170
---
[00:50:21] 
[00:50:21] ---- [ui] ui/lint/type-overflow.rs stdout ----
[00:50:21] diff of stderr:
[00:50:21] 
[00:50:21] 12 LL |     let fail = 0b1000_0001i8; //~WARNING literal out of range for i8
[00:50:21] 13    |                ^^^^^^^^^^^^^ help: consider using `u8` instead: `0b1000_0001u8`
[00:50:21] 14    |
[00:50:21] -    = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `-127i8`
[00:50:21] +    = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `340282366920938463463374607431768211329i8`
[00:50:21] 16 
[00:50:21] 17 warning: literal out of range for i64
[00:50:21] 
[00:50:21] 
[00:50:21] 20 LL |     let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64
[00:50:21] 21    |                ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x8000_0000_0000_0000u64`
[00:50:21] 22    |
[00:50:21] -    = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `-9223372036854775808i64`
[00:50:21] +    = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `340282366920938463454151235394913435648i64`
[00:50:21] 24 
[00:50:21] 25 warning: literal out of range for u32
[00:50:21] 
[00:50:21] 
[00:50:21] 36 LL |     let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;
[00:50:21] 38    |
[00:50:21] 38    |
[00:50:21] -    = note: the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into an `i128` and will become `-170141183460469231731687303715884105728i128`
[00:50:21] +    = note: the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into an `i128` and will become `170141183460469231731687303715884105728i128`
[00:50:21] 40    = help: consider using `u128` instead
[00:50:21] 41 
[00:50:21] 42 warning: literal out of range for i32
[00:50:21] 
[00:50:21] 45 LL |     let fail = 0x8FFF_FFFF_FFFF_FFFE; //~WARNING literal out of range for i32
[00:50:21] 47    |
[00:50:21] 47    |
[00:50:21] -    = note: the literal `0x8FFF_FFFF_FFFF_FFFE` (decimal `10376293541461622782`) does not fit into an `i32` and will become `-2i32`
[00:50:21] +    = note: the literal `0x8FFF_FFFF_FFFF_FFFE` (decimal `10376293541461622782`) does not fit into an `i32` and will become `340282366920938463463374607431768211454i32`
[00:50:21] 49    = help: consider using `i128` instead
[00:50:21] 50 
[00:50:21] 51 warning: literal out of range for i8
[00:50:21] 
[00:50:21] 54 LL |     let fail = -0b1111_1111i8; //~WARNING literal out of range for i8
[00:50:21] 55    |                 ^^^^^^^^^^^^^ help: consider using `i16` instead: `0b1111_1111i16`
[00:50:21] 56    |
[00:50:21] -    = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `-1i8`
[00:50:21] +    = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `340282366920938463463374607431768211455i8`
[00:50:21] 59 
[00:50:21] 
[00:50:21] 
[00:50:21] The actual stderr differed from the expected stderr.
[00:50:21] The actual stderr differed from the expected stderr.
[00:50:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow/type-overflow.stderr
[00:50:21] To update references, rerun the tests and pass the `--bless` flag
[00:50:21] To only update this specific test, also pass `--test-args lint/type-overflow.rs`
[00:50:21] error: 1 errors occurred comparing output.
[00:50:21] status: exit code: 0
[00:50:21] status: exit code: 0
[00:50:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/type-overflow.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow/auxiliary" "-A" "unused"
[00:50:21] ------------------------------------------
[00:50:21] 
[00:50:21] ------------------------------------------
[00:50:21] stderr:
[00:50:21] stderr:
[00:50:21] ------------------------------------------
[00:50:21] {"message":"literal out of range for i8","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":512,"byte_end":517,"line_start":14,"line_end":14,"column_start":17,"column_end":22,"is_primary":true,"text":[{"text":"    let error = 255i8; //~WARNING literal out of range for i8","highlight_start":17,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(overflowing_literals)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i8\n  --> /checkout/src/test/ui/lint/type-overflow.rs:14:17\n   |\nLL |     let error = 255i8; //~WARNING literal out of range for i8\n   |                 ^^^^^\n   |\n   = note: #[warn(overflowing_literals)] on by default\n\n"}
[00:50:21] {"message":"literal out of range for i8","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lintl":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":747,"byte_end":771,"line_start":21,"line_end":21,"column_start":16,"column_end":40,"is_primary":true,"text":[{"text":"    let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64","highlight_start":16,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `340282366920938463454151235394913435648i64`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `u64` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":747,"byte_end":771,"line_start":21,"line_end":21,"column_start":16,"column_end":40,"is_primary":true,"text":[{"text":"    let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64","highlight_start":16,"highlight_end":40}],"label":null,"suggested_replacement":"0x8000_0000_0000_0000u64","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i64\n  --> /checkout/src/test/ui/lint/type-overflow.rs:21:16\n   |\nLL |     let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64\n   |                ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x8000_0000_0000_0000u64`\n   |\n   = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `340282366920938463454151235394913435648i64`\n\n"}
[00:50:21] {"message":"literal out of range for u32","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":829,"byte_end":845,"line_start":23,"line_end":23,"column_start":16,"column_end":32,"is_primary":true,"text":[{"text":"    let fail = 0x1_FFFF_FFFFu32; //~WARNING literal out of range for u32","highlight_start":16,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the literal `0x1_FFFF_FFFFu32` (decimal `8589934591`) does not fit into an `u32` and will become `4294967295u32`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `u64` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":829,"byte_end":845,"line_start":23,"line_end":23,"column_start":16,"column_end":32,"is_primary":true,"text":[{"text":"    let fail = 0x1_FFFF_FFFFu32; //~WARNING literal out of range for u32","highlight_start":16,"highlight_end":32}],"label":null,"suggested_replacement":"0x1_FFFF_FFFFu64","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for u32\n  --> /checkout/src/test/ui/lint/type-overflow.rs:23:16\n   |\nLL |     let fail = 0x1_FFFF_FFFFu32; //~WARNING literal out of range for u32\n   |                ^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x1_FFFF_FFFFu64`\n   |\n   = note: the literal `0x1_FFFF_FFFFu32` (decimal `8589934591`) does not fit into an `u32` and will become `4294967295u32`\n\n"}
[00:50:21] {"message":"literal out of range for i128","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":909,"byte_end":950,"line_start":25,"line_end":25,"column_start":22,"column_end":63,"is_primary":true,"text":[{"text":"    let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;","highlight_start":22,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into an `i128` and will become `170141183460469231731687303715884105728i128`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `u128` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i128\n  --> /checkout/src/test/ui/lint/type-overflow.rs:25:22\n   |\nLL |     let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;\n   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into an `i128` and will become `170141183460469231731687303715884105728i128`\n   = help: consider using `u128` instead\n\n"}
[00:50:21] {"message":"literal out of range for i32","code":{"code":"over11i8; //~WARNING literal out of range for i8","highlight_start":17,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `340282366920938463463374607431768211455i8`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `i16` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":1095,"byte_end":1108,"line_start":30,"line_end":30,"column_start":17,"column_end":30,"is_primary":true,"text":[{"text":"    let fail = -0b1111_1111i8; //~WARNING literal out of range for i8","highlight_start":17,"highlight_end":30}],"label":null,"suggested_replacement":"0b1111_1111i16","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i8\n  --> /checkout/src/test/ui/lint/type-overflow.rs:30:17\n   |\nLL |     let fail = -0b1111_1111i8; //~WARNING literal out of range for i8\n   |                 ^^^^^^^^^^^^^ help: consider using `i16` instead: `0b1111_1111i16`\n   |\n   = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `340282366920938463463374607431768211455i8`\n\n"}
[00:50:21] ------------------------------------------
[00:50:21] 
[00:50:21] thread '[ui] ui/lint/type-overflow.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:50:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:21] 
[00:50:21] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:50:21] 
[00:50:21] 
[00:50:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
