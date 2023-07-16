plain
travis_time:end:037be9be:start=1546428278359775563,finish=1546428334391924669,duration=56032149106
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:57] .................................................................................................... 2700/5223
[01:00:00] .................................................................................................... 2800/5223
[01:00:03] .................................................................................................... 2900/5223
[01:00:06] .................................................................................................... 3000/5223
[01:00:10] ....................................................F............................................... 3100/5223
[01:00:17] ...........................................................................ii...i..ii............... 3300/5223
[01:00:21] .................................................................................................... 3400/5223
[01:00:24] .................................................................................................... 3500/5223
[01:00:27] ...............................................................ii................................... 3600/5223
---
[01:01:24] 
[01:01:24] ---- [ui] ui/lint/type-overflow.rs stdout ----
[01:01:24] diff of stderr:
[01:01:24] 
[01:01:24] 12 LL |     let fail = 0b1000_0001i8; //~WARNING literal out of range for i8
[01:01:24] 13    |                ^^^^^^^^^^^^^ help: consider using `u8` instead: `0b1000_0001u8`
[01:01:24] 14    |
[01:01:24] -    = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `-127i8`
[01:01:24] +    = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `340282366920938463463374607431768211329i8`
[01:01:24] 16 
[01:01:24] 17 warning: literal out of range for i64
[01:01:24] 
[01:01:24] 
[01:01:24] 20 LL |     let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64
[01:01:24] 21    |                ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x8000_0000_0000_0000u64`
[01:01:24] 22    |
[01:01:24] -    = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `-9223372036854775808i64`
[01:01:24] +    = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `340282366920938463454151235394913435648i64`
[01:01:24] 24 
[01:0l out of range for i8","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":45,"byte_end":50,"line_start":4,"line_end":4,"column_start":17,"column_end":22,"is_primary":true,"text":[{"text":"    let error = 255i8; //~WARNING literal out of range for i8","highlight_start":17,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(overflowing_literals)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i8\n  --> /checkout/src/test/ui/lint/type-overflow.rs:4:17\n   |\nLL |     let error = 255i8; //~WARNING literal out of range for i8\n   |                 ^^^^^\n   |\n   = note: #[warn(overflowing_literals)] on by default\n\n"}
[01:01:24] {"message":"literal out of range for i8","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":210,"byte_end":223,"line_start":9,"line_end":9,"column_start":16,"column_end":29,"is_primary":true,"text":[{"text":"    let fail = 0b1000_0001i8; //~WARNING literal out of range for i8","highlight_start":16,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `340282366920938463463374607431768211329i8`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `u8` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":210,"byte_end":223,"line_start":9,"line_end":9,"column_start":16,"column_end":29,"is_primary":true,"text":[{"text":"    let fail = 0b1000_0001i8; //~WARNING literal out of range for i8","highlight_start":16,"highlight_end":29}],"label":null,"suggested_replacement":"0b1000_0001u8","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i8\n  --> /checkout/src/test/ui/lint/type-overflow.rs:9:16\n   |\nLL |     let fail = 0b1000_0001i8; //~WARNING literal out of range for i8\n   |                ^^^^^^^^^^^^^ help: consider using `u8` instead: `0b1000_0001u8`\n   |\n   = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `340282366920938463463374607431768211329i8`\n\n"}
[01:01:24] {"message":"literal out of range for i64","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":280,"byte_end":304,"line_start":11,"line_end":11,"column_start":16,"column_end":40,"is_primary":true,"text":[{"text":"    let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64","highlight_start":16,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `340282applicability":null,"expansion":null}],"children":[{"message":"the literal `0x1_FFFF_FFFFu32` (decimal `8589934591`) does not fit into an `u32` and will become `4294967295u32`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `u64` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":362,"byte_end":378,"line_start":13,"line_end":13,"column_start":16,"column_end":32,"is_primary":true,"text":[{"text":"    let fail = 0x1_FFFF_FFFFu32; //~WARNING literal out of range for u32","highlight_start":16,"highlight_end":32}],"label":null,"suggested_replacement":"0x1_FFFF_FFFFu64","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for u32\n  --> /checkout/src/test/ui/lint/type-overflow.rs:13:16\n   |\nLL |     let fail = 0x1_FFFF_FFFFu32; //~WARNING literal out of range for u32\n   |                ^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x1_FFFF_FFFFu64`\n   |\n   = note: the literal `0x1_FFFF_FFFFu32` (decimal `8589934591`) does not fit into an `u32` and will become `4294967295u32`\n\n"}
[01:01:24] {"message":"literal out of range for i128","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/type-overflow.rs","byte_start":442,"byte_end":483,"line_start":15,"line_end":15,"column_start":22,"column_end":63,"is_primary":true,"text":[{"text":"    let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;","highlight_start":22,"highlight_end":63}],"label":nue_start":20,"line_end":20,"column_start":17,"column_end":30,"is_primary":true,"text":[{"text":"    let fail = -0b1111_1111i8; //~WARNING literal out of range for i8","highlight_start":17,"highlight_end":30}],"label":null,"suggested_replacement":"0b1111_1111i16","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i8\n  --> /checkout/src/test/ui/lint/type-overflow.rs:20:17\n   |\nLL |     let fail = -0b1111_1111i8; //~WARNING literal out of range for i8\n   |                 ^^^^^^^^^^^^^ help: consider using `i16` instead: `0b1111_1111i16`\n   |\n   = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `340282366920938463463374607431768211455i8`\n\n"}
[01:01:24] ------------------------------------------
[01:01:24] 
[01:01:24] thread '[ui] ui/lint/type-overflow.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:01:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
travis_time:end:0ca12f7a:start=1546432028590343710,finish=1546432028595687994,duration=5344284
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0950cb8c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb 
