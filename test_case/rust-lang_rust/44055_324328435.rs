
[00:47:25] failures:
[00:47:25] 
[00:47:25] ---- [compile-fail] compile-fail/deriving-non-type.rs stdout ----
[00:47:25] 	
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:15: unexpected "error": '15:1: 15:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:18: unexpected "error": '18:1: 18:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:21: unexpected "error": '21:1: 21:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:24: unexpected "error": '24:1: 24:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:27: unexpected "error": '27:1: 27:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:30: unexpected "error": '30:1: 30:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:33: unexpected "error": '33:1: 33:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:36: unexpected "error": '36:1: 36:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:39: unexpected "error": '39:1: 39:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:15: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:18: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:21: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:24: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:27: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:30: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:33: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:36: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/deriving-non-type.rs:39: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: 9 unexpected errors found, 9 expected errors not found
[00:47:25] status: exit code: 101
[00:47:25] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/deriving-non-type.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/deriving-non-type.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -A unused -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/deriving-non-type.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:47:25] unexpected errors (from JSON output): [
[00:47:25]     Error {
[00:47:25]         line_num: 15,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "15:1: 15:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 18,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "18:1: 18:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 21,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "21:1: 21:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 24,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "24:1: 24:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 27,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "27:1: 27:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 30,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "30:1: 30:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 33,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "33:1: 33:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 36,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "36:1: 36:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 39,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "39:1: 39:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     }
[00:47:25] ]
[00:47:25] 
[00:47:25] not found errors (from test file): [
[00:47:25]     Error {
[00:47:25]         line_num: 15,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 18,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 21,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 24,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 27,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 30,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 33,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 36,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 39,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     }
[00:47:25] ]
[00:47:25] 
[00:47:25] thread '[compile-fail] compile-fail/deriving-non-type.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1133:12
[00:47:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:25] 
[00:47:25] ---- [compile-fail] compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs stdout ----
[00:47:25] 	
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:15: unexpected "error": '15:1: 15:18: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:18: unexpected "error": '18:1: 18:17: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:21: unexpected "error": '21:17: 21:34: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:24: unexpected "error": '24:5: 24:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:40: unexpected "error": '40:5: 40:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:44: unexpected "error": '44:5: 44:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:15: expected error not found: cannot find derive macro `x3300` in this scope
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:18: expected error not found: cannot find derive macro `x3300` in this scope
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:21: expected error not found: cannot find derive macro `x3300` in this scope
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:24: expected error not found: cannot find derive macro `x3300` in this scope
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:28: expected error not found: cannot find derive macro `x3300` in this scope
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:32: expected error not found: cannot find derive macro `x3300` in this scope
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:36: expected error not found: cannot find derive macro `x3300` in this scope
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:40: expected error not found: cannot find derive macro `x3300` in this scope
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs:44: expected error not found: cannot find derive macro `x3300` in this scope
[00:47:25] 
[00:47:25] error: 6 unexpected errors found, 9 expected errors not found
[00:47:25] status: exit code: 101
[00:47:25] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -A unused -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate/issue-43106-gating-of-derive-2.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:47:25] unexpected errors (from JSON output): [
[00:47:25]     Error {
[00:47:25]         line_num: 15,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "15:1: 15:18: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 18,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "18:1: 18:17: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 21,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "21:17: 21:34: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 24,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "24:5: 24:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 40,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "40:5: 40:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 44,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "44:5: 44:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     }
[00:47:25] ]
[00:47:25] 
[00:47:25] not found errors (from test file): [
[00:47:25]     Error {
[00:47:25]         line_num: 15,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "cannot find derive macro `x3300` in this scope"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 18,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "cannot find derive macro `x3300` in this scope"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 21,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "cannot find derive macro `x3300` in this scope"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 24,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "cannot find derive macro `x3300` in this scope"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 28,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "cannot find derive macro `x3300` in this scope"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 32,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "cannot find derive macro `x3300` in this scope"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 36,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "cannot find derive macro `x3300` in this scope"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 40,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "cannot find derive macro `x3300` in this scope"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 44,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "cannot find derive macro `x3300` in this scope"
[00:47:25]     }
[00:47:25] ]
[00:47:25] 
[00:47:25] thread '[compile-fail] compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1133:12
[00:47:25] 
[00:47:25] ---- [compile-fail] compile-fail/feature-gate/issue-43106-gating-of-derive.rs stdout ----
[00:47:25] 	
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:15: unexpected "error": '15:1: 15:18: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:18: unexpected "error": '18:1: 18:17: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:21: unexpected "error": '21:17: 21:34: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:24: unexpected "error": '24:5: 24:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:37: unexpected "error": '37:5: 37:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:41: unexpected "error": '41:5: 41:21: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:15: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:18: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:21: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:24: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:37: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs:41: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: 6 unexpected errors found, 6 expected errors not found
[00:47:25] status: exit code: 101
[00:47:25] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-derive.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate/issue-43106-gating-of-derive.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -A unused -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate/issue-43106-gating-of-derive.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:47:25] unexpected errors (from JSON output): [
[00:47:25]     Error {
[00:47:25]         line_num: 15,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "15:1: 15:18: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 18,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "18:1: 18:17: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 21,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "21:17: 21:34: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 24,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "24:5: 24:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 37,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "37:5: 37:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 41,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "41:5: 41:21: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     }
[00:47:25] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:323:21
[00:47:25] ]
[00:47:25] 
[00:47:25] not found errors (from test file): [
[00:47:25]     Error {
[00:47:25]         line_num: 15,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 18,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 21,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 24,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 37,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     },
[00:47:25]     Error {
[00:47:25]         line_num: 41,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     }
[00:47:25] ]
[00:47:25] 
[00:47:25] thread '[compile-fail] compile-fail/feature-gate/issue-43106-gating-of-derive.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1133:12
[00:47:25] 
[00:47:25] ---- [compile-fail] compile-fail/issue-36617.rs stdout ----
[00:47:25] 	
[00:47:25] error: /checkout/src/test/compile-fail/issue-36617.rs:11: unexpected "error": '11:1: 11:17: `derive` may only be applied to structs, enums, and unions'
[00:47:25] 
[00:47:25] error: /checkout/src/test/compile-fail/issue-36617.rs:11: expected error not found: `derive` may only be applied to structs, enums and unions
[00:47:25] 
[00:47:25] error: 1 unexpected errors found, 1 expected errors not found
[00:47:25] status: exit code: 101
[00:47:25] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/issue-36617.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-36617.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -A unused -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-36617.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:47:25] unexpected errors (from JSON output): [
[00:47:25]     Error {
[00:47:25]         line_num: 11,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "11:1: 11:17: `derive` may only be applied to structs, enums, and unions"
[00:47:25]     }
[00:47:25] ]
[00:47:25] 
[00:47:25] not found errors (from test file): [
[00:47:25]     Error {
[00:47:25]         line_num: 11,
[00:47:25]         kind: Some(
[00:47:25]             Error
[00:47:25]         ),
[00:47:25]         msg: "`derive` may only be applied to structs, enums and unions"
[00:47:25]     }
[00:47:25] ]
[00:47:25] 
[00:47:25] thread '[compile-fail] compile-fail/issue-36617.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1133:12
[00:47:25] 
[00:47:25] 
[00:47:25] failures:
[00:47:25]     [compile-fail] compile-fail/deriving-non-type.rs
[00:47:25]     [compile-fail] compile-fail/feature-gate/issue-43106-gating-of-derive-2.rs
[00:47:25]     [compile-fail] compile-fail/feature-gate/issue-43106-gating-of-derive.rs
[00:47:25]     [compile-fail] compile-fail/issue-36617.rs
[00:47:25] 
[00:47:25] test result: [31mFAILED(B[m. 2715 passed; 4 failed; 13 ignored; 0 measured; 0 filtered out
