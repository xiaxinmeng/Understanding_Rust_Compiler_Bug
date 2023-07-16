plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 8bd12e8cca3f28f302b9cc0f1f47bb64bd1f98fd and 7e3c248f1b2b823240ae56c5fa7711e7ec68299d
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
+ browser-ui-test@0.9.7
added 49 packages from 86 contributors in 10.082s
Removing intermediate container ce9476804190
 ---> db37aecac5b7
Step 14/15 : ENV RUST_CONFIGURE_ARGS   --set llvm.allow-old-toolchain   --build=x86_64-unknown-linux-gnu   --save-toolstates=/tmp/toolstate/toolstates.json
Removing intermediate container 57999a5d253e
 ---> e00fcb67fc67
Step 15/15 : ENV SCRIPT /tmp/checktools.sh ../x.py &&   NODE_PATH=`npm root -g` python3 ../x.py test src/test/rustdoc-gui --stage 2
 ---> Running in 3984e556fdf5
 ---> Running in 3984e556fdf5
Removing intermediate container 3984e556fdf5
 ---> c32e0df803eb
Successfully built c32e0df803eb
Successfully tagged rust-ci:latest
Built container sha256:c32e0df803ebaea19ca5dc5c6f9a91de8a6ac271bc569e908c8cbe7522f0223a
Uploading finished image to https://ci-caches.rust-lang.org/docker/4efbbf0681fd3e9468ecd1be7221f14775c2110e0e111af3eb5743a8d97c5bf6130c3330faf4d2621c626205d50eb9f69718760884e147eae94a85054b0f58c5
upload failed: - to s3://rust-lang-ci-sccache2/docker/4efbbf0681fd3e9468ecd1be7221f14775c2110e0e111af3eb5743a8d97c5bf6130c3330faf4d2621c626205d50eb9f69718760884e147eae94a85054b0f58c5 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
configure: build.submodules     := False
configure: build.build          := x86_64-unknown-linux-gnu
configure: rust.debug-assertions := True
configure: rust.channel         := nightly
configure: llvm.allow-old-toolchain := True
configure: rust.verify-llvm-ir  := True
configure: rust.verify-llvm-ir  := True
configure: build.configure-args := ['--set', 'llvm.allow-old-toolchain', '--build ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
CMAKE_x86_64-unknown-linux-gnu = None
CMAKE_x86_64_unknown_linux_gnu = None
HOST_CMAKE = None
CMAKE = None
running: "cmake" "/checkout/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.64.0-nightly" "-DLLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN=YES" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_ASM_COMPILER=cc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is GNU 5.4.0
-- The ASM compiler identification is GNU
-- Found assembler: /usr/bin/cc
-- Check for working C compiler: /usr/bin/cc
---
-- Detecting CXX compiler ABI info - done
-- Detecting CXX compile features
-- Detecting CXX compile features - done
CMake Warning at cmake/modules/CheckCompilerVersion.cmake:42 (message):
  Host GCC version should be at least 7.1 because LLVM will soon use new C++
  features which your toolchain version doesn't support.  Your version is
  5.4.0.  Ignoring because you've set LLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN,
  but very soon your toolchain won't be supported.
  cmake/modules/CheckCompilerVersion.cmake:49 (check_compiler_version)
  cmake/config-ix.cmake:15 (include)
  CMakeLists.txt:766 (include)

---
-- Found Python3: /usr/bin/python3.5 (found suitable version "3.5.2", minimum required is "3.0") found components:  Interpreter 
-- Linker detection: GNU ld
-- Performing Test HAS_WERROR_GLOBAL_CTORS
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Failed
-- Looking for __x86_64__
-- Looking for __x86_64__ - found
-- LLVMHello ignored -- Loadable modules not supported on this platform.
-- Targeting AArch64
-- Targeting ARM
-- Targeting BPF
---
[5/3020] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.o
[6/3020] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/RustDemangle.cpp.o
[7/3020] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/DLangDemangle.cpp.o
[8/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/AddressRanges.cpp.o
[9/3020] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3.c.o
[10/3020] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_dispatch.c.o
[11/3020] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_portable.c.o
[12/3020] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_neon.c.o
[13/3020] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_sse2_x86-64_unix.S.o
[14/3020] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_sse41_x86-64_unix.S.o
[15/3020] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_avx2_x86-64_unix.S.o
[16/3020] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_avx512_x86-64_unix.S.o
[18/3020] Generating VCSRevision.h
[19/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SmallVector.cpp.o
[20/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SourceMgr.cpp.o
[21/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/AMDGPUMetadata.cpp.o
---
[61/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DAGDeltaAlgorithm.cpp.o
[62/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DJB.cpp.o
[63/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ELFAttributeParser.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/Demangle/ItaniumDemangle.cpp:14:0:
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:194:21: warning: 'llvm::itanium_demangle::Node::Precedence' is too small to hold all values of 'enum class llvm::itanium_demangle::Node::Prec'
   Prec Precedence : 6;
                     ^
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:200:29: warning: 'llvm::itanium_demangle::Node::RHSComponentCache' is too small to hold all values of 'enum class llvm::itanium_demangle::Node::Cache'
   Cache RHSComponentCache : 2;
                             ^
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:204:22: warning: 'llvm::itanium_demangle::Node::ArrayCache' is too small to hold all values of 'enum class llvm::itanium_demangle::Node::Cache'
   Cache ArrayCache : 2;
                      ^
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:208:25: warning: 'llvm::itanium_demangle::Node::FunctionCache' is too small to hold all values of 'enum class llvm::itanium_demangle::Node::Cache'
   Cache FunctionCache : 2;
In file included from /checkout/src/llvm-project/llvm/lib/Demangle/ItaniumDemangle.cpp:14:0:
In file included from /checkout/src/llvm-project/llvm/lib/Demangle/ItaniumDemangle.cpp:14:0:
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h: In instantiation of 'struct llvm::itanium_demangle::AbstractManglingParser<llvm::itanium_demangle::ManglingParser<{anonymous}::DefaultAllocator>, {anonymous}::DefaultAllocator>::OperatorInfo':
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:4530:30:   required from 'llvm::itanium_demangle::Node* llvm::itanium_demangle::AbstractManglingParser<Derived, Alloc>::parseExpr() [with Derived = llvm::itanium_demangle::ManglingParser<{anonymous}::DefaultAllocator>; Alloc = {anonymous}::DefaultAllocator]'
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:3903:41:   required from 'llvm::itanium_demangle::Node* llvm::itanium_demangle::AbstractManglingParser<Derived, Alloc>::parseType() [with Derived = llvm::itanium_demangle::ManglingParser<{anonymous}::DefaultAllocator>; Alloc = {anonymous}::DefaultAllocator]'
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:5466:37:   required from 'llvm::itanium_demangle::Node* llvm::itanium_demangle::AbstractManglingParser<Derived, Alloc>::parse() [with Derived = llvm::itanium_demangle::ManglingParser<{anonymous}::DefaultAllocator>; Alloc = {anonymous}::DefaultAllocator]'
/checkout/src/llvm-project/llvm/lib/Demangle/ItaniumDemangle.cpp:380:28:   required from here
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:2593:23: warning: 'llvm::itanium_demangle::AbstractManglingParser<llvm::itanium_demangle::ManglingParser<{anonymous}::DefaultAllocator>, {anonymous}::DefaultAllocator>::OperatorInfo::Prec' is too small to hold all values of 'enum class llvm::itanium_demangle::Node::Prec'
     Node::Prec Prec : 7; // Precedence
[64/3020] Linking CXX static library lib/libLLVMDemangle.a
[65/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ELFAttributes.cpp.o
[66/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Error.cpp.o
[67/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.o
---
[134/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Unicode.cpp.o
[135/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeCaseFold.cpp.o
[136/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeNameToCodepoint.cpp.o
[137/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/VersionTuple.cpp.o
[138/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeNameToCodepointGenerated.cpp.o
[140/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/WithColor.cpp.o
[141/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/X86TargetParser.cpp.o
[142/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/YAMLParser.cpp.o
[143/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/YAMLTraits.cpp.o
[143/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/YAMLTraits.cpp.o
[144/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/raw_os_ostream.cpp.o
[145/3020] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/raw_ostream.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/Support/ItaniumManglingCanonicalizer.cpp:13:0:
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:194:21: warning: 'llvm::itanium_demangle::Node::Precedence' is too small to hold all values of 'enum class llvm::itanium_demangle::Node::Prec'
   Prec Precedence : 6;
                     ^
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:200:29: warning: 'llvm::itanium_demangle::Node::RHSComponentCache' is too small to hold all values of 'enum class llvm::itanium_demangle::Node::Cache'
   Cache RHSComponentCache : 2;
                             ^
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:204:22: warning: 'llvm::itanium_demangle::Node::ArrayCache' is too small to hold all values of 'enum class llvm::itanium_demangle::Node::Cache'
   Cache ArrayCache : 2;
                      ^
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:208:25: warning: 'llvm::itanium_demangle::Node::FunctionCache' is too small to hold all values of 'enum class llvm::itanium_demangle::Node::Cache'
   Cache FunctionCache : 2;
In file included from /checkout/src/llvm-project/llvm/lib/Support/ItaniumManglingCanonicalizer.cpp:13:0:
In file included from /checkout/src/llvm-project/llvm/lib/Support/ItaniumManglingCanonicalizer.cpp:13:0:
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h: In instantiation of 'struct llvm::itanium_demangle::AbstractManglingParser<llvm::itanium_demangle::ManglingParser<{anonymous}::CanonicalizerAllocator>, {anonymous}::CanonicalizerAllocator>::OperatorInfo':
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:4530:30:   required from 'llvm::itanium_demangle::Node* llvm::itanium_demangle::AbstractManglingParser<Derived, Alloc>::parseExpr() [with Derived = llvm::itanium_demangle::ManglingParser<{anonymous}::CanonicalizerAllocator>; Alloc = {anonymous}::CanonicalizerAllocator]'
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:3903:41:   required from 'llvm::itanium_demangle::Node* llvm::itanium_demangle::AbstractManglingParser<Derived, Alloc>::parseType() [with Derived = llvm::itanium_demangle::ManglingParser<{anonymous}::CanonicalizerAllocator>; Alloc = {anonymous}::CanonicalizerAllocator]'
/checkout/src/llvm-project/llvm/lib/Support/ItaniumManglingCanonicalizer.cpp:229:36:   required from here
/checkout/src/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h:2593:23: warning: 'llvm::itanium_demangle::AbstractManglingParser<llvm::itanium_demangle::ManglingParser<{anonymous}::CanonicalizerAllocator>, {anonymous}::CanonicalizerAllocator>::OperatorInfo::Prec' is too small to hold all values of 'enum class llvm::itanium_demangle::Node::Prec'
     Node::Prec Prec : 7; // Precedence
[146/3020] Building C object lib/Support/CMakeFiles/LLVMSupport.dir/regcomp.c.o
[147/3020] Building C object lib/Support/CMakeFiles/LLVMSupport.dir/regerror.c.o
[147/3020] Building C object lib/Support/CMakeFiles/LLVMSupport.dir/regerror.c.o
FAILED: sccache /usr/bin/c++  -DGTEST_HAS_RTTI=0 -D_DEBUG -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib/Support -I/checkout/src/llvm-project/llvm/lib/Support -Iinclude -I/checkout/src/llvm-project/llvm/include -ffunction-sections -fdata-sections -fPIC -m64 -fPIC -fno-semantic-interposition -fvisibility-inlines-hidden -Werror=date-time -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -fdiagnostics-color -ffunction-sections -fdata-sections -O3 -DNDEBUG   -UNDEBUG -std=c++14  -fno-exceptions -fno-rtti -MD -MT lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeNameToCodepoint.cpp.o -MF lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeNameToCodepoint.cpp.o.d -o lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeNameToCodepoint.cpp.o -c /checkout/src/llvm-project/llvm/lib/Support/UnicodeNameToCodepoint.cpp
/checkout/src/llvm-project/llvm/lib/Support/UnicodeNameToCodepoint.cpp: In instantiation of 'llvm::sys::unicode::nearestMatchesForCodepointName(llvm::StringRef, std::size_t)::<lambda(const llvm::sys::unicode::Node&, std::size_t, auto:4&)> [with auto:4 = llvm::sys::unicode::nearestMatchesForCodepointName(llvm::StringRef, std::size_t)::<lambda(const llvm::sys::unicode::Node&, std::size_t, auto:4&)>; std::size_t = long unsigned int]':
/checkout/src/llvm-project/llvm/lib/Support/UnicodeNameToCodepoint.cpp:544:31:   required from here
/checkout/src/llvm-project/llvm/lib/Support/UnicodeNameToCodepoint.cpp:511:33: internal compiler error: Segmentation fault
       for (std::size_t I = 1; I < Columns; I++) {
                                 ^
Please submit a full bug report,
with preprocessed source if appropriate.
See <file:///usr/share/doc/gcc-5/README.Bugs> for instructions.
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.48/src/lib.rs:975:5
 finished in 36.676 seconds
Build completed unsuccessfully in 0:01:30
cat: /tmp/toolstate/toolstates.json: No such file or directory
