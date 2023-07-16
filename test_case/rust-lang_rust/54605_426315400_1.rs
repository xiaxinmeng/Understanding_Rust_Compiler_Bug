\n\nIt's invalid to directly import methods belonging to a trait or concrete type.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0253.rs","byte_start":541,"byte_end":567,"line_start":17,"line_end":17,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"use foo::MyTrait::do_something;","highlight_start":5,"highlight_end":31}],"label":"cannot be imported directly","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0253]: `do_something` is not directly importable\n  --> /checkout/src/test/ui/error-codes/E0253.rs:17:5\n   |\nLL | use foo::MyTrait::do_something;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot be imported directly\n\n"}
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:16] {"message":"For more information about this error, try `rustc --explain E0253`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0253`.\n"}
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/error-codes/E0253.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/error-codes/E0253.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/error-codes/E0432.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0432.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0432/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0432/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/error-codes/E0432.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/error-codes/E0432.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/export-import.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/export-import.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/export-import/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/export-import/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/export-import.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/export-import.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/feature-gates/feature-gate-extern_absolute_paths.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-extern_absolute_paths.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-extern_absolute_paths/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-extern_absolute_paths/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/feature-gates/feature-gate-extern_absolute_paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/feature-gates/feature-gate-extern_absolute_paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/feature-gates/feature-gate-uniform-paths.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-uniform-paths.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-uniform-paths/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-uniform-paths/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/feature-gates/feature-gate-uniform-paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/feature-gates/feature-gate-uniform-paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/hidden-rt-injection.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hidden-rt-injection.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hidden-rt-injection/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hidden-rt-injection/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/hidden-rt-injection.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/hidden-rt-injection.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/hidden-rt-injection2.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hidden-rt-injection2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hidden-rt-injection2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hidden-rt-injection2/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/hidden-rt-injection2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/hidden-rt-injection2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/import.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/import.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/import/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/import/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/import.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/import.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/import2.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/import2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/import2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/import2/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/import2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/import2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/import4.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/import4.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/import4/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/import4/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/import4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/import4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/import3.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/import3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/import3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/import3/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/import3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/import3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/imports/import-from-missing.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/import-from-missing.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-from-missing/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-from-missing/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/imports/import-from-missing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/imports/import-from-missing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/imports/import-loop-2.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/import-loop-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-loop-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-loop-2/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:16] 
[00:47:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:47:16] 
[00:47:16] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/imports/import-loop-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] thread '[ui] ui/imports/import-loop-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:47:16] 
[00:47:16] ---- [ui] ui/imports/import-loop.rs stdout ----
[00:47:16] 
[00:47:16] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/import-loop.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-loop/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-loop/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] thread 'main' panicked at 'librustc_resolve/lib.rs:4613: parent module is reset for binding', librustc/util/bug.rs:47:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
[00:47:16] 
[00:47:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
---
[00:47:16] test result: FAILED. 4238 passed; 61 failed; 20 ignored; 0 measured; 0 filtered out
[00:47:16] 
[00:47:16] 
[00:47:16] 
[00:47:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:16] 
[00:47:16] 
[00:47:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:16] Build completed unsuccessfully in 0:03:12
[00:47:16] Build completed unsuccessfully in 0:03:12
[00:47:16] Makefile:58: recipe for target 'check' failed
[00:47:16] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:090e4800
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:13ceeb55:start=1538493686430324764,finish=1538493686435171330,duration=4846566
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01853604
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:29f96d54
travis_time:start:29f96d54
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0608ba04
$ dmesg | grep -i kill
