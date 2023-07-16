plain
2019-10-30T02:17:13.3287920Z     Finished release [optimized] target(s) in 15m 52s
2019-10-30T02:17:13.4048670Z Copying stage0 rustc from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
2019-10-30T02:17:13.4415520Z [TIMING] Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } } -- 952.957
2019-10-30T02:17:13.4421950Z Building LLVM for x86_64-apple-darwin
2019-10-30T02:17:13.4436090Z running: "cmake" "/Users/runner/runners/2.159.2/work/1/s/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-apple-darwin" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DCOMPILER_RT_BUILD_SANITIZERS=ON" "-DCOMPILER_RT_USE_LIBCXX=OFF" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_ENABLE_PROJECTS=compiler-rt" "-DLLVM_VERSION_SUFFIX=-rust-1.40.0-dev" "-DPYTHON_EXECUTABLE=/usr/local/bin/python2.7" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=/Users/runner/runners/2.159.2/work/1/s/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang" "-DCMAKE_CXX_COMPILER=/Users/runner/runners/2.159.2/work/1/s/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++" "-DCMAKE_INSTALL_PREFIX=/Users/runner/runners/2.159.2/work/1/s/build/x86_64-apple-darwin/llvm" "-DCMAKE_BUILD_TYPE=Release"
2019-10-30T02:17:14.0774940Z -- The CXX compiler identification is Clang 7.0.0
2019-10-30T02:17:14.1279340Z -- The ASM compiler identification is Clang
2019-10-30T02:17:14.1321310Z -- Found assembler: /Users/runner/runners/2.159.2/work/1/s/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang
2019-10-30T02:17:14.1697750Z -- Check for working C compiler: /Users/runner/runners/2.159.2/work/1/s/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang
---
2019-10-30T02:17:59.4580660Z -- Looking for __cxa_throw in stdc++
2019-10-30T02:17:59.7042710Z -- Looking for __cxa_throw in stdc++ - found
2019-10-30T02:17:59.7066880Z -- Performing Test COMPILER_RT_HAS_Z_TEXT
2019-10-30T02:17:59.9103700Z -- Performing Test COMPILER_RT_HAS_Z_TEXT - Failed
2019-10-30T02:18:09.4135960Z -- Performing Test COMPILER_RT_HAS_APP_EXTENSION
2019-10-30T02:18:09.6205030Z -- Performing Test COMPILER_RT_HAS_APP_EXTENSION - Success
2019-10-30T02:18:09.6364410Z -- Got ld supported ARCHES: armv6 armv7 armv7s arm64 i386 x86_64 x86_64h armv6m armv7k armv7m armv7em (tvOS)
2019-10-30T02:18:09.6365390Z -- Toolchain supported arches: armv6;armv7;armv7s;arm64;i386;x86_64;x86_64h;armv6m;armv7k;armv7m;armv7em;(tvOS)
2019-10-30T02:18:09.6366180Z -- Finding valid architectures for osx...
2019-10-30T02:18:11.8295390Z -- OSX supported arches: i386;x86_64;x86_64h
2019-10-30T02:18:11.8296520Z -- Finding valid architectures for iossim...
2019-10-30T02:18:13.8260940Z -- ios Simulator supported arches: i386;x86_64
2019-10-30T02:18:13.8261880Z -- Finding valid architectures for ios...
2019-10-30T02:18:16.4161700Z -- ios supported arches: armv7;armv7s;arm64;armv7k
2019-10-30T02:18:16.4240790Z -- Compiler-RT supported architectures: i386;x86_64;x86_64h;armv7;armv7s;arm64;armv7k
2019-10-30T02:18:16.5475520Z -- Performing Test COMPILER_RT_HAS_VISIBILITY_HIDDEN_FLAG - Success
2019-10-30T02:18:16.5476500Z -- Performing Test COMPILER_RT_HAS_OMIT_FRAME_POINTER_FLAG
2019-10-30T02:18:16.5925840Z -- Performing Test COMPILER_RT_HAS_OMIT_FRAME_POINTER_FLAG - Success
2019-10-30T02:18:16.5926700Z -- Performing Test COMPILER_RT_HAS_FREESTANDING_FLAG
2019-10-30T02:18:16.5926700Z -- Performing Test COMPILER_RT_HAS_FREESTANDING_FLAG
2019-10-30T02:18:16.6365530Z -- Performing Test COMPILER_RT_HAS_FREESTANDING_FLAG - Success
2019-10-30T02:18:16.6366480Z -- Performing Test COMPILER_RT_HAS_XRAY_COMPILER_FLAG
2019-10-30T02:18:16.6586740Z -- Performing Test COMPILER_RT_HAS_XRAY_COMPILER_FLAG - Failed
2019-10-30T02:18:16.6587670Z -- Performing Test COMPILER_RT_HAS_ATOMIC_KEYWORD
2019-10-30T02:18:16.7133600Z -- Performing Test COMPILER_RT_HAS_ATOMIC_KEYWORD - Success
2019-10-30T02:18:16.7207860Z -- OSX supported arches: i386;x86_64;x86_64h
2019-10-30T02:18:16.7208940Z -- Using cached valid architectures for iossim.
2019-10-30T02:18:16.7209730Z -- ios Simulator supported builtin arches: i386;x86_64
2019-10-30T02:18:16.7210870Z -- Using cached valid architectures for ios.
2019-10-30T02:18:16.7211920Z -- ios supported builtin arches: armv7;armv7s;arm64;armv7k
2019-10-30T02:18:16.7217090Z -- Builtin supported architectures: i386;x86_64;x86_64h;armv7;armv7k;armv7s;arm64
2019-10-30T02:18:17.3663980Z -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
2019-10-30T02:18:17.3771240Z -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
2019-10-30T02:18:17.6168450Z -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
2019-10-30T02:18:17.6177470Z -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
---
2019-10-30T02:19:13.6045340Z Scanning dependencies of target yaml-bench
2019-10-30T02:19:13.6318990Z [  8%] Building CXX object utils/yaml-bench/CMakeFiles/yaml-bench.dir/YAMLBench.cpp.o
2019-10-30T02:19:13.6995310Z [  8%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/Hash.cpp.o
2019-10-30T02:19:13.7587870Z [  8%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordPrinter.cpp.o
2019-10-30T02:19:13.8070590Z Scanning dependencies of target RTfuzzer.osx
2019-10-30T02:19:13.8334140Z [  8%] Building CXX object projects/compiler-rt/lib/fuzzer/CMakeFiles/RTfuzzer.osx.dir/FuzzerCrossOver.cpp.o
2019-10-30T02:19:13.9053230Z clang-7: error: cannot use 'c++-cpp-output' output with multiple -arch options
2019-10-30T02:19:13.9064040Z make[2]: *** [projects/compiler-rt/lib/fuzzer/CMakeFiles/RTfuzzer.osx.dir/FuzzerCrossOver.cpp.o] Error 1
2019-10-30T02:19:13.9067830Z make[1]: *** [projects/compiler-rt/lib/fuzzer/CMakeFiles/RTfuzzer.osx.dir/all] Error 2
2019-10-30T02:19:13.9068210Z make[1]: *** Waiting for unfinished jobs....
2019-10-30T02:19:14.5381630Z [  8%] Linking CXX executable ../../bin/yaml-bench
2019-10-30T02:19:14.5389290Z [  8%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Trace.cpp.o
2019-10-30T02:19:14.5390770Z [  8%] Built target yaml-bench
2019-10-30T02:19:14.5395540Z [  8%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStream.cpp.o
---
2019-10-30T02:19:17.0199600Z [  8%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/TpiHashing.cpp.o
2019-10-30T02:19:17.0940150Z [  8%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/TpiStreamBuilder.cpp.o
2019-10-30T02:19:17.4352260Z [  8%] Linking CXX static library ../../libLLVMDebugInfoPDB.a
2019-10-30T02:19:17.5234560Z [  8%] Built target LLVMDebugInfoPDB
2019-10-30T02:19:17.5244570Z make: *** [all] Error 2
2019-10-30T02:19:17.5263570Z command did not execute successfully, got: exit code: 2
2019-10-30T02:19:17.5263640Z 
2019-10-30T02:19:17.5263640Z 
2019-10-30T02:19:17.5264460Z build script failed, must exit now', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
2019-10-30T02:19:17.5322170Z  finished in 124.095
2019-10-30T02:19:17.5347550Z failed to run: /Users/runner/runners/2.159.2/work/1/s/build/bootstrap/debug/bootstrap test
2019-10-30T02:19:17.5348160Z Build completed unsuccessfully in 0:21:28
2019-10-30T02:19:17.5423310Z == clock drift check ==
2019-10-30T02:19:17.5423310Z == clock drift check ==
2019-10-30T02:19:17.5479520Z   local time: Wed Oct 30 02:19:17 UTC 2019
2019-10-30T02:19:17.5812530Z   network time: Wed, 30 Oct 2019 02:19:17 GMT
2019-10-30T02:19:17.5815380Z == end clock drift check ==
2019-10-30T02:19:17.5859190Z 
2019-10-30T02:19:17.6008040Z ##[error]Bash exited with code '1'.
2019-10-30T02:19:17.6071590Z ##[section]Starting: Checkout
2019-10-30T02:19:17.6074740Z ==============================================================================
2019-10-30T02:19:17.6074870Z Task         : Get sources
2019-10-30T02:19:17.6074970Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
