plain
DirectMap1G:    54525952 kB
+ python3 ../x.py dist --host= --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-pc-solaris,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi,i686-unknown-freebsd,x86_64-unknown-none
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.04s
thread 'main' panicked at 'need a managed LLVM submodule for optimized intrinsics support; unset `llvm-config` or `optimized-compiler-builtins`', compile.rs:315:13
Build completed unsuccessfully in 0:00:00
