plain
cargo:root=/checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers
 finished in 8.882 seconds
[TIMING] native::Sanitizers { target: x86_64-unknown-linux-gnu } -- 8.882
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (exit status: 127)
--- stderr
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc: error while loading shared libraries: libLLVM-15-rust-1.66.0-nightly.so: cannot open shared object file: No such file or directory
Build completed unsuccessfully in 0:03:47
