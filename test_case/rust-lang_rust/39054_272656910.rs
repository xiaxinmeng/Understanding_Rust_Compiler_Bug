
~/Desktop/rust/rust-tmp $ ./x.py build --stage 0 -j1                                                                                                                    
   Compiling build_helper v0.1.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/build_helper)
   Compiling libc v0.2.17
   Compiling gcc v0.3.40
   Compiling rustc-serialize v0.3.19
   Compiling getopts v0.2.14
   Compiling filetime v0.1.10
   Compiling cmake v0.1.18
   Compiling num_cpus v0.2.13
   Compiling toml v0.1.30
   Compiling bootstrap v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/bootstrap)
    Finished debug [unoptimized] target(s) in 16.84 secs
Synchronising submodule url for 'src/compiler-rt'
Synchronising submodule url for 'src/jemalloc'
Synchronising submodule url for 'src/liblibc'
Synchronising submodule url for 'src/llvm'
Synchronising submodule url for 'src/rt/hoedown'
Synchronising submodule url for 'src/rust-installer'
HEAD is now at a8fc4c1 Merge pull request #28 from xen0n/preprocessor-firefighting
HEAD is now at 9858987 [WIP] sparc64-linux support
HEAD is now at a3736a0 Merge pull request #6 from intelfx/patch-1
HEAD is now at 4f99485 Merge pull request #54 from brson/docdir
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling libc v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/rustc/libc_shim)
   Compiling core v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/libcore)
   Compiling alloc v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/liballoc)
   Compiling build_helper v0.1.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/build_helper)
   Compiling rand v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/librand)
   Compiling gcc v0.3.40
   Compiling std v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/libstd)
   Compiling panic_abort v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/libpanic_abort)
   Compiling compiler_builtins v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/libcompiler_builtins)
   Compiling unwind v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/libunwind)
   Compiling alloc_system v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/liballoc_system)
   Compiling panic_unwind v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/libpanic_unwind)
   Compiling std_unicode v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/libstd_unicode)
   Compiling collections v0.0.0 (file:///home/aidanhs/Desktop/rust/rust-tmp/src/libcollections)
error[E0463]: can't find crate for `alloc_jemalloc`

error: aborting due to previous error

error: Could not compile `std`.

To learn more, run the command again with --verbose.


command did not execute successfully: "/home/aidanhs/.cargo/bin/cargo" "build" "-j" "1" "--target" "x86_64-unknown-linux-gnu" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "/home/aidanhs/Desktop/rust/rust-tmp/src/rustc/std_shim/Cargo.toml"
expected success, got: exit code: 101
