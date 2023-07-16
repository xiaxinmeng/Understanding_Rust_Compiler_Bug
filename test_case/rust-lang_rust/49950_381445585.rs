plain
[00:00:49] configure: rust.quiet-tests     := True
---
[00:45:02] .................................................................................i..................
[00:45:08] ........................i...........................................................................
---
[00:45:50] .......................i...........................................................................i
[00:45:56] ....................................................................................................
[00:46:02] .............ii.....................................................................................
---
[00:46:48] .............................................i......................................................
---
[00:50:41] ..............................i.....................................................................
[00:50:55] ...............................................................i....................................
[00:51:10] .................................................i..................................................
[00:51:30] ....................................................................................................
[00:51:51] ....................................................................................................
[00:52:12] ....................................................................................................
[00:52:37] .......i............................................................................................
[00:53:05] ...i...................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:53:12] .............
[00:53:44] ....................................................................................................
[00:54:15] .....................................................................ii.............................
[00:55:06] ................................i....................................................i.ii...test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:55:10] ........
[00:55:52] .............................................................................................iiiiiii
---
[00:57:58] ................................................F...................................................
[00:58:05] ......F..........FFiF............................................................iiiii..............
[00:58:12] ....................................................................................................
[00:58:20] ........i..............................i.......................................................F....
[00:58:27] ....................................................................................................
[00:58:34] ..........i.........................................................................................
[00:58:42] ............FF......F......................F.................................................F......
[00:58:51] ....................................................................................................
[00:59:02] ........F...........................................................................................
[00:59:12] ......................F.............................................................................
[00:59:25] ....................................................................................................
[00:59:34] ...i................................................................................................
[00:59:43] .......i..ii........................................................................................
[00:59:53] ....................................................................................................
[01:00:02] ....................................................................................................
[01:00:11] ..............................................................F..........i..........................
[01:00:22] ...................i................................................................................
---
[01:00:54] error: /checkout/src/test/compile-fail/coherence-inherited-assoc-ty-cycle-err.rs:19: unexpected error: '19:1: 19:15: cycle detected when processing `Trait` [E0391]'
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/coherence-inherited-assoc-ty-cycle-err.rs:19: expected message not found: cyclic dependency detected [E0391]
[01:00:54]
[01:00:54] error: 1 unexpected errors found, 1 expected errors not found
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/coherence-inherited-assoc-ty-cycle-err.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/coherence-inherited-assoc-ty-cycle-err.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/coherence-inherited-assoc-ty-cycle-err.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:54] unexpected errors (from JSON output): [
[01:00:54]     Error {
[01:00:54]         line_num: 19,
[01:00:54]         kind: Some(
[01:00:54]             Error
[01:00:54]         ),
[01:00:54]         msg: "19:1: 19:15: cycle detected when processing `Trait` [E0391]"
[01:00:54]     }
[01:00:54] ]
[01:00:54]
[01:00:54] not found errors (from test file): [
[01:00:54]     Error {
[01:00:54]         line_num: 19,
[01:00:54]         kind: None,
[01:00:54]         msg: "cyclic dependency detected [E0391]"
[01:00:54]     }
[01:00:54] ]
[01:00:54]
[01:00:54] thread '[compile-fail] compile-fail/coherence-inherited-assoc-ty-cycle-err.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:00:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:54]
[01:00:54] ---- [compile-fail] compile-fail/const-size_of-cycle.rs stdout ----
[01:00:54]
[01:00:54] error: error pattern ' cyclic dependency detected' not found!
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-size_of-cycle.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-size_of-cycle.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-size_of-cycle.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:54] stdout:
[01:00:54] ------------------------------------------
[01:00:54]
[01:00:54] -------------------------------:00:54]     Error {
[01:00:54]         line_num: 27,
[01:00:54]         kind: Some(
[01:00:54]             Error
[01:00:54]         ),
[01:00:54]         msg: "cyclic dependency detected"
---
[01:00:54] error: /checkout/src/test/compile-fail/cycle-trait-default-type-trait.rs:14: unexpected error: '14:19: 14:22: cycle detected when processing `Foo::X` [E0391]'
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/cycle-trait-default-type-trait.rs:14: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: 1 unexpected errors found, 1 expected errors not found
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/cycle-trait-default-type-trait.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cycle-trait-default-type-trait.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cycle-trait-default-type-trait.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:54] unexpected errors (from JSON output): [
[01:00:54]     Error {
[01:00:54]         line_num: 14,
[01:00:54]         kind: Some(
[01:00:54]             Error
[01:00:54]         ),
[01:00:54]         msg: "14:19: 14:22: cycle detected when processing `Foo::X` [E0391]"
---
[01:00:54]         msg: "cyclic dependency detected"
---
[01:00:54] error: /checkout/src/test/compile-fail/cycle-trait-supertrait-direct.rs:13: unexpected error: '13:1: 13:29: cycle detected when computing the supertraits of `Chromosome` [E0391]'
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/cycle-trait-supertrait-direct.rs:13: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: 1 unexpected errors found, 1 expected errors not found
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/cycle-trait-supertrait-direct.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "-- cyclic dependency detected
[01:00:54]
[01:00:54] error: 1 unexpected errors found, 1 expected errors not found
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/infinite-vec-type-recursion.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/infinite-vec-type-recursion.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/infinite-vec-type-recursion.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:54] unexpected errors (from JSON output): [
[01:00:54]     Error {
[01:00:54]         line_num: 11,
[01:00:54]         kind: Some(
[01:00:54]             Error
[01:00:54]         ),
[01:00:54]         msg: "11:14: 11:15: cycle detected when processing `x` [E0391]"
---
[01:00:54]         msg: "cyclic dependency detected"
[01:00:54]     }
[01:00:54] ]
[01:00:54]
[01:00:54] thread '[compile-fail] compile-fail/infinite-vec-type-recursion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:00:54]
[01:00:54] ---- [compile-fail] compile-fail/issue-20772.rs stdout ----
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-20772.rs:11: unexpected error: '11:1: 14:3: cycle detected when computing the supertraits of `T` [E0391]'
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-20772.rs:11: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: 1 unexpected errors found, 1 expected errors not found
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-20772.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20772.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20772.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:54] unexpected errors (from JSON output): [
[01:00:54]     Error {
[01:00:54]         line_num: 11,
[01:00:54]         kind: Some(
[01:00:54]             Error
[01:00:54]         ),
[01:00:54]         msg: "11:1: 14:3: cycle detected when computing the supertraits of `T` [E0391]"
---
[01:00:54]         msg: "cyclic dependency detected"
[01:00:54]     }
[01:00:54] ]
[01:00:54]
[01:00:54] thread '[compile-fail] compile-fail/issue-20772.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:00:54]
[01:00:54] ---- [compile-fail] compile-fail/issue-20825.rs stdout ----
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-20825.rs:15: unexpected error: '15:1: 15:53: cycle detected when computing the supertraits of `Processor` [E0391]'
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-20825.rs:15: expected error not found: cyclic dependency detected [E0391]
[01:00:54]
[01:00:54] error: 1 unexpected errors found, 1 expected errors not found
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-20825.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20825.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20825.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:54] unexpected errors (from JSON output): [
[01:00:54]     Error {
[01:00:54]         line_num: 15,
[01:00:54]         kind: Some(
[01:00:54]             Error
[01:00:54]         ),
[01:00:54]         msg: "15:1: 15:53: cycle detected when computing the supertraits of `Processor` [E0391]"
---
[01:00:54]         msg: "cyclic dependency detected [E0391]"
[01:00:54]     }
[01:00:54] ]
[01:00:54]
[01:00:54] thread '[compile-fail] compile-fail/issue-20825.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:00:54]
[01:00:54] ---- [compile-fail] compile-fail/issue-21177.rs stdout ----
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-21177.rs:16: unexpected error: '16:21: 16:25: cycle detected when computing the bounds for type parameter `T` [E0391]'
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-21177.rs:16: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: 1 unexpected errors found, 1 expected errors not found
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-21177.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-21177.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-21177.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:54] unexpected errors (from JSON output): [
[01:00:54]     Error {
[01:00:54]         line_num: 16,
[01:00:54]         kind: Some(
[01:00:54]             Error
[01:00:54]         ),
[01:00:54]         msg: "16:21: 16:25: cycle detected when computing the bounds for type parameter `T` [E0391]"
---
[01:00:54]         msg: "cyclic dependency detected"
[01:00:54]     }
[01:00:54] ]
[01:00:54]
[01:00:54] thread '[compile-fail] compile-fail/issue-21177.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:00:54]
[01:00:54] ---- [compile-fail] compile-fail/issue-22673.rs stdout ----
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-22673.rs:11: unexpected error: '11:1: 11:35: cycle detected when computing the supertraits of `Expr` [E0391]'
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-22673.rs:11: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: 1 unexpected errors found, 1 expected errors not found
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-22673.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-22673.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-22673.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:54] unexpected errors (from JSON output): [
[01:00:54]     Error {
[01:00:54]         line_num: 11,
[01:00:54]         kind: Some(
[01:00:54]             Error
[01:00:54]         ),
[01:00:54]         msg: "11:1: 11:35: cycle detected when computing the supertraits of `Expr` [E0391]"
---
[01:00:54]         msg: "cyclic dependency detected"
[01:00:54]     }
[01:00:54] ]
[01:00:54]
[01:00:54] thread '[compile-fail] compile-fail/issue-22673.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:00:54]
[01:00:54] ---- [compile-fail] compile-fail/issue-26548.rs stdout ----
[01:00:54]
[01:00:54] error: error pattern ' cyclic dependency detected' not found!
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-26548.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-26548.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-26548.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[01:00:54] error[E0391]: cycle detected when computing layout of `std::option::Option<S>`
[01:00:54]    |
[01:00:54] note: ...which requires computing layout of `S`...
[01:00:54]    = note: ...which again requires computing layout of `std::option::Option<S>`, completing the cycle
[01:00:54] note: cycle used when compile_codegen_unit
---
[01:00:54] thread '[compile-fail] compile-fail/issue-26548.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:00:54]
[01:00:54] ---- [compile-fail] compile-fail/issue-34373.rs stdout ----
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-34373.rs:17: unexpected error: '17:30: 17:40: ' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1254:13
[01:00:54]
[01:00:54] ---- [compile-fail] compile-fail/issue-44415.rs stdout ----
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-44415.rs:1: unexpected error: '1:1: 1:1: cycle detected when computing layout of `Foo` [E0391]'
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/issue-44415.rs:17: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: 1 unexpected errors found, 1 expected errors not found
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-44415.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-44415.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-44415.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:54] unexpected errors (from JSON output): [
[01:00:54]     Error {
[01:00:54]         line_num: 1,
[01:00:54]         kind: Some(
[01:00:54]             Error
[01:00:54]         ),
[01:00:54]         msg: "1:1: 1:1: cycle detected when computing layout of `Foo` [E0391]"
---
[01:00:54] error: /checkout/src/test/compile-fail/resolve-self-in-impl.rs:24: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/resolve-self-in-impl.rs:25: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/resolve-self-in-impl.rs:26: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/resolve-self-in-impl.rs:27: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: /checkout/src/test/compile-fail/resolve-self-in-impl.rs:28: expected error not found: cyclic dependency detected
[01:00:54]
[01:00:54] error: 5 unexpected errors found, 5 expected errors not found
[01:00:54] status: exit code: 101
[01:00:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/resolve-self-in-impl.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/resolve-self-in-impl.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/resolve-self-in-impl.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:54] unexpected errors (from JSON output): [
[01:00:54]     Error {
[01:00:54]         line_pertrait-direct.rs
[01:00:54]     [compile-fail] compile-fail/infinite-vec-type-recursion.rs
[01:00:54]     [compile-fail] compile-fail/issue-20772.rs
[01:00:54]     [compile-fail] compile-fail/issue-20825.rs
[01:00:54]     [compile-fail] compile-fail/issue-21177.rs
[01:00:54]     [compile-fail] compile-fail/issue-22673.rs
[01:00:54]     [compile-fail] compile-fail/issue-26548.rs
[01:00:54]     [compile-fail] compile-fail/issue-34373.rs
[01:00:54]     [compile-fail] compile-fail/issue-44415.rs
[01:00:54]     [compile-fail] compile-fail/resolve-self-in-impl.rs
[01:00:54]
[01:00:54] test result: FAILED. 2281 passed; 14 failed; 15 ignored; 0 measured; 0 filtered out
[01:00:54]
[01:00:54]
[01:00:54]
[01:00:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:54] expected success, got: exit code: 101
[01:00:54]
[01:00:54]
[01:00:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:54] Build completed unsuccessfully in 0:16:30
[01:00:54] Makefile:58: recipe for target 'check' failed
[01:00:54] make: *** [check] Error 1
---
55760 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08/s-f05j2vyycc-ggfnk5-3irgqbf4ljze2
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:29ca0f0c:start=1523833730266935852,finish=1523833730274126485,duration=7190633
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0b652510
$ find $HOME/Library/Ltravis_time:start:00651a3c
$ dmesg | grep -i kill
[   10.610675] init: failsafe main process (1093) killed by TERM signal
