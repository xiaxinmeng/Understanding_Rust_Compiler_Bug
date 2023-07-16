plain
[00:00:48] configure: rust.quiet-tests     := True
---
[00:41:01] .........................................................................i..........................
[00:41:07] ................i...................................................................................
---
[00:41:42] ............................................................................................i.......
[00:41:49] ................................................................i...................................
---
[00:42:44] .............................................i......................................................
---
[00:46:40] .............................i......................................................................
[00:46:55] ..............................................................i.....................................
[00:47:11] ...............................................i....................................................
[00:47:31] ....................................................................................................
[00:47:53] ....................................................................................................
[00:48:15] ....................................................................................................
[00:48:40] .i................................................................................................i.
[00:49:07] .................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:49:17] ...................
[00:49:48] ....................................................................................................
[00:50:24] ...............................................................ii...................................
[00:51:09] ..........................i....................................................i.ii....test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:51:14] .............
[00:51:54] .......................................................................................iiiiiii......
---
[00:53:53] ....................................i...............................................................
[00:54:01] ....................................................................................................
[00:54:08] ..................i............................................................ii.iii...............
[00:54:16] ....................................................................................................
[00:54:24] ........i..............................i............................................................
[00:54:31] ....................................................................................................
[00:54:38] .....................i..............................................................................
[00:54:46] ....................................................................................................
[00:54:56] ....................................................................................................
[00:55:07] ....................................................................................................
[00:55:18] ....................................................................................................
[00:55:32] ....................................................................................................
[00:55:40] ..............i.....................................................................................
[00:55:50] .................i..ii..............................................................................
[00:56:00] ....................................................................................................
[00:56:11] ....................................................................................................
[00:56:20] ....................................................................................i...............
[00:56:31] ..............................i.....................................................................
---
[00:57:09] ...........................i........................................................................
[00:57:10] ....................................................................i...............................
[00:57:12] ................i.......................................................
---
[00:57:26] ...........i........................
---
[00:57:56] i...i..ii....i.............ii........iii......i..i...i...ii..i..i..ii.....
---
[00:57:59] i.......i......................i......
---
" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] ---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" failed to compile:
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b.stage2-x86_64-unknown-linux-gnu.aux"
[00:58:14] stdout:
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] ---------------------------------------inux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/change_crate_dep_kind.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_dep_kind.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Cpanic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C panic=unwind
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/change_crate_dep_kind.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/change_private_impl_method/struct_point.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/change_crate_order/main.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" failed to compile:
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" failed to compile:
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" failed to compile:
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/change_symbol_export_status.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/change_symbol_export_status.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/crate_hash_reorder.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/dirty_clean.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: compiler encountered internal error
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/dirty_clean.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/dirty_clean.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/extern_static/issue-49153.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/extern_static/issue-49153.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/extern_static/issue-49153.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/commandline-args.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass3`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=2
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stageflags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/call_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/closure_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/feature_gate.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: compiler encountered internal error
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/feature_gate.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/feature_gate.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/feature_gate.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/feature_gate.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
---
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/consts.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/consts.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/enum_constructors.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/extern_mods.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfaimd#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/extern_mods.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/exported_vs_not.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/enum_defs.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z que "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/indexing_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/function_interfaces.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/for_loops.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/for_loops.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/inline_asm.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/inline_asm.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/let_expressions.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/let_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/loop_expressions.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/loop_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls.stage2-x86_64-unknown-linux-gnu.aux"
[00:58:14] stdout:
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] ---------6_64-unknown-linux-gnu/test/incremental/hashes/match_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/match_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/statics.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/statics.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/statics.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/statics.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/statics.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/statics.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/struct_constructors.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/struct_constructors.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/struct_constructors.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/panic_exprs.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/panic_exprs.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debug-assertions
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/panic_exprs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/struct_defs.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/struct_defs.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_defs.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_defs.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/struct_defs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/type_defs.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/type_defs.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/type_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/type_defs.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/type_defs.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/type_defs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/trait_impls.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_impls.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/trait_impls.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/trait_defs.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_defs.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/traitremental] incremental/hashes/trait_defs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/while_let_loops.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_let_loops.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/while_let_loops.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/while_loops.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_loops.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/while_loops.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hashes/unary_and_binary_exprs.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/unary_and_binary_exprs.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hashes/unary_and_binary_exprs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/hello_world.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hello_world.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hello_world.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hello_world.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hello_world.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/hello_world.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/ich_nested_items.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_nested_items.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--ta
[00:58:14]
[00:58:14] thread '[incremental] incremental/ich_nested_items.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/ich_method_call_trait_scope.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_method_call_trait_scope.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_method_call_trait_scope.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_method_call_trait_scope.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_method_call_trait_scope.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
---
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_resolve_results.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/ich_resolve_results.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/inlined_hir_34991/main.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unkned: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/issue-35593.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/issue-38222.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-38222.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=1
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/issue-38222.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/issue-39569.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-39569.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/issue-39569.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/issue-42602.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-42602.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/issue-42602.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/issue-39828/issue-39828.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/issue-39828/auxiliary/generic.rs" failed to compile:
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-39828/auxiliary/generic.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/issue-39828/issue-39828.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/krate-inherent.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate-inherent.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/krate-inherent.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/macro_export.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/macro_export.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/macro_export.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/macro_export.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkoental/krate-inlined.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/krate-inlined.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/krate_reassign_34991/main.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/krate_reassign_34991/auxiliary/a.rs" failed to compile:
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate_reassign_34991/auxiliary/a.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate_reassign_34991/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate_reassign_34991/main.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate_reassign_34991/main.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/krate_reassign_34991/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/remapped_paths_cc/main.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/remapped_paths_cc/auxiliary/extern_crate.rs" failed to compile:
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remapped_paths_cc/auxiliary/extern_crate.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/remapped_paths_cc/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/remove_source_file/main.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remove_source_file/main.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove_source_file/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove_source_file/main.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove_source_file/main.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` f"-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/remove-private-item-cross-crate/main.rs' panicked at 'explicit panic', tools/compiletest/src/ted panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/remove_crate/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/source_loc_macros.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/source_loc_macros.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/source_loc_macros.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/rlib_cross_crate/b.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/rlib_cross_crate/auxiliary/a.rs" failed to compile:
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/rlib_cross_crate/auxiliary/a.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/rlib_cross_crate/b.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/span_hash_stable/main.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/span_hash_stable/main.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/span_hash_stable/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/spans_in_type_debuginfo.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_in_type_debuginfo.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/spans_in_type_debuginfo.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/spans_significant_w_debuginfo.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_significant_w_debuginfo.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/spans_significant_w_debuginfo.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/spans_significant_w_panic.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_significant_w_panic.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C overflow-checks=on
---
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spike.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C pread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/static_refering_to_other_static/issue-49081.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/static_refering_to_other_static2/issue.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/static_refering_to_other_static2/issue.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_refering_to_other_static2/issue.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_refering_to_other_static2/issue.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_refering_to_other_static2/issue.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath
---
[00:58:14] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/string_constant.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/string_constant.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/static_stable_hash/issue-49301.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/static_stable_hash/issue-49301.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_stable_hash/issue-49301.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_stable_hash/issue-49301.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_stable_hash/issue-49301.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/static_stable_hash/issue-49301.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/struct_add_field.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_add_field.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_add_field.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_add_field.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_add_field.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/struct_add_field.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/struct_change_field_name.rs stdout ----
[00:58:14]
[00:58:14] error in revision `cfail2`: compiler encountered internal error
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_change_field_name.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_name.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_name.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_name.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
---
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_change_field_type.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incrementa] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_change_nothing.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_nothing.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_nothing.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_nothing.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/struct_change_nothing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/struct_remove_field.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: compilation failed!
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_remove_field.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_remove_field.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_remove_field.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_remove_field.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/struct_remove_field.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/struct_change_field_type_cross_crate/b.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/struct_change_field_type_cross_crate/auxiliary/a.rs" failed to compile:
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_change_field_type_cross_crate/auxiliary/a.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type_cross_crate/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type_cross_crate/b.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type_cross_crate/b.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/struct_change_field_type_cross_crate/b.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14] ---- [incremental] incremental/type_alias_cross_crate/b.rs stdout ----
[00:58:14]
[00:58:14] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/type_alias_cross_crate/auxiliary/a.rs" failed to compile:
[00:58:14] status: exit code: 101
[00:58:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/type_alias_cross_crate/auxiliary/a.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/type_alias_cross_crate/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/type_alias_cross_crate/b.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/type_alias_cross_crate/b.stage2-x86_64-unknown-linux-gnu.aux"
[00:58:14] stdout:
[00:58:14] ----------------------------------------t/obj/build/x86_64-unknown-linux-gnu/test/incremental/unchecked_dirty_clean.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/unchecked_dirty_clean.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/unchecked_dirty_clean.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:58:14] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:58:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:14]
[00:58:14] error: internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[0 internal compiler error: unexpected panic
[00:58:14]
[00:58:14] note: the compiler unexpectedly panicked. this is a bug.
[00:58:14]
[00:58:14] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:14]
[00:58:14] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:58:14]
[00:58:14] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z miri -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C overflow-checks=on
[00:58:14]
[00:58:14]
[00:58:14] ------------------------------------------
[00:58:14]
[00:58:14] thread '[incremental] incremental/warnings-reemitted.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:58:14]
[00:58:14]
[00:58:14] failures:
[00:58:14]     [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs
[00:58:14]     [incremental] incremental/callee_caller_cross_crate/b.rs
[00:58:14]     [incremental] incremental/change_add_field/struct_point.rs
[00:58:14]     [incremental] incremental/change_crate_dep_kind.rs
[00:58:14]     [incremental] incremental/change_crate_order/main.rs
[00:58:14]     [incremental] incremental/change_private_fn/struct_point.rs
[00:58:14]     [incremental] incremental/change_private_fn_cc/struct_point.rs
[00:58:14]     [incremental] incremental/change_private_impl_method/struct_point.rs
[00:58:14]     [incremental] incremental/change_private_impl_method_cc/struct_point.rs
[00:58:14]     [incremental] incremental/change_pub_inherent_method_body/struct_point.rs
[00:58:14]     [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs
[00:58:14]     [incremental] incremental/change_symbol_export_status.rs
[00:58:14]     [incremental] incremental/commandline-args.rs
[00:58:14]     [incremental] incremental/crate_hash_reorder.rs
[00:58:14]     [incremental] incremental/dirty_clean.rs
[00:58:14]     [incremental] incremental/extern_static/issue-49153.rs
[00:58:14]     [incremental] incremental/feature_gate.rs
[00:58:14]     [incremental] incremental/hashes/call_expressions.rs
[00:58:14]     [incremental] incremental/hashes/closure_expressions.rs
[00:58:14]     [incremental] incremental/hashes/consts.rs
[00:58:14]     [incremental] incremental/hashes/enum_constructors.rs
[00:58:14]     [incremental] incremental/hashes/enum_defs.rs
[00:58:14]     [incremental] incremental/hashes/exported_vs_not.rs
[00:58:14]     [incremental] incremental/hashes/extern_mods.rs
[00:58:14]     [incremental] incremental/hashes/for_loops.rs
[00:58:14]     [incremental] incremental/hashes/function_interfaces.rs
[00:58:14]     [incremental] incremental/hashes/if_expressions.rs
[00:58:14]     [incremental] incremental/hashes/indexing_expressions.rs
[00:58:14]     [incremental] incremental/hashes/inherent_impls.rs
[00:58:14]     [incremental] incremental/hashes/inline_asm.rs
[00:58:14]     [incremental] incremental/hashes/let_expressions.rs
[00:58:14]     [incremental] incremental/hashes/loop_expressions.rs
[00:58:14]     [incremental] incremental/hashes/match_expressions.rs
[00:58:14]     [incremental] incremental/hashes/panic_exprs.rs
[00:58:14]     [incremental] incremental/hashes/statics.rs
[00:58:14]     [incremental] incremental/hashes/struct_constructors.rs
[00:58:14]     [incremental] incremental/hashes/struct_defs.rs
[00:58:14]     [incremental] incremental/hashes/trait_defs.rs
[00:58:14]     [incremental] incremental/hashes/trait_impls.rs
[00:58:14]     [incremental] incremental/hashes/type_defs.rs
[00:58:14]     [incremental] incremental/hashes/unary_and_binary_exprs.rs
[00:58:14]     [incremental] incremental/hashes/while_let_loops.rs
[00:58:14]     [incremental] incremental/hashes/while_loops.rs
[00:58:14]     [incremental] incremental/hello_world.rs
[00:58:14]     [incremental] incremental/ich_method_call_trait_scope.rs
[00:58:14] make: *** [check] Error 1
[00:58:14]     [incremental] incremental/ich_nested_items.rs
[00:58:14]     [incremental] incremental/ich_resolve_results.rs
[00:58:14]     [incremental] incremental/inlined_hir_34991/main.rs
[00:58:14]     [incremental] incremental/issue-35593.rs
[00:58:14]     [incremental] incremental/issue-38222.rs
[00:58:14]     [incremental] incremental/issue-39569.rs
[00:58:14]     [incremental] incremental/issue-39828/issue-39828.rs
[00:58:14]     [incremental] incremental/issue-42602.rs
[00:58:14]     [incremental] incremental/krate-inherent.rs
[00:58:14]     [incremental] incremental/krate-inlined.rs
[00:58:14]     [incremental] incremental/krate_reassign_34991/main.rs
[00:58:14]     [incremental] incremental/macro_export.rs
[00:58:14]     [incremental] incremental/remapped_paths_cc/main.rs
[00:58:14]     [incremental] incremental/remove-private-item-cross-crate/main.rs
[00:58:14]     [incremental] incremental/remove_crate/main.rs
[00:58:14]     [incremental] incremental/remove_source_file/main.rs
[00:58:14]     [incremental] incremental/rlib_cross_crate/b.rs
[00:58:14]     [incremental] incremental/source_loc_macros.rs
[00:58:14]     [incremental] incremental/span_hash_stable/main.rs
[00:58:14]     [incremental] incremental/spans_in_type_debuginfo.rs
[00:58:14]     [incremental] incremental/spans_significant_w_debuginfo.rs
[00:58:14]     [incremental] incremental/spans_significant_w_panic.rs
[00:58:14]     [incremental] incremental/spike.rs
[00:58:14]     [incremental] incremental/static_refering_to_other_static/issue-49081.rs
[00:58:14]     [incremental] incremental/static_refering_to_other_static2/issue.rs
[00:58:14]     [incremental] incremental/static_stable_hash/issue-49301.rs
[00:58:14]     [incremental] incremental/string_constant.rs
[00:58:14]     [incremental] incremental/struct_add_field.rs
[00:58:14]     [incremental] incremental/struct_change_field_name.rs
[00:58:14]     [incremental] incremental/struct_change_field_type.rs
[00:58:14]     [incremental] incremental/struct_change_field_type_cross_crate/b.rs
[00:58:14]     [incremental] incremental/struct_change_nothing.rs
[00:58:14]     [incremental] incremental/struct_remove_field.rs
[00:58:14]     [incremental] incremental/type_alias_cross_crate/b.rs
[00:58:14]     [incremental] incremental/unchecked_dirty_clean.rs
[00:58:14]     [incremental] incremental/warnings-reemitted.rs
[00:58:14]
[00:58:14] test result: FAILED. 5 passed; 81 failed; 0 ignored; 0 measured; 0 filtered out
[00:58:14]
[00:58:14]
[00:58:14]
[00:58:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:14] expected success, got: exit code: 101
[00:58:14]
[00:58:14]
[00:58:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:14] Build completed unsuccessfully in 0:18:06
[00:58:14] Makefile:58: recipe for target 'check' failed
---
$ cat obj/tmp/sccache.log
