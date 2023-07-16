plain
    Finished release [optimized] target(s) in 2m 55s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (exit status: 254)
--- stderr
thread '<unnamed>' panicked at '`new_layout.size()` must be greater than or equal to `old_layout.size()`', /checkout/library/alloc/src/alloc.rs:185:9
thread panicked while processing panic. aborting.
thread panicked while processing panic. aborting.
rustc exited with signal: 6 (core dumped)
Build completed unsuccessfully in 0:03:56
