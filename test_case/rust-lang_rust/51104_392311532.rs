plain
[00:51:25] ..............................................................i.....................................
[00:51:29] ....................................................................................................
[00:51:35] ....................................................................................................
[00:51:42] ...........................................................................................i........
[00:51:44] .........iiiiiiiii...................................................
[00:51:44] 
[00:51:44] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:52:34] ..............................................................i.....................................
[00:52:38] ....................................................................................................
[00:52:44] ....................................................................................................
[00:52:50] ...........................................................................................i........
[00:52:52] .........iiiiiiiii...................................................
[00:52:52] 
[00:52:52]  finished in 68.036
[00:52:52] travis_fold:end:test_ui_nll

---
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:05] 
[01:02:05] running 2420 tests
[01:02:09] ....................................................................................................
[01:02:14] ................................F...................................................................
[01:02:23] ....................................................................................................
[01:02:23] ....................................................................................................
[01:02:30] .................................................................................F....i.............
[01:02:35] ...............................................ii.iiiF...................F.FF.......................
[01:02:40] ..............................................................................i..F..................
[01:02:45] .......................i............................................................................
[01:02:49] ....................................F.........................................................i.....
[01:02:53] ..........................................F............F.................................F..........
[01:02:59] .F..........................................F.................F.....................................
[01:03:05] ...............................................................................F....................
[01:03:11] .......................................................................F............F...............
[01:03:16] ........................................F.......F...............................................FF.F
[01:03:24] ....................................................................................................
[01:03:29] ...i....................................F...........................................................
[01:03:35] ..i..ii...............................................................................F...........F.
[01:03:43] ......F..........................................................F.......................F..........
[01:03:53] ..............................................................................i.....................
[01:03:58] ........................i...........................................................................
[01:03:58] ........................i...........................................................................
[01:04:03] .......................F.....................F....................F..F........F...............F.....
[01:04:13] ....................
[01:04:13] failures:
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/bad-sized.rs stdout ----
[01:04:13] ---- [compile-fail] compile-fail/bad-sized.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/bad-sized.rs:14: unexpected error: '14:12: 14:30: the trait bound `dyn Trait: std::marker::Sized` is not satisfied [E0277]'
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/bad-sized.rs:14: unexpected error: '14:33: 14:41: the trait bound `dyn Trait: std::marker::Sized` is not satisfied [E0277]'
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/bad-sized.rs:14: expected error not found: the trait bound `Trait: std::marker::Sized` is not satisfied
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/bad-sized.rs:14: expected error not found: the trait bound `Trait: std::marker::Sized` is not satisfied
[01:04:13] 
[01:04:13] error: 2 unexpected errors found, 2 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/bad-sized.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/bad-sized/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-u:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]         line_num: 45,
[01:04:13]         kind: None,
[01:04:13]         msg: "expected type `ToBar`"
[01:04:13]     Error {
[01:04:13]         line_num: 45,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "`ToBar: std::marker::Sized` is not satisfied"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/dst-bad-assign.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/fn-trait-formatting.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/fn-trait-formatting.rs:16: expected message not found: found type `std::boxed::Box<std::ops::FnOnce(isize)>`
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/fn-trait-formatting.rs:20: expected message not found: found type `std::boxed::Box<std::ops::Fn(isize, isize)>`
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/fn-trait-formatting.rs:24: expected message not found: found type `std::boxed::Box<std::ops::FnMut() -> isize>`
[01:04:13] 
[01:04:13] error: 0 unexpected errors found, 3 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/fn-trait-formatting.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/fn-trait-formatting/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/fn-trait-formatting/auxiliary" "-A" "unused"
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]         line_num: 16,
[01:04:13]         kind: None,
[01:04:13]         msg: "found type `std::boxed::Box<std::ops::FnOnce(isize)>`"
[01:04:13]     Error {
[01:04:13]         line_num: 20,
[01:04:13]         kind: None,
[01:04:13]         kind: None,
[01:04:13]         msg: "found type `std::boxed::Box<std::ops::Fn(isize, isize)>`"
[01:04:13]     Error {
[01:04:13]         line_num: 24,
[01:04:13]         kind: None,
[01:04:13]         kind: None,
[01:04:13]         msg: "found type `std::boxed::Box<std::ops::FnMut() -> isize>`"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/fn-trait-formatting.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/issue-13033.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-13033.rs:18: expected message not found: expected type `fn(&mut Baz, &mut Foo)`
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-13033.rs:18: expected message not found: found type `fn(&mut Baz, &Foo)`
[01:04:13] 
[01:04:13] error: 0 unexpected errors found, 2 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compustc" "/checkout/src/test/compile-fail/issue-18107.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-18107/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-18107/auxiliary" "-A" "unused"
[01:04:13] unexpected errors (from JSON output): [
[01:04:13]     Error {
[01:04:13]         line_num: 14,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "14:5: 14:21: the trait bound `dyn AbstractRenderer + \'static: std::marker::Sized` is not satisfied [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]         line_num: 14,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "`AbstractRenderer + \'static: std::marker::Sized` is not satisfied"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/issue-18107.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/issue-18919.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-18919.rs:13: unexpected error: '13:1: 15:2: the trait bound `dyn for<'r> std::ops::Fn(&'r isize) -> isize: std::marker::Sized` is not satisfied [E0277]'
[01:04:13] error: /checkout/srcc/runtest.rs:1283:13
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/issue-20605.rs stdout ----
[01:04:13] 
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-20605.rs:12: unexpected error: '12:17: 12:24: the trait bound `dyn std::iter::Iterator<Item=&mut u8>: std::marker::Sized` is not satisfied [E0277]'
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-20605.rs:12: expected error not found: the trait bound `std::iter::Iterator<Item=&mut u8>: std::marker::Sized` is not satisfied
[01:04:13] 
[01:04:13] error: 1 unexpected errors found, 1 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-20605.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20605/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20605/auxiliary" "-A" "unused"
[01:04:13] unexpected errors (from JSON output): [
[01:04:13]     Error {
[01:04:13]         line_num: 12,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "12:17: 12:24: the trait bound `dyn std::iter::Iterator<Item=&mut u8>: std::marker::Sized` is not satisfied [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]     Some(
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "13:1: 13:24: the object type `dyn Foo + \'a` automatically implements the trait `Foo` [E0371]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]         line_num: 13,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "the object type `Foo + \'a` automatically implements the trait `Foo`"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/issue-20939.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/issue-23281.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-23281.rs:16: unexpected error: '16:5: 16:46: the trait bound `dyn std::ops::Fn() + 'static: std::marker::Sized` is not satisfied [E0277]'
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-23281.rs:16: expected error not found: the trait bound `std::ops::Fn() + 'static: std::marker::Sized` is not satisfied
[01:04:13] 
[01:04:13] error: 1 unexpected errors found, 1 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-23281.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-23281/a" "-Crpath" "-O" "-Zunstable-options" "-Lna13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-32963.rs:18: expected error not found: the trait bound `Misc: std::marker::Copy` is not satisfied
[01:04:13] 
[01:04:13] error: 1 unexpected errors found, 1 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-32963.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-32963/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-32963/auxiliary" "-A" "unused"
[01:04:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:04:13] unexpected errors (from JSON output): [
[01:04:13]     Error {
[01:04:13]         line_num: 18,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "18:5: 18:30: the trait bound `dyn Misc: std::marker::Copy` is not satisfied [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]         line_num: 18,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "the trait bound `Misc: std::marker::Copy` is not satisfied"
[01:04:13] ]
[01:04:13] 
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/issue-32963.rs'ile): [
[01:04:13]     Error {
[01:04:13]         line_num: 16,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "cannot move a value of type Trait + \'static"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/issue-41139.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/issue-42312.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-42312.rs:18: unexpected error: '18:23: 18:23: the trait bound `dyn std::string::ToString + 'static: std::marker::Sized` is not satisfied [E0277]'
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-42312.rs:18: expected error not found: the trait bound `std::string::ToString + 'static: std::marker::Sized` is not satisfied
[01:04:13] 
[01:04:13] error: 1 unexpected errors found, 1 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-42312.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-42312/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-42312/auxiliary" "-A" "unused"
[01:04:13] unexpected errors (from JSON output): [
[01:04:13]     Error {
[01:04:13]         line_num: 18,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "18:23: 18:23: the trait bound `dyn std::string::ToString + \'static: std::marker::Sized` is not satisfied [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]         line_num: 18,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "the trait bound `std::string::ToString + \'static: std::marker::Sized` is not satisfied"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/issue-42312.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/issue-5153.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-5153.rs:20: unexpected error: '20:23: 20:26: no method named `foo` found for type `&dyn Foo` in the current scope [E0599]'
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-5153.rs:20: expected error not found: no method named `foo` found for type `&Foo` in the current scope
[01:04:13] 
[01:04:13] error: 1 unexpected errors found, 1 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-5153.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-5ic: std::marker::Sized` is not satisfied
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/issue-5883.rs:18: expected error not found: `A + 'static: std::marker::Sized` is not satisfied
[01:04:13] 
[01:04:13] error: 2 unexpected errors found, 2 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-5883.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-5883/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-5883/auxiliary" "-A" "unused"
[01:04:13] unexpected errors (from JSON output): [
[01:04:13]     Error {
[01:04:13]         line_num: 17,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "17:15: 17:16: the trait bound `dyn A + \'static: std::marker::Sized` is not satisfied [E0277]"
[01:04:13]     Error {
[01:04:13]         line_num: 18,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "18:8: 18:14: the trait bound `dyn A + \'static: std::marker::Sized` is not satisfied in `Struct` [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]         line_num: 17,
[01:04:13]         kind: Some(
["-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/kindck-send-object1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/kindck-send-object1/auxiliary" "-A" "unused"
[01:04:13] unexpected errors (from JSON output): [
[01:04:13]     Error {
[01:04:13]         line_num: 20,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "20:5: 20:29: `dyn Dummy + \'a` cannot be shared between threads safely [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]         line_num: 20,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "`Dummy + \'a` cannot be shared between threads safely [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/kindck-send-object1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/kindck-send-object2.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/kindck-send-object2.rs:17: unexpected error: '17:5: 17:34: `dyn Dummy + 'static` cannot be shared between threads safely [E0277]'
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/kindck-send-object2.rs:17: expected error not found: `Dummy + 'static` cannot be shared between threads safely [E0277]
[01:04:13] 
[01:04:13] error: 1 unexpected errors found, 1 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/kindck-send-object2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/kindck-send-object2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/kindck-send-object2/auxiliary" "-A" "unused"
[01:04:13] unexpected errors (from JSON output): [
[01:04:13]     Error {
[01:04:13]         line_num: 17,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "17:5: 17:34: `dyn Dummy + \'static` cannot be shared between threads safely [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]         line_num: 17,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "`Dummy + \'static` cannot be shared between threads safely [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/kindck-send-object2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/map-types.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/map-tterger-atomic.rs:73: expected error not found: expected basic integer type, found `&std::ops::Fn()`
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/non-interger-atomic.rs:78: expected error not found: expected basic integer type, found `&std::ops::Fn()`
[01:04:13] 
[01:04:13] error: 4 unexpected errors found, 4 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/non-interger-atomic.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/non-interger-atomic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/non-interger-atomic/auxiliary" "-A" "unused"
[01:04:13] unexpected errors (from JSON output): [
[01:04:13]     Error {
[01:04:13]         line_num: 63,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "63:5: 63:31: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()` [E0511]"
[01:04:13]     Error {
[01:04:13]         line_num: 68,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "68:5: 68:35: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()` [E0511]"
[01:04:13]     Error {
[01:04:13]         line_num: 73,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "73:5: 73:34: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()` [E0511]"
[01:04:13]     Error {
[01:04:13]         line_num: 78,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "78:5: 78:38: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()` [E0511]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]         line_num: 63,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "expected basic integer type, found `&std::ops::Fn()`"
[01:04:13]     Error {
[01:04:13]         line_num: 68,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "expected basic integer type, found `&std::ops::Fn()`"
[01:04:13]     Error {
[01:04:13]         line_num: 73,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "expected basic integer type, found `&std::ops::Fn()`"
[01:04:13]     Error {
[01:04:13]         line_num: 78,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "expected basic integer type, found `&std::ops::Fn()`"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/non-interger-atomic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/object-does-not-impl-trait.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/object-does-not-impl-trait.rs:16: unexpected error: '16:31: 16:39: the trait bound `std::boxed::Box<dyn Foo>: Foo` is not satisfied [E0277]'
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/object-does-not-impl-trait.rs:16: expected error not found: `std::boxed::Box<Foo>: Foo` is not satisfied
[01:04:13] 
[01:04:13] error: 1 unexpected errors found, 1 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/object-does-not-impl-trait.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/object-does-not-impl-trait/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/object-does-not-impl-trait/auxiliary" "-A" "unused"
[01:04:13] unexpected errors (from JSON output): [
[01:04:13]     Error {
[01:04:13]         line_num: 16,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "16:31: 16:39: the trait bound `std::boxed::Box<dyn Foo>: Foo` is not satisfied [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]         line_num: 16,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "`std::boxed::Box<Foo>: Foo` is not satisfied"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/object-does-not-impl-trait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/object-safety-by-value-self-use.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/object-safety-by-value-self-use.rs:25: unexpected error: '25:5: 25:6: cannot move a value of type dyn Bar + 'static: the size of dyn Bar + 'static cannot be statically determined [E0161]'
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/object-safety-by-value-self-use.rs:25: expected error not found: cannot move a value of type Bar
[01:04:13] 
[01:04:13] error: 1 unexpected errors found, 1 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/object-safety-by-value-self-use.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/object-safety-by-value-self-use/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/c36:7: 36:14: the trait bound `dyn CompareToInts: CompareTo<i32>` is not satisfied [E0277]"
[01:04:13]     Error {
[01:04:13]         line_num: 44,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "44:5: 44:27: the trait bound `dyn CompareToInts: CompareTo<i32>` is not satisfied [E0277]"
[01:04:13] ]
[01:04:13] 
[01:04:13] not found errors (from test file): [
[01:04:13]     Error {
[01:04:13]     Error {
[01:04:13]         line_num: 36,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "`CompareToInts: CompareTo<i32>` is not satisfied"
[01:04:13]     Error {
[01:04:13]         line_num: 44,
[01:04:13]         kind: Some(
[01:04:13]             Error
[01:04:13]             Error
[01:04:13]         ),
[01:04:13]         msg: "`CompareToInts: CompareTo<i32>` is not satisfied"
[01:04:13] ]
[01:04:13] 
[01:04:13] thread '[compile-fail] compile-fail/traits-repeated-supertrait-ambig.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:04:13] 
[01:04:13] 
[01:04:13] ---- [compile-fail] compile-fail/trivial_casts.rs stdout ----
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/trivial_casts.rs:62: unexpected error: '62:13: 62:22: trivial cast: `&Bar` as `&dyn Foo`. Cast can be replaced by coercion, this might require type ascription or a temporary variable [trivial_casts]'
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/trivial_casts.rs:63: unexpected error: '63:13: 63:28: trivial cast: `&Bar` as `*const dyn Foo`. Cast can be replaced`&mut Foo`
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/trivial_casts.rs:69: expected error not found: trivial cast: `&mut Bar` as `*mut Foo`
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/trivial_casts.rs:74: expected error not found: trivial cast: `std::boxed::Box<Bar>` as `std::boxed::Box<Foo>`
[01:04:13] 
[01:04:13] error: /checkout/src/test/compile-fail/trivial_casts.rs:80: expected error not found: trivial cast: `&fn(i32) {main::baz}` as `&std::ops::Fn(i32)`
[01:04:13] 
[01:04:13] error: 6 unexpected errors found, 6 expected errors not found
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/trivial_casts.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/trivial_casts/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/trivial_casts/auxiliary" "-A" "unused"
[01:04:13] unexpected errors (from JSON output): [
[01:04:13]     Error {
[01:04:13]         line_num: 62,
[01:04:13]         kind: Some(
[01:04:13]         ),
[01:04:13]         ),
[01:04:13]         msg: "62:13: 62:22: trivial cast: `&Bar` as `&dyn Foo`. Cast can be replaced by coercion, this might require type ascription or a temporary variable [trivial_casts]"
[01:04:13]     Error {
[01:04:13]         line_num: 63,
---
[01:04:13] test result: FAILED. 2371 passed; 34 failed; 15 ignored; 0 measured; 0 filtered out
[01:04:13] 
[01:04:13] 
[01:04:13] 
[01:04:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" " directory
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23426676
$ dmesg | grep -i kill
