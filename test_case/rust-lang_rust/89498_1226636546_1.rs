console
>cargo build -Zbuild-std --target m68k-unknown-linux-gnu
   Compiling compiler_builtins v0.1.79
   Compiling core v0.0.0 (/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling libc v0.2.131
   Compiling cc v1.0.73
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std)
   Compiling unwind v0.0.0 (/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x27babd3)[0x7fe8cff2cbd3]
/usr/lib/libc.so.6(+0x38a40)[0x7fe8cd418a40]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm12SelectionDAG7getNodeEjRKNS_5SDLocENS_3EVTENS_7SDValueENS_11SDNodeFlagsE+0x46)[0x7fe8cb0a2f86]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(+0x2a835fc)[0x7fe8caadc5fc]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZNK4llvm14TargetLowering11LowerCallToERNS0_16CallLoweringInfoE+0x125a)[0x7fe8cab7a3ea]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm19SelectionDAGBuilder11LowerCallToERKNS_8CallBaseENS_7SDValueEbbPKNS_10BasicBlockE+0x96e)[0x7fe8cab04d8e]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm19SelectionDAGBuilder9visitCallERKNS_8CallInstE+0x2ad)[0x7fe8caabeddd]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm16SelectionDAGISel20SelectAllBasicBlocksERKNS_8FunctionE+0xfdb)[0x7fe8caabadab]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm16SelectionDAGISel20runOnMachineFunctionERNS_15MachineFunctionE+0x376)[0x7fe8caef9136]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm13FPPassManager13runOnFunctionERNS_8FunctionE+0x890)[0x7fe8cb2c3990]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm13FPPassManager11runOnModuleERNS_6ModuleE+0x2f)[0x7fe8cb2c2f6f]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm6legacy15PassManagerImpl3runERNS_6ModuleE+0x224)[0x7fe8cb2c2774]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x2297c55)[0x7fe8cfa09c55]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x22975a9)[0x7fe8cfa095a9]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x229545a)[0x7fe8cfa0745a]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x2276a34)[0x7fe8cf9e8a34]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x2275863)[0x7fe8cf9e7863]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x22396e0)[0x7fe8cf9ab6e0]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-4393e7d07259b8a4.so(rust_metadata_std_e55f2e3b0b26feec+0xfd8b3)[0x7fe8cd6ed8b3]
/usr/lib/libc.so.6(+0x8678d)[0x7fe8cd46678d]
/usr/lib/libc.so.6(__clone+0x44)[0x7fe8cd4e78e4]
   Compiling alloc v0.0.0 (/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
LLVM ERROR: unable to allocate function return #2
error: could not compile `compiler_builtins`
warning: build failed, waiting for other jobs to finish...
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x27babd3)[0x7f01ded34bd3]
/usr/lib/libc.so.6(+0x38a40)[0x7f01dc220a40]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(+0x2f5a626)[0x7f01d9dbb626]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm16SelectionDAGISel17CodeGenAndEmitDAGEv+0x144c)[0x7f01d98eebcc]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm16SelectionDAGISel20SelectAllBasicBlocksERKNS_8FunctionE+0x13b6)[0x7f01d98c3186]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm16SelectionDAGISel20runOnMachineFunctionERNS_15MachineFunctionE+0x376)[0x7f01d9d01136]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm13FPPassManager13runOnFunctionERNS_8FunctionE+0x890)[0x7f01da0cb990]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm13FPPassManager11runOnModuleERNS_6ModuleE+0x2f)[0x7f01da0caf6f]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.65.0-nightly.so(_ZN4llvm6legacy15PassManagerImpl3runERNS_6ModuleE+0x224)[0x7f01da0ca774]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x2297c55)[0x7f01de811c55]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x22975a9)[0x7f01de8115a9]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x229545a)[0x7f01de80f45a]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x2276a34)[0x7f01de7f0a34]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x2275863)[0x7f01de7ef863]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-f10772d81e144ae7.so(+0x22396e0)[0x7f01de7b36e0]
/home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-4393e7d07259b8a4.so(rust_metadata_std_e55f2e3b0b26feec+0xfd8b3)[0x7f01dc4f58b3]
/usr/lib/libc.so.6(+0x8678d)[0x7f01dc26e78d]
/usr/lib/libc.so.6(__clone+0x44)[0x7f01dc2ef8e4]
LLVM ERROR: unable to allocate function return #2
error: could not compile `libc`
error: could not compile `adler`

Caused by:
  process didn't exit successfully: `rustc --crate-name adler /home/wcampbell/.cargo/registry/src/github.com-1ecc6299db9ec823/adler-0.2.3/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="compiler_builtins"' --cfg 'feature="core"' --cfg 'feature="rustc-dep-of-std"' -C metadata=0ab7c7911269cebb -C extra-filename=-0ab7c7911269cebb --out-dir /home/wcampbell/projects/wcampbell/code/m68k/target/m68k-unknown-linux-gnu/debug/deps --target m68k-unknown-linux-gnu -Z force-unstable-if-unmarked -L dependency=/home/wcampbell/projects/wcampbell/code/m68k/target/m68k-unknown-linux-gnu/debug/deps -L dependency=/home/wcampbell/projects/wcampbell/code/m68k/target/debug/deps --extern compiler_builtins=/home/wcampbell/projects/wcampbell/code/m68k/target/m68k-unknown-linux-gnu/debug/deps/libcompiler_builtins-fb1f862333bf4539.rmeta --extern core=/home/wcampbell/projects/wcampbell/code/m68k/target/m68k-unknown-linux-gnu/debug/deps/librustc_std_workspace_core-0320986fdb3fa22c.rmeta --cap-lints allow` (signal: 11, SIGSEGV: invalid memory reference)
error: could not compile `core`

Caused by:
  process didn't exit successfully: `rustc --crate-name core --edition=2021 /home/wcampbell/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=4cf3f75b36268316 -C extra-filename=-4cf3f75b36268316 --out-dir /home/wcampbell/projects/wcampbell/code/m68k/target/m68k-unknown-linux-gnu/debug/deps --target m68k-unknown-linux-gnu -Z force-unstable-if-unmarked -L dependency=/home/wcampbell/projects/wcampbell/code/m68k/target/m68k-unknown-linux-gnu/debug/deps -L dependency=/home/wcampbell/projects/wcampbell/code/m68k/target/debug/deps --cap-lints allow` (signal: 11, SIGSEGV: invalid memory reference)
