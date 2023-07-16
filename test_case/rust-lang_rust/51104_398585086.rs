plain
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:20] 
[00:54:20] running 2426 tests
[00:54:23] ....................................................................................................
[00:54:28] ................................F...................................................................
[00:54:38] ....................................................................................................
[00:54:38] ....................................................................................................
[00:54:44] .................................................................................F....i.............
[00:54:50] ...............................................ii.iii..F.................F.FF.......................
[00:54:54] ..............................................................................i....F................
[00:54:59] .......................i............................................................................
[00:55:04] ....................................F.........................................................i.....
[00:55:08] .........................................F.............F...................................F........
[00:55:14] F...........................................F.................F.....................................
[00:55:19] ...............................................................................F....................
[00:55:25] .......................................................................F...........F................
[00:55:31] ........................................F......F................................................FFF.
[00:55:39] ....................................................................................................
[00:55:44] ...i....................................F...........................................................
[00:55:50] ..i..ii...............................................................................F...........F.
[00:55:58] ......F...................................................................F....................F....
[00:56:07] ....................................................................................i...............
[00:56:12] ..............................i.....................................................................
[00:56:12] ..............................i.....................................................................
[00:56:18] .............................F.....................F...................F...F........F...............
[00:56:22] F...................................................................................................
[00:56:28] ..........................
[00:56:28] failures:
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/bad-sized.rs stdout ----
[00:56:28] ---- [compile-fail] compile-fail/bad-sized.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/bad-sized.rs:14: unexpected error: '14:12: 14:30: the trait bound `dyn Trait: std::marker::Sized` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/bad-sized.rs:14: unexpected error: '14:33: 14:41: the trait bound `dyn Trait: std::marker::Sized` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/bad-sized.rs:14: expected error not found: the trait bound `Trait: std::marker::Sized` is not satisfied
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/bad-sized.rs:14: expected error not found: the trait bound `Trait: std::marker::Sized` is not satisfied
[00:56:28] 
[00:56:28] error: 2 unexpected errors found, 2 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/bad-sized.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/bad-sized/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/bad-sized/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 14,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "14:12: 14:30: the trait bound `dyn Trait: std::marker::Sized` is not satisfied [E0277]"
[00:56:28]     Error {
[00:56:28]         line_num: 14,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "14:33: 14:41: the trait bound `dyn Trait: std::marker::Sized` is not satisfied [E0277]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 14,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "the trait bound `Trait: std::marker::Sized` is not satisfied"
[00:56:28]     Error {
[00:56:28]         line_num: 14,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "the trait bound `Trait: std::marker::Sized` is not satisfied"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/bad-sized.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/cross-borrow-trait.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/cross-borrow-trait.rs:20: expected message not found: expected type `&Trait`
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/cross-borrow-trait.rs:20: expected message not found: found type `std::boxed::Box<Trait>`
[00:56:28] 
[00:56:28] error: 0 unexpected errors found, 2 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/cross-borrow-trait.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cross-borrow-trait/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cross-borrow-trait/auxiliary" "-A" "unused"
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]         line_num: 20,
[00:56:28]         kind: None,
[00:56:28]         msg: "expected type `&Trait`"
[00:56:28]     Error {
[00:56:28]         line_num: 20,
[00:56:28]         kind: None,
[00:56:28]         kind: None,
[00:56:28]         msg: "found type `std::boxed::Box<Trait>`"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/cross-borrow-trait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/destructure-trait-ref.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/destructure-trait-ref.rs:36: unexpected error: '36:9: 36:11: type `&dyn T` cannot be dereferenced [E0033]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/destructure-trait-ref.rs:37: unexpected error: '37:10: 37:12: type `&dyn T` cannot be dereferenced [E0033]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/destructure-trait-ref.rs:38: unexpected error: '38:9: 38:14: type `std::boxed::Box<dyn T>` cannot be dereferenced [E0033]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/destructure-trait-ref.rs:36: expected error not found: type `&T` cannot be dereferenced
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/destructure-trait-ref.rs:37: expected error not found: type `&T` cannot be dereferenced
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/destructure-trait-ref.rs:38: expected error not found: type `std::boxed::Box<T>` cannot be dereferenced
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/destructure-trait-ref.rs:41: expected message not found: expected type `T`
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/destructure-trait-ref.rs:46: expected message not found: expected type `T`
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/destructure-trait-ref.rs:51: expected message not found: expected type `T`
[00:56:28] 
[00:56:28] error: 3 unexpected errors found, 6 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/destructure-trait-ref.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/destructure-trait-ref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/destructure-trait-ref/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 36,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "36:9: 36:11: type `&dyn T` cannot be dereferenced [E0033]"
[00:56:28]     Error {
[00:56:28]         line_num: 37,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "37:10: 37:12: type `&dyn T` cannot be dereferenced [E0033]"
[00:56:28]     Error {
[00:56:28]         line_num: 38,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "38:9: 38:14: type `std::boxed::Box<dyn T>` cannot be dereferenced [E0033]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 36,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "type `&T` cannot be dereferenced"
[00:56:28]     Error {
[00:56:28]         line_num: 37,
[00:56:28]         kind: Some(
[00:56:28]         kind: Some(
[00:56:28] thread 'main' panicked at 'Some tests fa/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/dst-bad-assign-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/dst-bad-assign-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/dst-bad-assign-2/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 45,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "45:5: 45:11: the trait bound `dyn ToBar: std::marker::Sized` is not satisfied [E0277]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 45,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "`ToBar: std::marker::Sized` is not satisfied"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/dst-bad-assign-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/dst-bad-assign-3.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/dst-bad-assign-3.rs:43: unexpected error: '43:5: 43:9: the trait bound `dyn ToBar: std::marker::Sized` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compitd::marker::Sized` is not satisfied"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/dst-bad-assign-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/dst-bad-assign.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/dst-bad-assign.rs:45: unexpected error: '45:5: 45:11: the trait bound `dyn ToBar: std::marker::Sized` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/dst-bad-assign.rs:45: expected message not found: expected type `ToBar`
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/dst-bad-assign.rs:45: expected error not found: `ToBar: std::marker::Sized` is not satisfied
[00:56:28] 
[00:56:28] error: 1 unexpected errors found, 2 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/dst-bad-assign.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/dst-bad-assign/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/dst-bad-assign/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 45,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "45:5: 45:11: the trait bound `dyn ToBar: std::marker::Sized` is not satisfied [E0277]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 45,
[00:56:28]         kind: None,
[00:56:28]         msg: "expected type `ToBar`"
[00:56:28]     Error {
[00:56:28]         line_num: 45,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "`ToBar: std::marker::Sized` is not satisfied"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/dst-bad-assign.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/fn-trait-formatting.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/fn-trait-formatting.rs:16: expected message not found: found type `std::boxed::Box<std::ops::FnOnce(isize)>`
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/fn-trait-formatting.rs:20: expected message not found: found type `std::boxed::Box<std::ops::Fn(isize, isize)>`
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/fn-trait-formatting.rs:24: expected message not found: found type `std::boxed::Box<std::ops::FnMut() -> isize>`
[00:56:28] 
[00:56:28] error: 0 unexpected errors found, 3 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/fn-trait-formatting.rs" "-pected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-13033.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-13033/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-13033/auxiliary" "-A" "unused"
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]         line_num: 18,
[00:56:28]         kind: None,
[00:56:28]         msg: "expected type `fn(&mut Baz, &mut Foo)`"
[00:56:28]     Error {
[00:56:28]         line_num: 18,
[00:56:28]         kind: None,
[00:56:28]         kind: None,
[00:56:28]         msg: "found type `fn(&mut Baz, &Foo)`"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/issue-13033.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/issue-18107.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-18107.rs:14: unexpected error: '14:5: 14:21: the trait bound `(dyn AbstractRenderer + 'static): std::marker::Sized` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-18107.rs:14: expected error not found: `AbstractRenderer + 'static: std::marker::Sized` is not satisfied
[00:56:28] 
[00:56:28] error: 1 unexpected errors found, 1 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-18107.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-18107/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-18107/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 14,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "14:5: 14:21: the trait bound `(dyn AbstractRenderer + \'static): std::marker::Sized` is not satisfied [E0277]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 14,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "`AbstractRenderer + \'static: std::marker::Sized` is not satisfied"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/issue-18107.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/issue-18919.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-18919.rs:13: unexpected error: '13:1: 15:2: the trait bound `dyn for<'r> std::ops::Fn(&'r isize) -> isize: std::marker::Sized` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-18919.rs:13: expected error not found: `for<'r> std::ops::Fn(&'r isize) -> isize: std::marker::Sized` is not satisfied
[00:56:28] 
[00:56:28] error: 1 unexpected errors found, 1 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-18919.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-18919/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-18919/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 13,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "13:1: 15:2: the trait bound `dyn for<\'r> std::ops::Fn(&\'r isize) -> isize: std::marker::Sized` is not satisfied [E0277]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 13,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "`for<\'r> std::ops::Fn(&\'r isize) -> isize: std::marker::Sized` is not satisfied"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/issue-18919.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/issue-20605.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-20605.rs:12: unexpected error: '12:17: 12:24: the trait bound `dyn std::iter::Iterator<Item=&mut u8>: std::marker::Sized` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-20605.rs:12: expected error not found: the trait bound `std::iter::Iterator<Item=&mut u8>: std::marker::Sized` is not satisfied
[00:56:28] 
[00:56:28] error: 1 unexpected errors found, 1 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-20605.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20605/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20605/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 12,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "12:17: 12:24: the trait bound `dyn std::iter::Iterator<Item=&mut u8>: std::marker::Sized` is not satisfied [E0277]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 12,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "the trait bound `std::iter::Iterator<Item=&mut u8>: std::marker::Sized` is not satisfied"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/issue-20605.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/issue-20939.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-20939.rs:13: unexpected error: '13:1: 13:24: the object type `(dyn Foo + 'a)` automatically implements the trait `Foo` [E0371]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-20939.rs:13: expected error not found: the object type `Foo + 'a` automatically implements the trait `Foo`
[00:56:28] 
[00:56:28] error: 1 unexpected errors found, 1 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-20939.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20939/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20939/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 13,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "13:1: 13:24: the object type `(dyn Foo + \'a)` automatically implements the trait `Foo` [E0371]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 13,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "the object type `Foo + \'a` automatically implements the trait `Foo`"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/issue-20939.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/issue-23281.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-23281.rs:16: unexpected error: '16:5: 16:46: the trait bound `(dyn std::ops::Fn() + 'static): std::marker::Sized` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-23281.rs:16: expected error not found: the trait bound `std::ops::Fn() + 'static: std::marker::Sized` is not satisfied
[00:56:28] 
[00:56:28] error: 1 unexpected errors found, 1 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-23281.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-23281/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-23281/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 16,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "16:5: 16:46: the trait bound `(dyn std::ops::Fn() + \'static): std::marker::Sized` is not satisfied [E0277]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 16,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "the trait bound `std::ops::Fn() + \'static: std::marker::Sized` is not satisfied"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/issue-23281.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/issue-24446.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-24446.rs:12: unexpected error: '12:31: 16:6: the trait bound `(dyn std::ops::Fn() -> u32 + 'static): std::marker::Sized` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-24446.rs:12: expected error not found: `std::ops::Fn() -> u32 + 's 
[00:56:28] error: /checkout/src/test/compile-fail/issue-32963.rs:18: unexpected error: '18:5: 18:30: the trait bound `dyn Misc: std::marker::Copy` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-32963.rs:18: expected error not found: the trait bound `Misc: std::marker::Copy` is not satisfied
[00:56:28] 
[00:56:28] error: 1 unexpected errors found, 1 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-32963.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-32963/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-32963/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 18,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "18:5: 18:30: the trait bound `dyn Misc: std::marker::Copy` is not satisfied [E0277]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 18,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "the trait bound `Misc: std::marker::Copy` is not satisfied"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/issue-32963.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/issue-41139.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-41139.rs:16: unexpected error: '16:23: 16:39: cannot move a value of type (dyn Trait + 'static): the size of (dyn Trait + 'static) cannot be statically determined [E0161]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-41139.rs:16: expected error not found: cannot move a value of type Trait + 'static
[00:56:28] 
[00:56:28] error: 1 unexpected errors found, 1 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-41139.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-41139/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-41139/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 16,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "16:23: 16:39: cannot move a value of type (dyn Trait + \'static): the size of (dyn Trait + \'static) cannot be statically determined [E0161]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
[00:56:28]     Error {
[00:56:28]     Error {
[00:56:28]         line_num: 16,
[00:56:28]         kind: Some(
[00:56:28]             Error
[00:56:28]         ),
[00:56:28]         msg: "cannot move a value of type Trait + \'static"
[00:56:28] ]
[00:56:28] 
[00:56:28] thread '[compile-fail] compile-fail/issue-41139.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:28] 
[00:56:28] 
[00:56:28] ---- [compile-fail] compile-fail/issue-42312.rs stdout ----
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-42312.rs:18: unexpected error: '18:23: 18:23: the trait bound `(dyn std::string::ToString + 'static): std::marker::Sized` is not satisfied [E0277]'
[00:56:28] 
[00:56:28] error: /checkout/src/test/compile-fail/issue-42312.rs:18: expected error not found: the trait bound `std::string::ToString + 'static: std::marker::Sized` is not satisfied
[00:56:28] 
[00:56:28] error: 1 unexpected errors found, 1 expected errors not found
[00:56:28] status: exit code: 101
[00:56:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-42312.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-42312/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-42312/auxiliary" "-A" "unused"
[00:56:28] unexpected errors (from JSON output): [
[00:56:28]     Error {
[00:56:28]         line_num: 18,
[00:56:28]         kind: Some(
[00:56:28]         ),
[00:56:28]         ),
[00:56:28]         msg: "18:23: 18:23: the trait bound `(dyn std::string::ToString + \'static): std::marker::Sized` is not satisfied [E0277]"
[00:56:28] ]
[00:56:28] 
[00:56:28] not found errors (from test file): [
---
[00:56:28] test result: FAILED. 2377 passed; 34 failed; 15 ignored; 0 measured; 0 filtered out
[00:56:28] 
[00:56:28] 
[00:56:28] 
[00:56:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:28] 
[00:56:28] 
[00:56:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:28] Build completed unsuccessfully in 0:14:22
[00:56:28] Build completed unsuccessfully in 0:14:22
[00:56:28] Makefile:58: recipe for target 'check' failed
[00:56:28] make: *** [check] Error 1
