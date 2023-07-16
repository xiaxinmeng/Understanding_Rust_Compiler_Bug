plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1186aaf6
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:06:23]  ---> 1cef20bb1afc
[00:06:23] Step 8/11 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:06:23]  ---> Using cache
[00:06:23]  ---> 7d1ef14c8d98
[00:06:23] Step 9/11 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:06:23]  ---> bf1e8cdbc88a
[00:06:23]  ---> bf1e8cdbc88a
[00:06:23] Step 10/11 : ENV RUSTFLAGS="-C target-feature=-crt-static"
[00:06:23]  ---> 53cc77f0865f
[00:06:23] Step 11/11 : ENV SCRIPT       python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:06:23]  ---> Using cache
[00:06:23]  ---> fc1d2b453b8b
---

[00:58:35] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-musl
[00:58:35] running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-musl" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NATIVE_BUILD=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=x86_64-linux-musl-gcc" "-DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -static" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -static -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-musl/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:58:35] -- The CXX compiler identification is GNU 6.3.0
[00:58:35] -- The ASM compiler identification is GNU
[00:58:35] -- Found assembler: /usr/local/bin/x86_64-linux-musl-gcc
[00:58:35] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
---
[01:01:42] [ 67%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64CompressJumpTables.cpp.o
[01:01:42] [ 67%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsInstrInfo.cpp.o
[01:01:43] [ 67%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsInstructionSelector.cpp.o
[01:01:43] [ 67%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelDAGToDAG.cpp.o
[01:01:43] /checkout/src/llvm-project/llvm/lib/Target/Mips/MipsInstructionSelector.cpp: In member function 'virtual bool {anonymous}::MipsInstructionSelector::select(llvm::MachineInstr&, llvm::CodeGenCoverage&) const':
[01:01:43] /checkout/src/llvm-project/llvm/lib/Target/Mips/MipsInstructionSelector.cpp:296:16: warning: types may not be defined in a for-range-declaration
[01:01:43]      for (const struct Instr &Instruction : Instructions) {
[01:01:43] [ 67%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelLowering.cpp.o
[01:01:43] [ 67%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64ConditionOptimizer.cpp.o
[01:01:43] [ 67%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86RetpolineThunks.cpp.o
[01:01:44] [ 67%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsFrameLowering.cpp.o
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-musl -> x86_64-unknown-linux-musl)
[01:40:52] 
[01:40:52] running 119 tests
[01:41:21] .iiiii..ii.....i..i...i..i.i..i.ii..ii....ii.ii....i..........iiii..........i...ii..ii.......ii.i.ii 100/119
[01:41:25] i.....iiiiii.....ii
[01:41:25] 
[01:41:25]  finished in 33.353
[01:41:25] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-musl -> x86_64-unknown-linux-musl)
[02:47:52] 
[02:47:52] running 15 tests
[02:47:53] ..iiiiiii.ii...
[02:47:53] 
[02:47:53]  finished in 0.815
[02:47:53] travis_fold:end:test_run-make

