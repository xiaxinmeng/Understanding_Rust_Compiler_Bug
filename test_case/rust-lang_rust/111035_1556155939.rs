plain
tests/fail/tokio/sleep.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/layout_cycle.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "tests/fail/layout_cycle.rs" "--edition" "2021"


actual output differed from expected
--- tests/fail/layout_cycle.stderr
--- tests/fail/layout_cycle.stderr
+++ <stderr output>
... 3 lines skipped ...
    = note: ...which again requires computing layout of `S<S<()>>`, completing the cycle
 
~error: post-monomorphization error: a cycle occurred during layout computation
+Miri caused an ICE during evaluation. Here's the interpreter backtrace at the time of the panic:
+note: the place in the program where the ICE was triggered
    |
 LL |     intrinsics::size_of::<T>()
 LL |     intrinsics::size_of::<T>()
~   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ a cycle occurred during layout computation
    |
    = note: inside `std::mem::size_of::<S<S<()>>>` at RUSTLIB/core/src/mem/mod.rs:LL:CC
... 8 lines skipped ...
 LL |     println!("{}", foo::<S<()>>());
+   = note: inside `<fn() as std::ops::FnOnce<()>>::call_once - shim(fn())` at RUSTLIB/core/src/ops/function.rs:LL:CC
+   = note: inside `std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>` at RUSTLIB/std/src/sys_common/backtrace.rs:LL:CC
+   = note: inside closure at RUSTLIB/std/src/rt.rs:LL:CC
+   = note: inside `std::ops::function::impls::<impl std::ops::FnOnce<()> for &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>::call_once` at RUSTLIB/core/src/ops/function.rs:LL:CC
---
 For more information about this error, try `rustc --explain E0391`.
 


substring `a cycle occurred during layout computation` not found in stderr output

full stderr:
full stderr:
error[E0391]: cycle detected when computing layout of `S<S<()>>`
   |
   = note: ...which requires computing layout of `<S<()> as Tr>::I`...
   = note: ...which again requires computing layout of `S<S<()>>`, completing the cycle


Miri caused an ICE during evaluation. Here's the interpreter backtrace at the time of the panic:
note: the place in the program where the ICE was triggered
   |
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: inside `std::mem::size_of::<S<S<()>>>` at /checkout/library/core/src/mem/mod.rs:313:5: 313:31
note: inside `foo::<S<()>>`
   |
   |
LL |     mem::size_of::<S<T>>()
note: inside `main`
  --> tests/fail/layout_cycle.rs:27:20
   |
   |
LL |     println!("{}", foo::<S<()>>());
   = note: inside `<fn() as std::ops::FnOnce<()>>::call_once - shim(fn())` at /checkout/library/core/src/ops/function.rs:250:5: 250:71
   = note: inside `std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>` at /checkout/library/std/src/sys_common/backtrace.rs:135:18: 135:21
   = note: inside closure at /checkout/library/std/src/rt.rs:166:18: 166:82
   = note: inside `std::ops::function::impls::<impl std::ops::FnOnce<()> for &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>::call_once` at /checkout/library/core/src/ops/function.rs:284:13: 284:31
---
test result: FAIL. 1 tests failed, 381 tests passed, 3 ignored, 0 filtered out
Error: 
   0: tests failed

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-2dd052f68a77f31f --quiet` (exit status: 1)
Build completed unsuccessfully in 0:03:31
