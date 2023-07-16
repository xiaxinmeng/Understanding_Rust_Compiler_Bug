\n\nOne fix may be to increase the recursion limit. Note that it is possible to\ncreate an infinite recursion of dereferencing, in which case the only fix is to\nsomehow break the recursion.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/recursion_limit_deref.rs","byte_start":1395,"byte_end":1397,"line_start":60,"line_end":60,"column_start":22,"column_end":24,"is_primary":true,"text":[{"text":"    let x: &Bottom = &t; //~ ERROR mismatched ty:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/dst/dst-bad-coerce1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/dst/dst-bad-coerce1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/dst/dst-bad-coerce2.rs stdout ----
[00:53:36] 
[00:53:36] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dst/dst-bad-coerce2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coerce2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coerce2/auxiliary" "-A" "unused"
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] stderr:
[00:53:36] stderr:
[00:53:36] ------------------------------------------
[00:53:36] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/demand.rs:428:38
[00:53:36] 
[00:53:36] error: internal compiler error: unexpected panic
[00:53:36] 
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] 
[00:53:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] 
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/dst/dst-bad-coerce2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/dst/dst-bad-coerce2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/dst/dst-bad-coercions.rs stdout ----
[00:53:36] 
[00:53:36] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dst/dst-bad-coercions.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coercions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coercions/auxiliary" "-A" "unused"
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] stderr:
[00:53:36] stderr:
[00:53:36] ----------used"
[00:53:36] stdout:
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] stderr:
[00:53:36] ------------------------------------------
[00:53:36] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/demand.rs:428:38
[00:53:36] 
[00:53:36] error: internal compiler error: unexpected panic
[00:53:36] 
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] 
[00:53:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] 
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/dst/dst-bad-coerce4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/dst/dst-bad-coerce4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/error-codes/E0070.rs stdout ----
[00:53:36] 
[00:53:36] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0070.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0070/a" "-Crpath" "-O" "-Zunstable-optiotruct.x = 12; // error : SomeStruct a structure name but it is used\n                       // like a variable!\n}\n