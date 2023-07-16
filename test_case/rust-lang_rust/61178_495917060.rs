plain
wry
xmoto
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1558788346~hmac=a3e136e4c77d2da80c8861f053ef3df879acc42769eb0c0d74f8d067c9f04751&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX19PGZGD4bxQWJGkfZE427nCtMRcrJv4Ifynf3v9Na91hiVW4VIyV_uTHR5ByiTxXUfouSRw1uPXam-nh3UTqJwPUDv5LhKsyi0zOLzG40Bi4TZuEkSEoF1jf_WvBdyC9FeRHBjcDwBdDQ&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1558788358~hmac=bb51c65b7d6c5cfce5c816721e78028e0b23ab686536082bec8ee1fd362306fd&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_LncBz_9G2M1yj4IVrth11Cz_g7j2sQQ92GFBkv-GL-3roxy4638A4B2NM04dg_w9u8heu8tcYjO1y0kMMFcqL_9IhDtOXRWfs20hLEw-nU02ycOAU2msNKAEkAxUokj5NE_-mgHesEA&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/ae/aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7?__gda__=exp=1558788360~hmac=4afde61a089f2b7c27771413de2dd3b5d6e2ca860f14421092d7aeb399939f62&response-content-disposition=attachment%3Bfilename%3D%22swig-4.0.0.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18apJLvnix89O7QWwQEaeZgpOTc_Ya9pj1FGr0KwNMI-8dNuFnVoXlkm2RQpg-lH637ADpOY_B4uVfkcc5q2fwTDY_yZ8M-nIITSAQyoRhX7gZf_-i136QrnL8xx3bEGjX-eExPRXQyvA&response-X-Checksum-Sha1=a9c428aee4337d91061a69c02d7ae508b627d03f&response-X-Checksum-Sha2=aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7
🍺  /usr/local/Cellar/swig/4.0.0: 722 files, 5.4MB
travis_time:end:0e258b13:start=1558787297127563000,finish=1558787678162363000,duration=381034800000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:21]       Memory: 8 GB
[00:03:21]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:21]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:21]       SMC Version (system): 2.8f0
[00:03:21]       Serial Number (system): VM/dqmKr2caQ
[00:03:21] 
[00:03:21] hw.ncpu: 4
[00:03:21] hw.byteorder: 1234
[00:03:21] hw.memsize: 8589934592
---
[00:27:47] git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[00:27:47] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-apple-darwin
[00:27:47] running: "cmake" "/Users/travis/build/rust-lang/rust/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-apple-darwin" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DCMAKE_EXE_LINKER_FLAGS=-static-libstdc++" "-DLLVM_ENABLE_PROJECTS=clang;lldb;compiler-rt" "-DLLDB_CODESIGN_IDENTITY=" "-DLLDB_NO_DEBUGSERVER=ON" "-DLLVM_VERSION_SUFFIX=-rust-1.36.0-nightly" "-DPYTHON_EXECUTABLE=/usr/local/bin/python2.7" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=/Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang" "-DCMAKE_CXX_COMPILER=/Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/travis/build/rust-lang/rust=/rustc/llvm" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/travis/build/rust-lang/rust=/rustc/llvm -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:27:47] -- The CXX compiler identification is Clang 7.0.0
[00:27:47] -- The ASM compiler identification is Clang
[00:27:47] -- Found assembler: /Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang
[00:27:47] -- Check for working C compiler: /Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang
---
[00:28:21] -- Targeting NVPTX
[00:28:21] -- Targeting Hexagon
[00:28:21] -- Targeting WebAssembly
[00:28:22] -- Targeting RISCV
[00:28:22] -- Looking for unwind.h
[00:28:22] -- Looking for unwind.h - found
[00:28:22] -- Looking for rpc/xdr.h
[00:28:22] -- Looking for rpc/xdr.h - not found
[00:28:23] -- Looking for fopen in c - found
[00:28:23] -- Looking for __gcc_personality_v0 in gcc_s
[00:28:23] -- Looking for __gcc_personality_v0 in gcc_s - not found
[00:28:23] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[00:28:23] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[00:28:23] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG - Success
[00:28:23] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG
[00:28:23] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG - Success
[00:28:23] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG
[00:28:24] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG - Success
[00:28:24] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG
[00:28:24] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG - Success
[00:28:24] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG
[00:28:24] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG - Success
[00:28:24] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG
[00:28:24] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG - Success
[00:28:24] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG
[00:28:25] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG - Success
[00:28:25] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG
[00:28:25] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG - Success
[00:28:25] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG
[00:28:25] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG - Success
[00:28:25] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG
[00:28:25] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG - Success
[00:28:25] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG
[00:28:26] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG - Success
[00:28:26] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG
[00:28:26] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG - Success
[00:28:26] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG
[00:28:26] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG - Success
[00:28:26] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG
[00:28:26] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG - Failed
[00:28:26] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG
[00:28:26] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG - Success
[00:28:26] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC
[00:28:27] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC - Success
[00:28:27] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG
[00:28:27] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG - Success
[00:28:27] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG
[00:28:27] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG - Failed
[00:28:27] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG
[00:28:27] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG - Failed
[00:28:27] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG
[00:28:27] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG - Success
[00:28:27] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG
[00:28:27] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG - Failed
[00:28:27] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG
[00:28:28] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
[00:28:28] -- Performing Test COMPILER_RT_HAS_GR_FLAG
[00:28:28] -- Performing Test COMPILER_RT_HAS_GR_FLAG - Failed
[00:28:28] -- Performing Test COMPILER_RT_HAS_GS_FLAG
[00:28:28] -- Performing Test COMPILER_RT_HAS_GS_FLAG - Failed
[00:28:28] -- Performing Test COMPILER_RT_HAS_MT_FLAG
[00:28:28] -- Performing Test COMPILER_RT_HAS_MT_FLAG - Failed
[00:28:28] -- Performing Test COMPILER_RT_HAS_Oy_FLAG
[00:28:28] -- Performing Test COMPILER_RT_HAS_Oy_FLAG - Failed
[00:28:28] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG
[00:28:28] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG - Success
[00:28:28] -- Performing Test COMPILER_RT_HAS_G_FLAG
[00:28:29] -- Performing Test COMPILER_RT_HAS_G_FLAG - Success
[00:28:29] -- Performing Test COMPILER_RT_HAS_Zi_FLAG
[00:28:29] -- Performing Test COMPILER_RT_HAS_Zi_FLAG - Failed
[00:28:29] -- Performing Test COMPILER_RT_HAS_WALL_FLAG
[00:28:29] -- Performing Test COMPILER_RT_HAS_WALL_FLAG - Success
[00:28:29] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG
[00:28:29] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG - Failed
[00:28:29] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG
[00:28:29] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG - Failed
[00:28:29] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG
[00:28:29] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG - Failed
[00:28:29] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG
[00:28:29] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG - Failed
[00:28:29] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG
[00:28:29] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG - Failed
[00:28:30] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG
[00:28:30] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG - Failed
[00:28:30] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG
[00:28:30] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG - Failed
[00:28:30] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG
[00:28:30] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG - Failed
[00:28:30] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG
[00:28:30] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG - Failed
[00:28:30] -- Performing Test COMPILER_RT_HAS_W4_FLAG
[00:28:30] -- Performing Test COMPILER_RT_HAS_W4_FLAG - Failed
[00:28:30] -- Performing Test COMPILER_RT_HAS_WX_FLAG
[00:28:30] -- Performing Test COMPILER_RT_HAS_WX_FLAG - Failed
[00:28:30] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG
[00:28:30] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG - Failed
[00:28:30] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG
[00:28:30] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG - Failed
[00:28:30] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG
[00:28:31] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG - Failed
[00:28:31] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG
[00:28:31] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG - Failed
[00:28:31] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG
[00:28:31] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG - Failed
[00:28:31] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG
[00:28:31] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG - Failed
[00:28:31] -- Looking for __func__
[00:28:31] -- Looking for __func__ - found
[00:28:31] -- Looking for dlopen in dl - found
[00:28:31] -- Looking for shm_open in rt
[00:28:32] -- Looking for shm_open in rt - not found
[00:28:32] -- Looking for pow in m
[00:28:32] -- Looking for pow in m
[00:28:32] -- Looking for pow in m - found
[00:28:32] -- Looking for pthread_create in pthread
[00:28:32] -- Looking for pthread_create in pthread - found
[00:28:32] -- Looking for backtrace in execinfo
[00:28:32] -- Looking for backtrace in execinfo - not found
[00:28:32] -- Looking for __cxa_throw in c++
[00:28:32] -- Looking for __cxa_throw in c++ - found
[00:28:32] -- Looking for __cxa_throw in stdc++
[00:28:33] -- Looking for __cxa_throw in stdc++ - found
[00:28:42] -- Performing Test COMPILER_RT_HAS_APP_EXTENSION
[00:28:42] -- Performing Test COMPILER_RT_HAS_APP_EXTENSION - Success
[00:28:42] -- Got ld supported ARCHES: armv6 armv7 armv7s arm64 i386 x86_64 x86_64h armv6m armv7k armv7m armv7em (tvOS)
[00:28:42] -- Toolchain supported arches: armv6;armv7;armv7s;arm64;i386;x86_64;x86_64h;armv6m;armv7k;armv7m;armv7em;(tvOS)
[00:28:42] -- Finding valid architectures for osx...
[00:28:45] -- OSX supported arches: i386;x86_64
[00:28:45] -- Finding valid architectures for iossim...
[00:28:47] -- ios Simulator supported arches: i386;x86_64
[00:28:47] -- Finding valid architectures for ios...
[00:28:50] -- ios supported arches: armv7;armv7s;arm64;armv7k
[00:28:50] -- Compiler-RT supported architectures: i386;x86_64;armv7;armv7s;arm64;armv7k
[00:28:50] -- Performing Test COMPILER_RT_HAS_STD_C11_FLAG
[00:28:50] -- Performing Test COMPILER_RT_HAS_STD_C11_FLAG - Success
[00:28:50] -- Performing Test COMPILER_RT_HAS_VISIBILITY_HIDDEN_FLAG
[00:28:50] -- Performing Test COMPILER_RT_HAS_VISIBILITY_HIDDEN_FLAG - Success
[00:28:50] -- Performing Test COMPILER_RT_HAS_OMIT_FRAME_POINTER_FLAG
[00:28:50] -- Performing Test COMPILER_RT_HAS_OMIT_FRAME_POINTER_FLAG - Success
[00:28:50] -- Performing Test COMPILER_RT_HAS_FREESTANDING_FLAG
[00:28:50] -- Performing Test COMPILER_RT_HAS_FREESTANDING_FLAG - Success
[00:28:50] -- Performing Test COMPILER_RT_HAS_XRAY_COMPILER_FLAG
[00:28:50] -- Performing Test COMPILER_RT_HAS_XRAY_COMPILER_FLAG - Failed
[00:28:50] -- Performing Test COMPILER_RT_HAS_ATOMIC_KEYWORD
[00:28:50] -- Performing Test COMPILER_RT_HAS_ATOMIC_KEYWORD - Success
[00:28:50] -- OSX supported arches: i386;x86_64
[00:28:50] -- Using cached valid architectures for iossim.
[00:28:50] -- ios Simulator supported builtin arches: i386;x86_64
[00:28:50] -- Using cached valid architectures for ios.
[00:28:50] -- ios supported builtin arches: armv7;armv7s;arm64;armv7k
[00:28:50] -- Builtin supported architectures: i386;x86_64;armv7;armv7k;armv7s;arm64
[00:28:50] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS
[00:28:51] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
[00:28:51] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
[00:28:51] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
[00:28:51] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
[00:28:51] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
[00:28:51] -- Performing Test HAS_THREAD_LOCAL - Success
[00:28:51] -- Looking for sys/resource.h
[00:28:52] -- Looking for sys/resource.h - found
[00:28:52] -- Clang version: 8.0.0
---
[00:30:46] [  4%] Built target count
[00:30:46] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:46] [  4%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypeVTable.cpp.o
[00:30:46] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:46] Scanning dependencies of target RTXrayFDR.osx
[00:30:46] [  4%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeRecordMapping.cpp.o
[00:30:46] [  4%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.osx.dir/xray_fdr_flags.cc.o
[00:30:46] [  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Path.cpp.o
[00:30:46] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:46] [  4%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypeVTableShape.cpp.o
[00:30:46] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:46] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:46] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.cc:15:
[00:30:46] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.h:18:
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:23:7: warning: '__sanitizer::FlagHandlerBase' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:46] class FlagHandlerBase {
[00:30:46]       ^
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<bool>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:46] class FlagHandler : public FlagHandlerBase {
[00:30:46]       ^
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:54:13: note: in instantiation of template class '__sanitizer::FlagHandler<bool>' requested here
[00:30:46] inline bool FlagHandler<bool>::Parse(const char *value) {
[00:30:46]             ^
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:46] class FlagHandler : public FlagHandlerBase {
[00:30:46]       ^
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:61:13: note: in instantiation of template class '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' requested here
[00:30:46] inline bool FlagHandler<HandleSignalMode>::Parse(const char *value) {
[00:30:46]             ^
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<const char *>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:46] class FlagHandler : public FlagHandlerBase {
[00:30:46]       ^
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:77:13: note: in instantiation of template class '__sanitizer::FlagHandler<const char *>' requested here
[00:30:46] inline bool FlagHandler<const char *>::Parse(const char *value) {
[00:30:46]             ^
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<int>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:46] class FlagHandler : public FlagHandlerBase {
[00:30:46]       ^
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:83:13: note: in instantiation of template class '__sanitizer::FlagHandler<int>' requested here
[00:30:46] inline bool FlagHandler<int>::Parse(const char *value) {
[00:30:46]             ^
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<unsigned long>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:46] class FlagHandler : public FlagHandlerBase {
[00:30:46]       ^
[00:30:46] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:92:13: note: in instantiation of template class '__sanitizer::FlagHandler<unsigned long>' requested here
[00:30:46] inline bool FlagHandler<uptr>::Parse(const char *value) {
[00:30:46] 6 warnings generated.
[00:30:46] 6 warnings generated.
[00:30:46] [  4%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.osx.dir/xray_fdr_logging.cc.o
[00:30:46] [  4%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeStreamMerger.cpp.o
[00:30:47] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:47] [  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Process.cpp.o
[00:30:47] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
---
[00:30:47] [  5%] Linking CXX static library ../../libLLVMDebugInfoCodeView.a
[00:30:47] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:47] [  5%] Built target LLVMDebugInfoCodeView
[00:30:47] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/RWMutex.cpp.o
[00:30:47] Scanning dependencies of target RTXray.osx
[00:30:47] [  5%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.osx.dir/xray_buffer_queue.cc.o
[00:30:47] [  5%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymDumper.cpp.o
[00:30:48] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:48] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Signals.cpp.o
[00:30:48] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
---
[00:30:48] [  5%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiModuleDescriptor.cpp.o
[00:30:48] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:48] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:48] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ThreadLocal.cpp.o
[00:30:48] [  5%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.osx.dir/xray_init.cc.o
[00:30:49] [  5%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiModuleDescriptorBuilder.cpp.o
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:49] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Threading.cpp.o
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:49] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_init.cc:21:
[00:30:49] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_flags.h:18:
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:23:7: warning: '__sanitizer::FlagHandlerBase' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<bool>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:54:13: note: in instantiation of template class '__sanitizer::FlagHandler<bool>' requested here
[00:30:49] inline bool FlagHandler<bool>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:61:13: note: in instantiation of template class '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' requested here
[00:30:49] inline bool FlagHandler<HandleSignalMode>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<const char *>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:77:13: note: in instantiation of template class '__sanitizer::FlagHandler<const char *>' requested here
[00:30:49] inline bool FlagHandler<const char *>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<int>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:83:13: note: in instantiation of template class '__sanitizer::FlagHandler<int>' requested here
[00:30:49] inline bool FlagHandler<int>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<unsigned long>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:92:13: note: in instantiation of template class '__sanitizer::FlagHandler<unsigned long>' requested here
[00:30:49] inline bool FlagHandler<uptr>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_init.cc:34:48: warning: zero size arrays are an extension [-Wzero-length-array]
[00:30:49] const XRaySledEntry __start_xray_instr_map[] = {};
[00:30:49]                                                ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_init.cc:35:54: warning: zero size arrays are an extension [-Wzero-length-array]
[00:30:49] extern const XRaySledEntry __stop_xray_instr_map[] = {};
[00:30:49]                                                      ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_init.cc:36:60: warning: zero size arrays are an extension [-Wzero-length-array]
[00:30:49] extern const XRayFunctionSledIndex __start_xray_fn_idx[] = {};
[00:30:49]                                                            ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_init.cc:37:59: warning: zero size arrays are an extension [-Wzero-length-array]
[00:30:49] extern const XRayFunctionSledIndex __stop_xray_fn_idx[] = {};
[00:30:49] 10 warnings generated.
[00:30:49] 10 warnings generated.
[00:30:49] [  5%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.osx.dir/xray_flags.cc.o
[00:30:49] [  5%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiModuleList.cpp.o
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:49] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_fdr_logging.cc:36:
[00:30:49] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.h:18:
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:23:7: warning: '__sanitizer::FlagHandlerBase' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<bool>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:54:13: note: in instantiation of template class '__sanitizer::FlagHandler<bool>' requested here
[00:30:49] inline bool FlagHandler<bool>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:61:13: note: in instantiation of template class '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' requested here
[00:30:49] inline bool FlagHandler<HandleSignalMode>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<const char *>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:77:13: note: in instantiation of template class '__sanitizer::FlagHandler<const char *>' requested here
[00:30:49] inline bool FlagHandler<const char *>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<int>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:83:13: note: in instantiation of template class '__sanitizer::FlagHandler<int>' requested here
[00:30:49] inline bool FlagHandler<int>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<unsigned long>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:92:13: note: in instantiation of template class '__sanitizer::FlagHandler<unsigned long>' requested here
[00:30:49] inline bool FlagHandler<uptr>::Parse(const char *value) {
[00:30:49] 6 warnings generated.
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:49] [  5%] Built target RTXrayFDR.osx
[00:30:49] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Valgrind.cpp.o
[00:30:49] Scanning dependencies of target RTSanitizerCommonLibc.osx
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:49] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_flags.cc:15:
[00:30:49] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_flags.h:18:
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:23:7: warning: '__sanitizer::FlagHandlerBase' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<bool>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:54:13: note: in instantiation of template class '__sanitizer::FlagHandler<bool>' requested here
[00:30:49] inline bool FlagHandler<bool>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:61:13: note: in instantiation of template class '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' requested here
[00:30:49] inline bool FlagHandler<HandleSignalMode>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<const char *>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:77:13: note: in instantiation of template class '__sanitizer::FlagHandler<const char *>' requested here
[00:30:49] inline bool FlagHandler<const char *>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<int>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:83:13: note: in instantiation of template class '__sanitizer::FlagHandler<int>' requested here
[00:30:49] inline bool FlagHandler<int>::Parse(const char *value) {
[00:30:49]             ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<unsigned long>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:49] class FlagHandler : public FlagHandlerBase {
[00:30:49]       ^
[00:30:49] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:92:13: note: in instantiation of template class '__sanitizer::FlagHandler<unsigned long>' requested here
[00:30:49] inline bool FlagHandler<uptr>::Parse(const char *value) {
[00:30:49] 6 warnings generated.
[00:30:49] 6 warnings generated.
[00:30:49] [  5%] Building CXX object projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.osx.dir/sanitizer_common_libcdep.cc.o
[00:30:49] [  5%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.osx.dir/xray_interface.cc.o
[00:30:49] clang-7: error: cannot use 'c++-cpp-output' output with multiple -arch options
[00:30:49] make[3]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.osx.dir/sanitizer_common_libcdep.cc.o] Error 1
[00:30:49] make[2]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.osx.dir/all] Error 2
[00:30:49] make[2]: *** Waiting for unfinished jobs....
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:49] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Watchdog.cpp.o
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:49] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:50] [  5%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.osx.dir/xray_log_interface.cc.o
[00:30:50] [  5%] Linking CXX static library ../libLLVMSupport.a
[00:30:50] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:50] [  5%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiStreamBuilder.cpp.o
[00:30:50] [  5%] Built target LLVMSupport
[00:30:50] [  5%] Built target LLVMSupport
[00:30:50] [  5%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.osx.dir/xray_utils.cc.o
[00:30:50] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:50] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_interface.cc:36:
[00:30:50] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_flags.h:18:
[00:30:50] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_flags.h:18:
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:23:7: warning: '__sanitizer::FlagHandlerBase' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:50] class FlagHandlerBase {
[00:30:50]       ^
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<bool>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:50] class FlagHandler : public FlagHandlerBase {
[00:30:50]       ^
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:54:13: note: in instantiation of template class '__sanitizer::FlagHandler<bool>' requested here
[00:30:50] inline bool FlagHandler<bool>::Parse(const char *value) {
[00:30:50]             ^
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:50] class FlagHandler : public FlagHandlerBase {
[00:30:50]       ^
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:61:13: note: in instantiation of template class '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' requested here
[00:30:50] inline bool FlagHandler<HandleSignalMode>::Parse(const char *value) {
[00:30:50]             ^
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<const char *>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:50] class FlagHandler : public FlagHandlerBase {
[00:30:50]       ^
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:77:13: note: in instantiation of template class '__sanitizer::FlagHandler<const char *>' requested here
[00:30:50] inline bool FlagHandler<const char *>::Parse(const char *value) {
[00:30:50]             ^
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<int>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:50] class FlagHandler : public FlagHandlerBase {
[00:30:50]       ^
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:83:13: note: in instantiation of template class '__sanitizer::FlagHandler<int>' requested here
[00:30:50] inline bool FlagHandler<int>::Parse(const char *value) {
[00:30:50]             ^
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<unsigned long>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:50] class FlagHandler : public FlagHandlerBase {
[00:30:50]       ^
[00:30:50] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:92:13: note: in instantiation of template class '__sanitizer::FlagHandler<unsigned long>' requested here
[00:30:50] inline bool FlagHandler<uptr>::Parse(const char *value) {
[00:30:50] 6 warnings generated.
[00:30:50] 6 warnings generated.
[00:30:50] [  5%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.osx.dir/xray_x86_64.cc.o
[00:30:50] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:50] [  5%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/GlobalsStream.cpp.o
[00:30:50] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:50] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:50] [  5%] Building C object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.osx.dir/xray_trampoline_x86_64.S.o
[00:30:51] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:51] [  5%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/HashTable.cpp.o
[00:30:51] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:51] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:51] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_utils.cc:19:
[00:30:51] In file included from /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/xray_flags.h:18:
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:23:7: warning: '__sanitizer::FlagHandlerBase' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:51] class FlagHandlerBase {
[00:30:51]       ^
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<bool>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:51] class FlagHandler : public FlagHandlerBase {
[00:30:51]       ^
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:54:13: note: in instantiation of template class '__sanitizer::FlagHandler<bool>' requested here
[00:30:51] inline bool FlagHandler<bool>::Parse(const char *value) {
[00:30:51]             ^
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:51] class FlagHandler : public FlagHandlerBase {
[00:30:51]       ^
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:61:13: note: in instantiation of template class '__sanitizer::FlagHandler<__sanitizer::HandleSignalMode>' requested here
[00:30:51] inline bool FlagHandler<HandleSignalMode>::Parse(const char *value) {
[00:30:51]             ^
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<const char *>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:51] class FlagHandler : public FlagHandlerBase {
[00:30:51]       ^
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:77:13: note: in instantiation of template class '__sanitizer::FlagHandler<const char *>' requested here
[00:30:51] inline bool FlagHandler<const char *>::Parse(const char *value) {
[00:30:51]             ^
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<int>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:51] class FlagHandler : public FlagHandlerBase {
[00:30:51]       ^
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:83:13: note: in instantiation of template class '__sanitizer::FlagHandler<int>' requested here
[00:30:51] inline bool FlagHandler<int>::Parse(const char *value) {
[00:30:51]             ^
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:29:7: warning: '__sanitizer::FlagHandler<unsigned long>' has virtual functions but non-virtual destructor [-Wnon-virtual-dtor]
[00:30:51] class FlagHandler : public FlagHandlerBase {
[00:30:51]       ^
[00:30:51] /Users/travis/build/rust-lang/rust/src/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:92:13: note: in instantiation of template class '__sanitizer::FlagHandler<unsigned long>' requested here
[00:30:51] inline bool FlagHandler<uptr>::Parse(const char *value) {
[00:30:51] 6 warnings generated.
[00:30:51] [  5%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStream.cpp.o
[00:30:51] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:51] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:51] [  5%] Built target RTXray.osx
[00:30:51] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:51] [  5%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/ModuleDebugStream.cpp.o
[00:30:51] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:51] [  5%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeCompilandSymbol.cpp.o
---
[00:30:54] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:54] clang-7: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[00:30:54] [  6%] Linking CXX static library ../../libLLVMDebugInfoPDB.a
[00:30:54] [  6%] Built target LLVMDebugInfoPDB
[00:30:54] make[1]: *** [all] Error 2
[00:30:54] command did not execute successfully, got: exit code: 2
[00:30:54] 
[00:30:54] 
[00:30:54] build script failed, must exit now', /Users/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
[00:30:54]  finished in 187.326
[00:30:54] travis_fold:end:llvm

[00:30:54] travis_time:end:llvm:start=1558789345745938000,finish=1558789533063324000,duration=187317386000
---
travis_fold:start:after_failure.2
travis_time:start:1afdf353
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:254d9f30
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:254d9f30:start=1558789538090884000,finish=1558789538121003000,duration=30119000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:036cbc88
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11c1ba39
travis_time:start:11c1ba39
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a0f314b
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
