plain
...............................F..iiii..ii.iii...................

failures:

---- [assembly] tests/assembly/stack-protector/stack-protector-target-support.rs#r73 stdout ----

error in revision `r73`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/assembly/stack-protector/stack-protector-target-support.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "r73" "-O" "--emit" "asm" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-target-support.r73/stack-protector-target-support.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-target-support.r73/auxiliary" "--target" "wasm32-wasi" "-Z" "stack-protector=all" "-C" "opt-level=2"
stdout: none
--- stderr -------------------------------
error: Error loading target specification: Could not find specification for target "wasm32-wasi". Run `rustc --print target-list` for a list of built-in targets



failures:
