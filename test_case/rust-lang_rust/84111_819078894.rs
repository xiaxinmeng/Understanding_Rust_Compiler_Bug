plain
.................................................................................................... 9400/11755
.................................................................................................... 9500/11755
.........................................................................................i......i... 9600/11755
.................................................................................................... 9700/11755
...................................iiiiiii..iiiiii.i................................................ 9800/11755
.................................................................................................... 10000/11755
.................................................................................................... 10100/11755
.................................................................................................... 10200/11755
.................................................................................................... 10300/11755
---
 finished in 0.480 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.12s

 finished in 0.188 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.57s

 finished in 2.653 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 500/577
.............................................................................
failures:

---- src/collections/binary_heap.rs - collections::binary_heap::BinaryHeap<T>::from (line 1448) stdout ----
error[E0596]: cannot borrow `h1` as mutable, as it is not declared as mutable
  |
  |
5 | let h1 = BinaryHeap::from([1, 2, 3, 4]);
  |     -- help: consider changing this to be mutable: `mut h1`
6 | let h2: BinaryHeap<_> = [1, 2, 3, 4].into();
7 | while let Some((a, b)) = h1.pop().zip(h2.pop()) {
  |                          ^^ cannot borrow as mutable

error[E0596]: cannot borrow `h2` as mutable, as it is not declared as mutable
  |
  |
6 | let h2: BinaryHeap<_> = [1, 2, 3, 4].into();
  |     -- help: consider changing this to be mutable: `mut h2`
7 | while let Some((a, b)) = h1.pop().zip(h2.pop()) {
  |                                       ^^ cannot borrow as mutable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:03
