plain
    Finished release [optimized] target(s) in 2m 39s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (exit status: 101)
--- stderr
thread 'main' panicked at 'cannot transmute_copy if Dst is larger than Src', /checkout/library/core/src/mem/mod.rs:1047:5

Build completed unsuccessfully in 0:03:30
