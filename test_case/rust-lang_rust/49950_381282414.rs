plain
Resolving deltas: 100% (615434/615434), completed with 4902 local objects.
---
[00:00:45] configure: rust.quiet-tests     := True
---
[00:45:51] ...............................................................................i....................
[00:45:58] ......................i.............................................................................
---
[00:46:40] ...................i...........................................................................i....
[00:46:46] ....................................................................................................
[00:46:53] .........ii.........................................................................................
---
[00:47:39] .............................................i......................................................
---
[00:51:40] .............................i......................................................................
[00:51:54] ..............................................................i.....................................
[00:52:10] ................................................i...................................................
[00:52:30] ....................................................................................................
[00:52:51] ....................................................................................................
[00:53:13] ....................................................................................................
[00:53:38] ......i.............................................................................................
[00:54:06] ..i.................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:54:14] ................
[00:54:46] ....................................................................................................
[00:55:21] ....................................................................ii..............................
[00:56:08] ...............................i....................................................i.ii.test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:56:14] ...........
[00:56:57] ............................................................................................iiiiiii.
---
[00:59:08] ...............................................F....................................................
[00:59:15] ......F..........FFiF...........................................................ii.iii..............
[00:59:23] ....................................................................................................
[00:59:31] ........i..............................i.......................................................F....
[00:59:38] ....................................................................................................
[00:59:45] ..........i....F....................................................................................
[00:59:53] ...........F.F........F.....................F.................................................F.....
[01:00:03] ....................................................................................................
[01:00:13] .........F..........................................................................................
[01:00:23] .......................F............................................................................
[01:00:37] ....................................................................................................
[01:00:45] ....i...............................................................................................
[01:00:54] ........i..ii.......................................................................................
[01:01:05] ....................................................................................................
[01:01:14] ....................................................................................................
[01:01:23] ...............................................................F..........i.........................
[01:01:34] ....................i...............................................................................
---
[01:02:06] error: /checkout/src/test/compile-fail/coherence-inherited-assoc-ty-cycle-err.rs:19: expected message not found: cyclic dependency detected [E0391]
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/coherence-inherited-assoc-ty-cycle-err.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/coherence-inherited-assoc-ty-cycle-err.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/coherence-inherited-assoc-ty-cycle-err.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 19,
[01:02:06]         kind: None,
[01:02:06]         msg: "cyclic dependency detected [E0391]"
[01:02:06]     }
[01:02:06] ]
[01:02:06]
[01:02:06] thread '[compile-fail] compile-fail/coherence-inherited-assoc-ty-cycle-err.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:02:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:06]
[01:02:06] ---- [compile-fail] compile-fail/const-size_of-cycle.rs stdout ----
[01:02:06]
[01:02:06] error: error pattern ' cyclic dependency detected' not found!
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-size_of-cycle.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-size_of-cycle.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-size_of-cycle.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[01:02:06] error[E0391]: cycle detected when computing layout of `Foo`
[01:02:06]    |
[01:02:06] note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: Slice([]), reveal: All }, value: [u8; _] }`...
[01:02:06] note: ...which requires const-evaluating `Foo::{{initializer}}`...
[01:02:06]   --> /checkout/src/test/compile-fail/const-size_of-cycle.rs:16:17
[01:02:06]    |
[01:02:06] LL |     bytes: [u8; std::mem::size_of::<Foo>()]
[01:02:06]    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:02:06] note: ...which again requires computing layout of `Foo`, completing the cycle
[01:02:06]   --> /checkout/src/libcore/mem.rs:316:14
[01:02:06]    |
[01:02:06] LL |     unsafe { intrinsics::size_of::<T>() }
[01:02:06]    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:02:06] note: cycle used when const-evaluating `Foo::{{initializer}}`
[01:02:06]   --> /checkout/src/libcore/mem.rs:316:14
[01:02:06]    |
[01:02:06] LL |     unsafe { intrinsics::size_of::<T>() }
---
[01:02:06] error: /checkout/src/test/compile-fail/cycle-trait-default-type-trait.rs:14: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/cycle-trait-default-type-trait.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cycle-trait-default-type-trait.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cycle-trait-default-type-trait.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 14,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "cyclic dependency detected"
---
[01:02:06] error: /checkout/src/test/compile-fail/cycle-projection-based-on-where-clause.rs:27: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/cycle-projection-based-on-where-clause.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cycle-projection-based-on-where-clause.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cycle-projection-based-on-where-clause.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 27,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "cyclic dependency detected"
---
[01:02:06] error: /checkout/src/test/compile-fail/cycle-trait-supertrait-direct.rs:13: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/cycle-trait-supertrait-direct.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cycle-trait-supertrait-direct.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cycle-trait-supertrait-direct.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 13,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "cyclic dependency detected"
---
[01:02:06] error: /checkout/src/test/compile-fail/infinite-vec-type-recursion.rs:11: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/infinite-vec-type-recursion.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/infinite-vec-type-recursion.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zwn-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 11,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "E0391"
[01:02:06]     }
[01:02:06] ]
[01:02:06]
[01:02:06] thread '[compile-fail] compile-fail/issue-17252.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:02:06]
[01:02:06] ---- [compile-fail] compile-fail/issue-20772.rs stdout ----
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/issue-20772.rs:11: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-20772.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20772.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20772.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 11,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "cyclic dependency detected"
[01:02:06]     }
[01:02:06] ]
[01:02:06]
[01:02:06] thread '[compile-fail] compile-fail/issue-20772.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:02:06]
[01:02:06] ---- [compile-fail] compile-fail/issue-20825.rs stdout ----
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/issue-20825.rs:15: expected error not found: cyclic dependency detected [E0391]
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-20825.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20825.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20825.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 15,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "cyclic dependency detected [E0391]"
[01:02:06]     }
[01:02:06] ]
[01:02:06]
[01:02:06] thread '[compile-fail] compile-fail/issue-20825.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:02:06]
[01:02:06] ---- [compile-fail] compile-fail/issue-21177.rs stdout ----
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/issue-21177.rs:16: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-21177.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-21177.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-21177.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 16,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "cyclic dependency detected"
[01:02:06]     }
[01:02:06] ]
[01:02:06]
[01:02:06] thread '[compile-fail] compile-fail/issue-21177.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:02:06]
[01:02:06] ---- [compile-fail] compile-fail/issue-22673.rs stdout ----
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/issue-22673.rs:11: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-22673.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-22673.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-22673.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 11,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "cyclic dependency detected"
[01:02:06]     }
[01:02:06] ]
[01:02:06]
[01:02:06] thread '[compile-fail] compile-fail/issue-22673.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:02:06]
[01:02:06] ---- [compile-fail] compile-fail/issue-26548.rs stdout ----
[01:02:06]
[01:02:06] error: error pattern ' cyclic dependency detected' not found!
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-26548.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-26548.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-26548.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[01:02:06] error[E0391]: cycle detected when computing layout of `std::option::Option<S>`
[01:02:06]    |
[01:02:06] note: ...which requires computing layout of `S`...
[01:02:06] note: ...which again requires computing layout of `std::option::Option<S>`, completing the cycle
[01:02:06] note: cycle used when compile_codegen_unit
---
[01:02:06] thread '[compile-fail] compile-fail/issue-26548.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:02:06]
[01:02:06] ---- [compile-fail] compile-fail/issue-34373.rs stdout ----
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/issue-34373.rs:18: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-34373.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-34373.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-34373.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 18,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "cyclic dependency detected"
[01:02:06]     }
[01:02:06] ]
[01:02:06]
[01:02:06] thread '[compile-fail] compile-fail/issue-34373.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:02:06]
[01:02:06] ---- [compile-fail] compile-fail/issue-44415.rs stdout ----
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/issue-44415.rs:17: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 1 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-44415.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-44415.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-44415.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 17,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "cyclic dependency detected"
[01:02:06]     }
[01:02:06] ]
[01:02:06]
[01:02:06] thread '[compile-fail] compile-fail/issue-44415.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:02:06]
[01:02:06] ---- [compile-fail] compile-fail/resolve-self-in-impl.rs stdout ----
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/resolve-self-in-impl.rs:24: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/resolve-self-in-impl.rs:25: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/resolve-self-in-impl.rs:26: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/resolve-self-in-impl.rs:27: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: /checkout/src/test/compile-fail/resolve-self-in-impl.rs:28: expected error not found: cyclic dependency detected
[01:02:06]
[01:02:06] error: 0 unexpected errors found, 5 expected errors not found
[01:02:06] status: exit code: 101
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/resolve-self-in-impl.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/resolve-self-in-impl.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/resolve-self-in-impl.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:02:06] not found errors (from test file): [
[01:02:06]     Error {
[01:02:06]         line_num: 24,
[01:02:06]         kind: Some(
[01:02:06]             Error
[01:02:06]         ),
[01:02:06]         msg: "cyclic dependency detected"
---
[01:02:06]         msg: "cyclic dependency detected"
---
[01:02:06]         msg: "cyclic dependency detected"
---
[01:02:06]         msg: "cyclic dependency detected"
---
[01:02:06]         msg: "cyclic dependency detected"
[01:02:06]     }
[01:02:06] ]
[01:02:06]
[01:02:06] thread '[compile-fail] compile-fail/resolve-self-in-impl.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
---
[01:02:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:06] expected success, got: exit code: 101
[01:02:06]
[01:02:06]
[01:02:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:06] Build completed unsuccessfully in 0:16:53
[01:02:06] make: *** [check] Error 1
[01:02:06] Makefile:58: recipe for target 'check' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:1ba4d337:start=1523661484867905463,finish=1523661484878343870,duration=10438407
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:04069336
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:04069336:start=1523661484884521421,finish=1523661484891187311,duration=6665890
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dcab874
$ dmesg | grep -i kill
[   10.363451] init: failsafe main process (1094) killed by TERM signal
