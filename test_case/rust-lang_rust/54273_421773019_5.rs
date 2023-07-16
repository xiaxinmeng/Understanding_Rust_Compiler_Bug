\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generic/generic-arg-mismatch-recover.rs","byte_start":554,"byte_end":561,"line_start":16,"line_end":16,"column_start":20,"column_end":27,"is_primary":true,"text":[{"text":"    Foo::<'static, 'static, ()>(&0); //~ ERROR wrong number of lifetime arguments","highlight_start":20,"highlight_end":27}],"label":"unexpected lifetime argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of lifetime arguments: expected 1, found 2\n  --> /checkout/src/test/ui/generic/generic-arg-mismatch-recover.rs:16:20\n   |\nLL |     Foo::<'static, 'static, ()>(&0); //~ ERROR wrong number of lifetime arguments\n   |                    ^^^^^^^ unexpected lifetime argument\n\n"}
[00:53:36] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/demand.rs:428:38
[00:53:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:36] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:53:36] {"message":"For more information about this error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0107`.\n"}
[00:53:36] error: internal compiler error: unexpected panic
[00:53:36] 
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] 
[00:53:36] 
[00:53:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] 
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/generic/generic-arg-mismatch-recover.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/generic/generic-arg-mismatch-recover.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/hr-subtype/hr-subtype.rs#bound_a_b_ret_a_vs_bound_a_ret_a stdout ----
[00:53:36] 
[00:53:36] error in revision `bound_a_b_ret_a_vs_bound_a_ret_a`: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/hr-subtype.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "bound_a_b_ret_a_vs_bound_a_ret_a" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-sus Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/hr-subtype.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "bound_a_b_vs_bound_a" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_a_b_vs_bound_a/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_a_b_vs_bound_a/auxiliary" "-A" "unused"
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
[00:53:36] 
[00:53:36] threaverflow', librustc_typeck/check/demand.rs:428:38
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
[00:53:36] thread '[ui] ui/hr-subtype/hr-subtype.rs#bound_co_a_b_vs_bound_co_a' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/hr-subtype/hr-subtype.rs#bound_co_a_b_vs_bound_co_a' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/hr-subtype/hr-subtype.rs#bound_co_a_co_b_ret_contra_a stdout ----
[00:53:36] 
[00:53:36] error in revision `bound_co_a_co_b_ret_contra_a`: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/hr-subtype.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "bound_co_a_co_b_ret_contra_a" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_co_a_co_b_ret_contra_a/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/bui/hr-subtype.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "bound_contra_a_contra_b_ret_co_a" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_contra_a_contra_b_ret_co_a/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.bound_contra_a_contra_b_ret_co_a/auxiliary" "-A" "unused"
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
[00:53:36] 
[00:53:36] thread '[ui] ui/hr-subtype/hr-subtype.rs#bound_contra_a_contra_b_ret_co_a' panicked at 'explicit panic', tools/compiletest/src/runtest.rr inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n