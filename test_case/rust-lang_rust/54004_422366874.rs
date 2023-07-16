plain

[00:17:15] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-gnu
[00:17:15] running: "cmake" "/checkout/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=cc" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=c++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=RelWithDebInfo"
[00:17:15] -- The CXX compiler identification is GNU 5.4.0
[00:17:15] -- The ASM compiler identification is GNU
[00:17:15] -- Found assembler: /usr/local/bin/sccache
[00:17:15] -- Check for working C compiler: /usr/local/bin/sccache
---
[00:17:33]     LLVM_OCAML_INSTALL_PATH
[00:17:33] 
[00:17:33] 
[00:17:33] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build
[00:17:33] running: "cmake" "--build" "." "--target" "install" "--config" "RelWithDebInfo" "--" "-j" "4"
[00:17:34] [  0%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.o
[00:17:34] Scanning dependencies of target LLVMTableGen
[00:17:34] [  0%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Error.cpp.o
[00:17:34] Scanning dependencies of target obj.llvm-tblgen
---
[01:13:17] [100%] Built target llvm-cov
[01:13:20] [100%] Linking CXX executable ../../bin/llvm-size
[01:13:20] [100%] Built target llvm-size
[01:13:20] Install the project...
[01:13:20] -- Install configuration: "RelWithDebInfo"
[01:13:20] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/InitializePasses.h
[01:13:20] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support
[01:13:20] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TargetRegistry.h
[01:13:20] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Solaris
---
[01:15:33] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-mc
[01:15:33] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-extract
[01:15:33] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-size
[01:15:33] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVMExports.cmake
[01:15:33] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVMExports-relwithdebinfo.cmake
[01:15:33] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVMConfigVersion.cmake
[01:15:33] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVM-Config.cmake
[01:15:33] -- Up-to-date: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/.
[01:15:33] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindLibpfm.cmake
---
[01:15:47]    Compiling rustc-demangle v0.1.9
[01:15:49]    Compiling memmap v0.6.2
[01:15:51]    Compiling num_cpus v1.8.0
[01:15:53]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[01:16:47] LLVM ERROR: IO failure on output stream: No space left on device
[01:16:47] error: Could not compile `rustc_llvm`.
[01:16:47] To learn more, run the command again with --verbose.
[01:16:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json"
[01:16:47] expected success, got: exit code: 101
[01:16:47] expected success, got: exit code: 101
[01:16:47] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1135:9
[01:16:47] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm

---
travis_time:end:02c5de0e:start=1537272528898609273,finish=1537272528911604044,duration=12994771
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13769835
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1eb3339d
travis_time:start:1eb3339d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ac037fd
$ dmesg | grep -i kill
