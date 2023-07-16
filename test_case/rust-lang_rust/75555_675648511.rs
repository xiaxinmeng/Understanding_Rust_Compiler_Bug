plain
    Finished release [optimized + debuginfo] target(s) in 1m 11s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 71.695
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_ENABLE_PROJECTS=clang;compiler-rt" "-DLLVM_VERSION_SUFFIX=-rust-1.47.0-nightly" "-DLLVM_USE_LINKER=lld" "-DPYTHON_EXECUTABLE=/usr/bin/python3" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=clang" "-DCMAKE_CXX_COMPILER=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_ASM_COMPILER=/usr/bin/cc" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 9.0.0
-- The ASM compiler identification is GNU
-- Found assembler: /usr/bin/cc
-- Check for working C compiler: /usr/bin/clang
---
[  5%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divsi3.c.o
[  5%] Building CXX object projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan_dynamic.x86_64.dir/hwasan_exceptions.cpp.o
[  5%] Building CXX object projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan_dynamic.x86_64.dir/hwasan_interceptors.cpp.o
[  5%] Building ASM object projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan_dynamic.x86_64.dir/hwasan_interceptors_vfork.S.o
cc: error: unrecognized argument to ‘-fno-sanitize=’ option: ‘safe-stack’
cc: error: unrecognized debug output level ‘line-tables-only’
make[2]: *** [projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan_dynamic.x86_64.dir/build.make:128: projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan_dynamic.x86_64.dir/hwasan_interceptors_vfork.S.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  5%] Building CXX object projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan_dynamic.x86_64.dir/hwasan_memintrinsics.cpp.o
[  5%] Building CXX object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_suppressions.cpp.o
[  5%] Building CXX object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_posix.cpp.o
[  5%] Building CXX object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_premap_shadow.cpp.o
---
[  5%] Building CXX object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_win.cpp.o
[  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/OptRSTEmitter.cpp.o
[  5%] Building CXX object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_shadow_setup.cpp.o
[  5%] Building ASM object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_interceptors_vfork.S.o
cc: error: unrecognized argument to ‘-fno-sanitize=’ option: ‘safe-stack’
cc: error: unrecognized debug output level ‘line-tables-only’
make[2]: *** [projects/compiler-rt/lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/build.make:453: projects/compiler-rt/lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_interceptors_vfork.S.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/PseudoLoweringEmitter.cpp.o
[  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/PseudoLoweringEmitter.cpp.o
make[1]: *** [CMakeFiles/Makefile2:24562: projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan_dynamic.x86_64.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/RegisterBankEmitter.cpp.o
Scanning dependencies of target RTHwasan.x86_64
[  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/RegisterInfoEmitter.cpp.o
[  5%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divti3.c.o
[  5%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divti3.c.o
[  5%] Building CXX object projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan.x86_64.dir/hwasan.cpp.o
[  6%] Building CXX object projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan.x86_64.dir/hwasan_allocator.cpp.o
[  6%] Building CXX object projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan.x86_64.dir/hwasan_dynamic_shadow.cpp.o
[  6%] Building CXX object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_stack.cpp.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divtf3.c.o
make[1]: *** [CMakeFiles/Makefile2:22719: projects/compiler-rt/lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/all] Error 2
[  6%] Building CXX object projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan.x86_64.dir/hwasan_interceptors.cpp.o
[  6%] Building ASM object projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan.x86_64.dir/hwasan_interceptors_vfork.S.o
[  6%] Building ASM object projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan.x86_64.dir/hwasan_interceptors_vfork.S.o
cc: error: unrecognized argument to ‘-fno-sanitize=’ option: ‘safe-stack’
cc: error: unrecognized debug output level ‘line-tables-only’
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extendsfdf2.c.o
make[2]: *** [projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan.x86_64.dir/build.make:128: projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan.x86_64.dir/hwasan_interceptors_vfork.S.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  6%] Building CXX object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_stats.cpp.o
[  6%] Building CXX object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_suppressions.cpp.o
[  6%] Building CXX object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_thread.cpp.o
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SDNodeProperties.cpp.o
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SDNodeProperties.cpp.o
[  6%] Building CXX object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_win.cpp.o
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SearchableTableEmitter.cpp.o
[  6%] Building ASM object projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_interceptors_vfork.S.o
cc: error: unrecognized argument to ‘-fno-sanitize=’ option: ‘safe-stack’
cc: error: unrecognized debug output level ‘line-tables-only’
make[2]: *** [projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/build.make:453: projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_interceptors_vfork.S.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SubtargetEmitter.cpp.o
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SubtargetEmitter.cpp.o
make[1]: *** [CMakeFiles/Makefile2:24669: projects/compiler-rt/lib/hwasan/CMakeFiles/RTHwasan.x86_64.dir/all] Error 2
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ffsdi2.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ffssi2.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ffsti2.c.o
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/Types.cpp.o
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/Types.cpp.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixdfdi.c.o
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86DisassemblerTables.cpp.o
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86EVEX2VEXTablesEmitter.cpp.o
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86FoldTablesEmitter.cpp.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixdfsi.c.o
make[1]: *** [CMakeFiles/Makefile2:22645: projects/compiler-rt/lib/asan/CMakeFiles/RTAsan.x86_64.dir/all] Error 2
[  7%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.o
[  7%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixdfti.c.o
[  7%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixsfdi.c.o
[  7%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixsfsi.c.o
---
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/mulxc3.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/i386/fp_mode.c.o
[ 10%] Linking C static library ../../../../lib/clang/10.0.1/lib/linux/libclang_rt.builtins-x86_64.a
[ 10%] Built target clang_rt.builtins-x86_64
make: *** [Makefile:152: all] Error 2
command did not execute successfully, got: exit code: 2


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
 finished in 33.144
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 build
Build completed unsuccessfully in 0:01:45
== clock drift check ==
== clock drift check ==
  local time: Tue Aug 18 18:42:12 UTC 2020
  network time: Tue, 18 Aug 2020 18:42:12 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4137) (node)
Terminate orphan process: pid (4165) (python)
