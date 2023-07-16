plain
---- src/collections/binary_heap.rs - collections::binary_heap::BinaryHeap<T>::try_reserve_exact (line 976) stdout ----
error[E0658]: use of unstable library feature 'try_reserve_binary_heap'
  --> src/collections/binary_heap.rs:984:10
   |
11 |     heap.try_reserve_exact(data.size())?;
   |
   |
   = help: add `#![feature(try_reserve_binary_heap)]` to the crate attributes to enable

error[E0599]: no method named `size` found for reference `&[u32]` in the current scope
   |
   |
11 |     heap.try_reserve_exact(data.size())?;
   |                                 ^^^^ method not found in `&[u32]`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0658.
For more information about an error, try `rustc --explain E0599`.
---
   |
11 |     heap.try_reserve(data.size())?;
   |          ^^^^^^^^^^^
   |
   = help: add `#![feature(try_reserve_binary_heap)]` to the crate attributes to enable

error[E0599]: no method named `size` found for reference `&[u32]` in the current scope
   |
11 |     heap.try_reserve(data.size())?;
   |                           ^^^^ method not found in `&[u32]`

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:18:49
