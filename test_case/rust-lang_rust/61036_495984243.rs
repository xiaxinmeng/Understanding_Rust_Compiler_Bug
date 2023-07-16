plain
wry
xmoto
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1558862271~hmac=dca29ac3556330f6f47fae59e38963fc1c7aacf5ec891e9d657bf803405c7bcf&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX19m2lbuMy-9c912o8CSUhfDdjtxMegwtVkTcVyIPSc6Bo-G7DyJxHmrxBQ1o-tIZw5Q1iEoXCkfS4sQnVj2G3b7ZrhxsBrEMfrY6hoakUm8QHNdl7_Yc4HLWqif8yVGUaStYazvvR6gRQ&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1558862282~hmac=6db2d76e5e0f25a96063c7a915fb683c744fb522f985cbdbf86ba25ae3624728&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_h5n_8Oc0a_bb2vklqliauGdCHQ_C-gUtwOKgHsonm66ACIwoA-Cp6AhU6nZXseBUwLIU54fb1sB3hIIgI2c6wGN_2XYD_-kKfJG2jkoOyq2CQ8Oc_rzAQGFlisRBGlxU7JyUrLRgFMg&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/ae/aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7?__gda__=exp=1558862285~hmac=dfe3885d43dd895d99ebc49029f9e01ff8a38fffbda7416a46b2fff8f37e0422&response-content-disposition=attachment%3Bfilename%3D%22swig-4.0.0.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_QDpzmtzz0LuIkLkqmOTf6GmA0YmgVTHGisBqVdikNhSchI4AxMGMfYgAEyZ63_G9wYDrZoHpZzaSnqtcy2RH_jYfOIcE1LSD3WQTeLvu9EpzbIj4ZiCjl1OzHQN9fbPrY9H7a7fIbaQ&response-X-Checksum-Sha1=a9c428aee4337d91061a69c02d7ae508b627d03f&response-X-Checksum-Sha2=aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7
🍺  /usr/local/Cellar/swig/4.0.0: 722 files, 5.4MB
travis_time:end:05dfc4b3:start=1558861216455675000,finish=1558861602435097000,duration=385979422000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:18]       Memory: 8 GB
[00:03:18]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:18]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:18]       SMC Version (system): 2.8f0
[00:03:18]       Serial Number (system): VMNRKXZpHJrq
[00:03:18] 
[00:03:18] hw.ncpu: 4
[00:03:18] hw.byteorder: 1234
[00:03:18] hw.memsize: 8589934592
---
[00:28:56] git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[00:28:56] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for i686-apple-darwin
[00:28:56] running: "cmake" "/Users/travis/build/rust-lang/rust/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=i686" "-DLLVM_DEFAULT_TARGET_TRIPLE=i686-apple-darwin" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DCMAKE_EXE_LINKER_FLAGS=-static-libstdc++" "-DLLVM_BUILD_32_BITS=ON" "-DLLVM_ENABLE_PROJECTS=clang;lldb;compiler-rt" "-DLLDB_CODESIGN_IDENTITY=" "-DLLDB_NO_DEBUGSERVER=ON" "-DLLVM_VERSION_SUFFIX=-rust-1.36.0-nightly" "-DPYTHON_EXECUTABLE=/usr/local/bin/python2.7" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=/Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang" "-DCMAKE_CXX_COMPILER=/Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=i686-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/travis/build/rust-lang/rust=/rustc/llvm" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=i686-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/travis/build/rust-lang/rust=/rustc/llvm -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:28:56] -- The CXX compiler identification is Clang 7.0.0
[00:28:56] -- The ASM compiler identification is Clang
[00:28:56] -- Found assembler: /Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang
[00:28:56] -- Check for working C compiler: /Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang
---
[00:29:30] -- Targeting NVPTX
[00:29:30] -- Targeting Hexagon
[00:29:30] -- Targeting WebAssembly
[00:29:31] -- Targeting RISCV
[00:29:31] -- Looking for unwind.h
[00:29:31] -- Looking for unwind.h - found
[00:29:31] -- Looking for rpc/xdr.h
[00:29:31] -- Looking for rpc/xdr.h - not found
[00:29:32] -- Looking for fopen in c - found
[00:29:32] -- Looking for __gcc_personality_v0 in gcc_s
[00:29:32] -- Looking for __gcc_personality_v0 in gcc_s - not found
[00:29:32] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[00:29:32] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[00:29:32] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG - Success
[00:29:32] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG
[00:29:32] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG - Success
[00:29:32] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG
[00:29:32] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG - Success
[00:29:32] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG
[00:29:33] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG - Success
[00:29:33] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG
[00:29:33] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG - Success
[00:29:33] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG
[00:29:33] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG - Success
[00:29:33] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG
[00:29:33] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG - Success
[00:29:33] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG
[00:29:33] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG - Success
[00:29:33] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG
[00:29:34] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG - Success
[00:29:34] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG
[00:29:34] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG - Success
[00:29:34] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG
[00:29:34] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG - Success
[00:29:34] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG
[00:29:34] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG - Success
[00:29:34] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG
[00:29:35] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG - Success
[00:29:35] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG
[00:29:35] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG - Failed
[00:29:35] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG
[00:29:35] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG - Success
[00:29:35] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC
[00:29:35] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC - Success
[00:29:35] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG
[00:29:35] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG - Success
[00:29:35] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG
[00:29:35] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG - Failed
[00:29:35] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG
[00:29:35] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG - Failed
[00:29:35] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG
[00:29:36] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG - Success
[00:29:36] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG
[00:29:36] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG - Failed
[00:29:36] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG
[00:29:36] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
[00:29:36] -- Performing Test COMPILER_RT_HAS_GR_FLAG
[00:29:36] -- Performing Test COMPILER_RT_HAS_GR_FLAG - Failed
[00:29:36] -- Performing Test COMPILER_RT_HAS_GS_FLAG
[00:29:36] -- Performing Test COMPILER_RT_HAS_GS_FLAG - Failed
[00:29:36] -- Performing Test COMPILER_RT_HAS_MT_FLAG
[00:29:36] -- Performing Test COMPILER_RT_HAS_MT_FLAG - Failed
[00:29:36] -- Performing Test COMPILER_RT_HAS_Oy_FLAG
[00:29:36] -- Performing Test COMPILER_RT_HAS_Oy_FLAG - Failed
[00:29:36] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG
[00:29:37] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG - Success
[00:29:37] -- Performing Test COMPILER_RT_HAS_G_FLAG
[00:29:37] -- Performing Test COMPILER_RT_HAS_G_FLAG - Success
[00:29:37] -- Performing Test COMPILER_RT_HAS_Zi_FLAG
[00:29:37] -- Performing Test COMPILER_RT_HAS_Zi_FLAG - Failed
[00:29:37] -- Performing Test COMPILER_RT_HAS_WALL_FLAG
[00:29:37] -- Performing Test COMPILER_RT_HAS_WALL_FLAG - Success
[00:29:37] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG
[00:29:37] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG - Failed
[00:29:37] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG
[00:29:38] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG - Failed
[00:29:38] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG
[00:29:38] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG - Failed
[00:29:38] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG
[00:29:38] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG - Failed
[00:29:38] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG
[00:29:38] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG - Failed
[00:29:38] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG
[00:29:38] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG - Failed
[00:29:38] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG
[00:29:38] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG - Failed
[00:29:38] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG
[00:29:38] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG - Failed
[00:29:38] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG
[00:29:38] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG - Failed
[00:29:38] -- Performing Test COMPILER_RT_HAS_W4_FLAG
[00:29:38] -- Performing Test COMPILER_RT_HAS_W4_FLAG - Failed
[00:29:38] -- Performing Test COMPILER_RT_HAS_WX_FLAG
[00:29:39] -- Performing Test COMPILER_RT_HAS_WX_FLAG - Failed
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG - Failed
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG - Failed
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG - Failed
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG - Failed
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG - Failed
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG
[00:29:39] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG - Failed
[00:29:39] -- Looking for __func__
[00:29:39] -- Looking for __func__ - found
[00:29:40] -- Looking for dlopen in dl - found
[00:29:40] -- Looking for shm_open in rt
[00:29:40] -- Looking for shm_open in rt - not found
[00:29:40] -- Looking for pow in m
[00:29:40] -- Looking for pow in m
[00:29:40] -- Looking for pow in m - found
[00:29:40] -- Looking for pthread_create in pthread
[00:29:40] -- Looking for pthread_create in pthread - found
[00:29:40] -- Looking for backtrace in execinfo
[00:29:41] -- Looking for backtrace in execinfo - not found
[00:29:41] -- Looking for __cxa_throw in c++
[00:29:41] -- Looking for __cxa_throw in c++ - found
[00:29:41] -- Looking for __cxa_throw in stdc++
[00:29:41] -- Looking for __cxa_throw in stdc++ - found
[00:29:50] -- Performing Test COMPILER_RT_HAS_APP_EXTENSION
[00:29:51] -- Performing Test COMPILER_RT_HAS_APP_EXTENSION - Success
[00:29:51] -- Got ld supported ARCHES: armv6 armv7 armv7s arm64 i386 x86_64 x86_64h armv6m armv7k armv7m armv7em (tvOS)
[00:29:51] -- Toolchain supported arches: armv6;armv7;armv7s;arm64;i386;x86_64;x86_64h;armv6m;armv7k;armv7m;armv7em;(tvOS)
[00:29:51] -- Finding valid architectures for osx...
[00:29:53] -- OSX supported arches: i386;x86_64
[00:29:53] -- Finding valid architectures for iossim...
[00:29:55] -- ios Simulator supported arches: i386;x86_64
[00:29:55] -- Finding valid architectures for ios...
[00:29:57] -- ios supported arches: armv7;armv7s;arm64;armv7k
[00:29:57] -- Compiler-RT supported architectures: i386;x86_64;armv7;armv7s;arm64;armv7k
[00:29:57] -- Performing Test COMPILER_RT_HAS_STD_C11_FLAG
[00:29:57] -- Performing Test COMPILER_RT_HAS_STD_C11_FLAG - Success
[00:29:57] -- Performing Test COMPILER_RT_HAS_VISIBILITY_HIDDEN_FLAG
[00:29:57] -- Performing Test COMPILER_RT_HAS_VISIBILITY_HIDDEN_FLAG - Success
[00:29:57] -- Performing Test COMPILER_RT_HAS_OMIT_FRAME_POINTER_FLAG
[00:29:58] -- Performing Test COMPILER_RT_HAS_OMIT_FRAME_POINTER_FLAG - Success
[00:29:58] -- Performing Test COMPILER_RT_HAS_FREESTANDING_FLAG
[00:29:58] -- Performing Test COMPILER_RT_HAS_FREESTANDING_FLAG - Success
[00:29:58] -- Performing Test COMPILER_RT_HAS_XRAY_COMPILER_FLAG
[00:29:58] -- Performing Test COMPILER_RT_HAS_XRAY_COMPILER_FLAG - Failed
[00:29:58] -- Performing Test COMPILER_RT_HAS_ATOMIC_KEYWORD
[00:29:58] -- Performing Test COMPILER_RT_HAS_ATOMIC_KEYWORD - Success
[00:29:58] -- OSX supported arches: i386;x86_64
[00:29:58] -- Using cached valid architectures for iossim.
[00:29:58] -- ios Simulator supported builtin arches: i386;x86_64
[00:29:58] -- Using cached valid architectures for ios.
[00:29:58] -- ios supported builtin arches: armv7;armv7s;arm64;armv7k
[00:29:58] -- Builtin supported architectures: i386;x86_64;armv7;armv7k;armv7s;arm64
[00:29:58] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS
[00:29:58] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
[00:29:58] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
[00:29:59] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
[00:29:59] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
[00:29:59] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
[00:29:59] -- Performing Test HAS_THREAD_LOCAL - Success
[00:29:59] -- Looking for sys/resource.h
[00:29:59] -- Looking for sys/resource.h - found
[00:29:59] -- Clang version: 8.0.0
---
[00:31:53] [  4%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeIndexDiscovery.cpp.o
[00:31:53] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:31:53] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:31:53] [  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Atomic.cpp.o
[00:31:53] Scanning dependencies of target RTUbsan.osx
[00:31:53] [  4%] Building CXX object projects/compiler-rt/lib/ubsan/CMakeFiles/RTUbsan.osx.dir/ubsan_diag.cc.o
[00:31:53] clang-7: error: cannot use 'c++-cpp-output' output with multiple -arch options
[00:31:53] make[3]: *** [projects/compiler-rt/lib/ubsan/CMakeFiles/RTUbsan.osx.dir/ubsan_diag.cc.o] Error 1
[00:31:53] make[2]: *** [projects/compiler-rt/lib/ubsan/CMakeFiles/RTUbsan.osx.dir/all] Error 2
[00:31:53] make[2]: *** Waiting for unfinished jobs....
[00:31:53] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:31:53] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:31:53] [  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DynamicLibrary.cpp.o
[00:31:53] [  4%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypeFriend.cpp.o
---
[00:32:00] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:32:00] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:32:00] [  6%] Linking CXX static library ../../libLLVMDebugInfoPDB.a
[00:32:00] [  6%] Built target LLVMDebugInfoPDB
[00:32:00] make[1]: *** [all] Error 2
[00:32:00] command did not execute successfully, got: exit code: 2
[00:32:00] 
[00:32:00] 
[00:32:00] build script failed, must exit now', /Users/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
[00:32:00]  finished in 184.664
[00:32:00] travis_fold:end:llvm

[00:32:00] travis_time:end:llvm:start=1558863338931482000,finish=1558863523586348000,duration=184654866000
---
travis_fold:start:after_failure.2
travis_time:start:0c6ad6e2
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1feba56a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:1feba56a:start=1558863526035711000,finish=1558863526065847000,duration=30136000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2a337f50
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e09f128
travis_time:start:0e09f128
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14fe24c8
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
