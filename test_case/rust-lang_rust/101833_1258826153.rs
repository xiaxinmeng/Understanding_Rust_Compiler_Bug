plain
[TIMING] builder::Builder::sysroot_libdir::Libdir { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
[TIMING] compile::Assemble { target_compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu } } -- 0.004
[TIMING] compile::StartupObjects { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (exit status: 127)
--- stderr
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc: error while loading shared libraries: libLLVM-15-rust-1.66.0-nightly.so: cannot open shared object file: No such file or directory
Build completed unsuccessfully in 0:03:28
