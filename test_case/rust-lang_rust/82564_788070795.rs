text
; RUST_SRC="../rust" ./run-test.sh alloc -- vec::test_stable_pointers
A libstd for Miri is now available in `/home/waffle/.cache/miri/HOST`.
   Compiling alloc_miri_test v0.0.0 (/home/waffle/projects/repos/miri-test-libstd/alloc_miri_test)
    Finished test [unoptimized + debuginfo] target(s) in 2.16s
     Running unittests (/home/waffle/projects/repos/miri-test-libstd/target/x86_64-unknown-linux-gnu/debug/deps/alloc_miri_test-6e6a55ec61be2275)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 251 filtered out

     Running ../liballoc/tests/lib.rs (/home/waffle/projects/repos/miri-test-libstd/target/x86_64-unknown-linux-gnu/debug/deps/collectionstests-8d70d32833968b04)

running 1 test
test vec::test_stable_pointers ... error: Undefined Behavior: trying to reborrow for SharedReadOnly at alloc935527, but parent tag <2398216> does not have an appropriate item in the borrow stack
    --> alloc_miri_test/../liballoc/tests/vec.rs:1696:5
     |
1696 |     assert_eq!(*v0, 13);
     |     ^^^^^^^^^^^^^^^^^^^^ trying to reborrow for SharedReadOnly at alloc935527, but parent tag <2398216> does not have an appropriate item in the borrow stack
     |
     = help: this indicates a potential bug in the program: it performed an invalid operation, but the rules it violated are still experimental
     = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information

     = note: inside `vec::test_stable_pointers` at /home/waffle/projects/repos/rust/library/core/src/macros/mod.rs:37:16
note: inside closure at alloc_miri_test/../liballoc/tests/vec.rs:1615:1
    --> alloc_miri_test/../liballoc/tests/vec.rs:1615:1
     |
1615 | / fn test_stable_pointers() {
1616 | |     /// Pull an element from the iterator, then drop it.
1617 | |     /// Useful to cover both the `next` and `drop` paths of an iterator.
1618 | |     fn next_then_drop<I: Iterator>(mut i: I) {
...    |
1700 | |     assert_eq!(v[0], 0);
1701 | | }
     | |_^
     = note: inside `<[closure@alloc_miri_test/../liballoc/tests/vec.rs:1615:1: 1701:2] as std::ops::FnOnce<()>>::call_once - shim` at /home/waffle/projects/repos/rust/library/core/src/ops/function.rs:227:5
     = note: inside `<fn() as std::ops::FnOnce<()>>::call_once - shim(fn())` at /home/waffle/projects/repos/rust/library/core/src/ops/function.rs:227:5
     = note: inside `test::__rust_begin_short_backtrace::<fn()>` at /home/waffle/projects/repos/rust/library/test/src/lib.rs:568:5
     = note: inside closure at /home/waffle/projects/repos/rust/library/test/src/lib.rs:559:30
     = note: inside `<[closure@test::run_test::{closure#2}] as std::ops::FnOnce<()>>::call_once - shim(vtable)` at /home/waffle/projects/repos/rust/library/core/src/ops/function.rs:227:5
     = note: inside `<std::boxed::Box<dyn std::ops::FnOnce() + std::marker::Send> as std::ops::FnOnce<()>>::call_once` at /home/waffle/projects/repos/rust/library/alloc/src/boxed.rs:1546:9
     = note: inside `<std::panic::AssertUnwindSafe<std::boxed::Box<dyn std::ops::FnOnce() + std::marker::Send>> as std::ops::FnOnce<()>>::call_once` at /home/waffle/projects/repos/rust/library/std/src/panic.rs:344:9
     = note: inside `std::panicking::r#try::do_call::<std::panic::AssertUnwindSafe<std::boxed::Box<dyn std::ops::FnOnce() + std::marker::Send>>, ()>` at /home/waffle/projects/repos/rust/library/std/src/panicking.rs:379:40
     = note: inside `std::panicking::r#try::<(), std::panic::AssertUnwindSafe<std::boxed::Box<dyn std::ops::FnOnce() + std::marker::Send>>>` at /home/waffle/projects/repos/rust/library/std/src/panicking.rs:343:19
     = note: inside `std::panic::catch_unwind::<std::panic::AssertUnwindSafe<std::boxed::Box<dyn std::ops::FnOnce() + std::marker::Send>>, ()>` at /home/waffle/projects/repos/rust/library/std/src/panic.rs:431:14
     = note: inside `test::run_test_in_process` at /home/waffle/projects/repos/rust/library/test/src/lib.rs:590:18
     = note: inside closure at /home/waffle/projects/repos/rust/library/test/src/lib.rs:487:39
     = note: inside `test::run_test::run_test_inner` at /home/waffle/projects/repos/rust/library/test/src/lib.rs:523:13
     = note: inside `test::run_test` at /home/waffle/projects/repos/rust/library/test/src/lib.rs:556:28
     = note: inside `test::run_tests::<[closure@test::run_tests_console::{closure#2}]>` at /home/waffle/projects/repos/rust/library/test/src/lib.rs:302:13
     = note: inside `test::run_tests_console` at /home/waffle/projects/repos/rust/library/test/src/console.rs:289:5
     = note: inside `test::test_main` at /home/waffle/projects/repos/rust/library/test/src/lib.rs:123:15
     = note: inside `test::test_main_static` at /home/waffle/projects/repos/rust/library/test/src/lib.rs:142:5
     = note: inside `main`
     = note: inside `<fn() as std::ops::FnOnce<()>>::call_once - shim(fn())` at /home/waffle/projects/repos/rust/library/core/src/ops/function.rs:227:5
     = note: inside `std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>` at /home/waffle/projects/repos/rust/library/std/src/sys_common/backtrace.rs:125:18
     = note: inside closure at /home/waffle/projects/repos/rust/library/std/src/rt.rs:66:18
     = note: inside `std::ops::function::impls::<impl std::ops::FnOnce<()> for &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>::call_once` at /home/waffle/projects/repos/rust/library/core/src/ops/function.rs:259:13
     = note: inside `std::panicking::r#try::do_call::<&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe, i32>` at /home/waffle/projects/repos/rust/library/std/src/panicking.rs:379:40
     = note: inside `std::panicking::r#try::<i32, &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>` at /home/waffle/projects/repos/rust/library/std/src/panicking.rs:343:19
     = note: inside `std::panic::catch_unwind::<&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe, i32>` at /home/waffle/projects/repos/rust/library/std/src/panic.rs:431:14
     = note: inside `std::rt::lang_start_internal` at /home/waffle/projects/repos/rust/library/std/src/rt.rs:51:25
     = note: inside `std::rt::lang_start::<()>` at /home/waffle/projects/repos/rust/library/std/src/rt.rs:65:5
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

error: test failed, to rerun pass '--test collectionstests'
