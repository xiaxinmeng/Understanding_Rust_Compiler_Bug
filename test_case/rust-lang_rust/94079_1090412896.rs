plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling  --edition=2021 --crate-type=rlib ../../../../library/alloc/src/lib.rs --cfg no_global_oom_handling
--- stderr -------------------------------
warning: unused import: `from_fn`
  --> ../../../../library/alloc/src/string.rs:51:18
   |
   |
51 | use core::iter::{from_fn, FusedIterator};
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `core::ops::Deref`
---

error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0635`.
make: *** [Makefile:4: all] Error 1



failures:
