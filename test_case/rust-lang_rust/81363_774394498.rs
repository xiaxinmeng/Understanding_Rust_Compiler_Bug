plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.068 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 9.250 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i.ii....i..i...ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.24s

 finished in 2.309 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
......i.....................i.....................i................................................. 2800/2832
................................
failures:

---- src/pin.rs - pin::Pin<&'a mut Pin<P>>::as_deref_mut (line 807) stdout ----
error[E0433]: failed to resolve: use of undeclared type `F`
   |
   |
11 |     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<F::Output> {
   |                                                                 ^ use of undeclared type `F`

error[E0119]: conflicting implementations of trait `std::future::Future` for type `std::pin::Pin<std::boxed::Box<main::_doctest_main_src_pin_rs_807_0::Type>>`:
   |
   |
10 | impl Future for Pin<Box<Type>> {
   |
   = note: conflicting implementation in crate `core`:
   = note: conflicting implementation in crate `core`:
           - impl<P> Future for Pin<P>
             where P: DerefMut, <P as Deref>::Target: Future;
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0119, E0433.
For more information about an error, try `rustc --explain E0119`.
For more information about an error, try `rustc --explain E0119`.
Couldn't compile the test.

failures:
    src/pin.rs - pin::Pin<&'a mut Pin<P>>::as_deref_mut (line 807)
test result: FAILED. 2804 passed; 1 failed; 27 ignored; 0 measured; 0 filtered out; finished in 44.70s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:43
