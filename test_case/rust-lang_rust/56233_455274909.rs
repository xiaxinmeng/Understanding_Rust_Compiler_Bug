plain
travis_time:end:0967b078:start=1547745500451120682,finish=1547745579458456864,duration=79007336182
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:58:15] .................................................................................................... 2800/5297
[00:58:18] .................................................................................................... 2900/5297
[00:58:21] .................................................................................................... 3000/5297
[00:58:25] .................................................................................................... 3100/5297
[00:58:28] .................F..............................................................i................... 3200/5297
[00:58:34] .............................................ii...i..ii............................................. 3400/5297
[00:58:38] .................................................................................................... 3500/5297
[00:58:41] .................................................................................................... 3600/5297
[00:58:44] .....................................ii............................................................. 3700/5297
---
[00:59:39] 
[00:59:39] ---- [ui] ui/lint/type-overflow.rs stdout ----
[00:59:39] diff of stderr:
[00:59:39] 
[00:59:39] 12 LL |     let fail = 0b1000_0001i8; //~WARNING literal out of range for i8
[00:59:39] 13    |                ^^^^^^^^^^^^^ help: consider using `u8` instead: `0b1000_0001u8`
[00:59:39] 14    |
[00:59:39] -    = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `-127i8`
[00:59:39] +    = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `340282366920938463463374607431768211329i8`
[00:59:39] 16 
[00:59:39] 17 warning: literal out of range for i64
[00:59:39] 
[00:59:39] 
[00:59:39] 20 LL |     let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64
[00:59:39] 21    |                ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x8000_0000_0000_0000u64`
[00:59:39] 22    |
[00:59:39] -    = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `-9223372036854775808i64`
[00:59:39] +    = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `340282366920938463454151235394913435648i64`
[00:59:39] 24 
[00:59:39] 25 warning: literal out of range for u32
[00:59:39] 
[00:59:39] 
[00:59:39] 36 LL |     let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;
[00:59:39] 38    |
[00:59:39] 38    |
[00:59:39] -    = note: the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into an `i128` and will become `-170141183460469231731687303715884105728i128`
[00:59:39] +    = note: the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into an `i128` and will become `170141183460469231731687303715884105728i128`
[00:59:39] 40    = help: consider using `u128` instead
[00:59:39] 41 
[00:59:39] 42 warning: literal out of range for i32
[00:59:39] 
[00:59:39] 45 LL |     let fail = 0x8FFF_FFFF_FFFF_FFFE; //~WARNING literal out of range for i32
[00:59:39] 47    |
[00:59:39] 47    |
[00:59:39] -    = note: the literal `0x8FFF_FFFF_FFFF_FFFE` (decimal `10376293541461622782`) does not fit into an `i32` and will become `-2i32`
[00:59:39] +    = note: the literal `0x8FFF_FFFF_FFFF_FFFE` (decimal `10376293541461622782`) does not fit into an `i32` and will become `340282366920938463463374607431768211454i32`
[00:59:39] 49    = help: consider using `i128` instead
[00:59:39] 50 
[00:59:39] 51 warning: literal out of range for i8
[00:59:39] 
[00:59:39] 54 LL |     let fail = -0b1111_1111i8; //~WARNING literal out of range for i8
[00:59:39] 55    |                 ^^^^^^^^^^^^^ help: consider using `i16` instead: `0b1111_1111i16`
[00:59:39] 56    |
[00:59:39] -    = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `-1i8`
[00:59:39] +    = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `340282366920938463463374607431768211455i8`
[00:59:39] 59 
[00:59:39] 
[00:59:39] 
[00:59:39] The actual stderr differed from the expected stderr.
[00:59:39] The actual stderr differed from the expected stderr.
[00:59:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow/type-overflow.stderr
[00:59:39] To update references, rerun the tests and pass the `--bless` flag
[00:59:39] To only update this specific test, also pass `--test-args lint/type-overflow.rs`
[00:59:39] error: 1 errors occurred comparing output.
[00:59:39] status: exit code: 0
[00:59:39] status: exit code: 0
[00:59:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/type-overflow.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow/auxiliary" "-A" "unused"
[00:59:39] ------------------------------------------
[00:59:39] 
[00:59:39] ------------------------------------------
[00:59:39] stderr:
[00:59:39] stderr:
[00:59:39] ------------------------------------------
[00:59:39] {"message":"literal out of range for i8","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":45,"byte_end":50,"line_start":4,"line_end":4,"column_start":17,"column_end":22,"is_primary":true,"text":[{"text":"    let error = 255i8; //~WARNING literal out of range for i8","highlight_start":17,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(overflowing_literals)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i8\n  --> /checkout/src/test/ui/lint/type-overflow.rs:4:17\n   |\nLL |     let error = 255i8; //~WARNING literal out of range for i8\n   |                 ^^^^^\n   |\n   = note: #[warn(overflowing_literals)] on by default\n\n"}
[00:59:39] {"message":"literal out of range for i8","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":210,"byte_end":223,"line_start":9,"line_end":9,"column_start":16,"column_end":29,"is_primary":true,"text":[{"text":"    let fail = 0b1000_0001i8; //~WARNING literal out of range for i8","highlight_start":16,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `340282366920938463463374607431768211329i8`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `u8` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":210,"byte_end":223,"line_start":9,"line_end":9,"column_start":16,"column_end":29,"is_primary":true,"text":[{"text":"    let fail = 0b1000_0001i8; //~WARNING literal out of range for i8","highlight_start":16,"highlight_end":29}],"label":null,"suggested_replacement":"0b1000_0001u8","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i8\n  --> /checkout/src/test/ui/lint/type-overflow.rs:9:16\n   |\nLL |     let fail = 0b1000_0001i8; //~WARNING literal out of range for i8\n   |                ^^^^^^^^^^^^^ help: consider using `u8` instead: `0b1000_0001u8`\n   |\n   = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `340282366920938463463374607431768211329i8`\n\n"}
[00:59:39] {"message":"literal out of range for i64","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":280,"byte_end":304,"line_start":11,"line_end":11,"column_start":16,"column_end":40,"is_primary":true,"text":[{"text":"    let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64","highlight_start":16,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `340282366920938463454151235394913435648i64`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `u64` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":280,"byte_end":304,"line_start":11,"line_end":11,"column_start":16,"column_end":40,"is_primary":true,"text":[{"text":"    let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64","highlight_start":16,"highlight_end":40}],"label":null,"suggested_replacement":"0x8000_0000_0000_0000u64","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i64\n  --> /checkout/src/test/ui/lint/type-overflow.rs:11,"rendered":null}],"rendered":"warning: literal out of range for u32\n  --> /checkout/src/test/ui/lint/type-overflow.rs:13:16\n   |\nLL |     let fail = 0x1_FFFF_FFFFu32; //~WARNING literal out of range for u32\n   |                ^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x1_FFFF_FFFFu64`\n   |\n   = note: the literal `0x1_FFFF_FFFFu32` (decimal `8589934591`) does not fit into an `u32` and will become `4294967295u32`\n\n"}
[00:59:39] {"message":"literal out of range for i128","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":442,"byte_end":483,"line_start":15,"line_end":15,"column_start":22,"column_end":63,"is_primary":true,"text":[{"text":"    let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;","highlight_start":22,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into an `i128` and will become `170141183460469231731687303715884105728i128`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `u128` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i128\n  --> /checkout/src/test/ui/lint/type-overflow.rs:15:22\n   |\nLL |     let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;\n   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: the lie for i8","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":628,"byte_end":641,"line_start":20,"line_end":20,"column_start":17,"column_end":30,"is_primary":true,"text":[{"text":"    let fail = -0b1111_1111i8; //~WARNING literal out of range for i8","highlight_start":17,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `340282366920938463463374607431768211455i8`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `i16` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":628,"byte_end":641,"line_start":20,"line_end":20,"column_start":17,"column_end":30,"is_primary":true,"text":[{"text":"    let fail = -0b1111_1111i8; //~WARNING literal out of range for i8","highlight_start":17,"highlight_end":30}],"label":null,"suggested_replacement":"0b1111_1111i16","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i8\n  --> /checkout/src/test/ui/lint/type-overflow.rs:20:17\n   |\nLL |     let fail = -0b1111_1111i8; //~WARNING literal out of range for i8\n   |                 ^^^^^^^^^^^^^ help: consider using `i16` instead: `0b1111_1111i16`\n   |\n   = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `34028236692093846346337hon" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:39] 
[00:59:39] 
[00:59:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:39] Build completed unsuccessfully in 0:03:51
[00:59:39] Build completed unsuccessfully in 0:03:51
[00:59:39] Makefile:48: recipe for target 'check' failed
[00:59:39] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:232d8881
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 17 18:19:27 UTC 2019
---
travis_time:end:020ba840:start=1547749168881951168,finish=1547749168888142812,duration=6191644
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ba501a9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:115cd562
$ dmesg | grep -i kill
