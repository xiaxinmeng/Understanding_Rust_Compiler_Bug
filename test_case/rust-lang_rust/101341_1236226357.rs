plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling  --edition=2021 -Dwarnings --crate-type=rlib ../../../../library/alloc/src/lib.rs --cfg no_global_oom_handling
--- stderr -------------------------------
--- stderr -------------------------------
error: associated constant `MIN_NON_ZERO_CAP` is never used
   --> ../../../../library/alloc/src/raw_vec.rs:111:22
    |
111 |     pub(crate) const MIN_NON_ZERO_CAP: usize = if mem::size_of::<T>() == 1 {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: aborting due to previous error

make: *** [Makefile:4: all] Error 1
------------------------------------------
