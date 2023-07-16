
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> m68k-unknown-linux-gnu)
   Compiling core v0.0.0 (/data/home/glaubitz/rust-m68k/library/core)
   Compiling rustc-std-workspace-core v1.99.0 (/data/home/glaubitz/rust-m68k/library/rustc-std-workspace-core)
   Compiling compiler_builtins v0.1.69
   Compiling libc v0.2.118
rustc: /data/home/glaubitz/llvm-project/llvm/lib/IR/Constants.cpp:2016: static llvm::Constant* llvm::ConstantExpr::getCast(unsigned int, llvm::Constant*, llvm::Type*, bool): Assertion `CastInst::castIsValid(opc, C, Ty) && "Invalid constantexpr cast!"' failed.
rustc exited with signal: 6 (core dumped)
error: could not compile `core`

Caused by:
  process didn't exit successfully: `/data/home/glaubitz/rust-m68k/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C metadata=283e6e00a9b48510 -C extra-filename=-283e6e00a9b48510 --out-dir /data/home/glaubitz/rust-m68k/build/x86_64-unknown-linux-gnu/stage1-std/m68k-unknown-linux-gnu/release/deps --target m68k-unknown-linux-gnu -C linker=cc -L dependency=/data/home/glaubitz/rust-m68k/build/x86_64-unknown-linux-gnu/stage1-std/m68k-unknown-linux-gnu/release/deps -L dependency=/data/home/glaubitz/rust-m68k/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
warning: build failed, waiting for other jobs to finish...
LLVM ERROR: Cannot select: t6: ch = AtomicStore<(store unordered (s16) into %ir.9)> t3:1, t5, t3
  t5: i32,ch = CopyFromReg t0, Register:i32 %1
    t4: i32 = Register %1
  t3: i16,ch = AtomicLoad<(load unordered (s16) from %ir.10)> t0, t2
    t2: i32,ch = CopyFromReg t0, Register:i32 %2
      t1: i32 = Register %2
In function: __llvm_memcpy_element_unordered_atomic_2
error: build failed
Build completed unsuccessfully in 0:00:17
