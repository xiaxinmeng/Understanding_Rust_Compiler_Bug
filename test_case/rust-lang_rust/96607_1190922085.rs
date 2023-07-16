plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling  --edition=2021 -Dwarnings --crate-type=rlib ../../../../library/alloc/src/lib.rs --cfg no_global_oom_handling
--- stderr -------------------------------
error: unused import: `NonNull`
  --> ../../../../library/alloc/src/vec/into_iter.rs:11:23
   |
   |
11 | use core::ptr::{self, NonNull};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`

error: field `buf` is never read
  --> ../../../../library/alloc/src/vec/into_iter.rs:31:16
27 | pub struct IntoIter<
   |            -------- field in this struct
...
...
31 |     pub(super) buf: *const [T],
   |
   |
   = note: `-D dead-code` implied by `-D warnings`
error: aborting due to 2 previous errors


make: *** [Makefile:4: all] Error 1



failures:
