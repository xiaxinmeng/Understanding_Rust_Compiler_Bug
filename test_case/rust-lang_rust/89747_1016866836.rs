plain
......i.....................i.....................i................................................. 3700/3743
...........................................
failures:

---- src/mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::slice_as_bytes (line 1232) stdout ----
error: the item `TryInto` is imported redundantly
    |
6   | use std::convert::TryInto;
    |     ^^^^^^^^^^^^^^^^^^^^^
    |
    |
   ::: /checkout/library/std/src/prelude/mod.rs:133:13
    |
133 |     pub use core::prelude::rust_2021::*;
    |             --------------------------- the item `TryInto` is already defined here
note: the lint level is defined here
   --> src/mem/maybe_uninit.rs:1230:9
    |
1   | #![deny(warnings)]
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:46
