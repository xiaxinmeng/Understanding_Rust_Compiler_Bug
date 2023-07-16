plain
.................................................................................................... 9200/11471
.................................................................................................... 9300/11471
.................................................................................................... 9400/11471
.............................i......i............................................................... 9500/11471
....................................................................iiiiiii..iiiiii.i............... 9600/11471
.................................................................................................... 9800/11471
.................................................................................................... 9900/11471
.................................................................................................... 10000/11471
.................................................................................................... 10100/11471
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.076 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.42s

 finished in 2.504 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- src/collections/binary_heap.rs - collections::binary_heap::BinaryHeap<T>::as_slice (line 899) stdout ----
error[E0658]: use of unstable library feature 'binary_heap_as_slice'
 --> src/collections/binary_heap.rs:902:18
  |
6 | let slice = heap.as_slice();
  |
  = note: see issue #82331 <https://github.com/rust-lang/rust/issues/82331> for more information
  = help: add `#![feature(binary_heap_as_slice)]` to the crate attributes to enable

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:08
