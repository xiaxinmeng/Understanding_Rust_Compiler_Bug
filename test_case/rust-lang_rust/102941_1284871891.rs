plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling  --edition=2021 -Dwarnings --crate-type=rlib ../../../../library/alloc/src/lib.rs --cfg no_global_oom_handling
--- stderr -------------------------------
error[E0425]: cannot find function `from_utf8_unchecked_mut` in this scope
    --> ../../../../library/alloc/src/string.rs:1879:18
     |
     |
1879 |         unsafe { from_utf8_unchecked_mut(slice) }
     |
help: consider importing one of these items
     |
47   | use core::str::from_utf8_unchecked_mut;
