
╭─kfairmasterz@Android ~/upstream/rust  ‹fb2d79578c› 
╰─➤  ./x.py test src/libstd --stage 1                           
    Finished dev [unoptimized] target(s) in 0.0 secs
Synchronizing submodule url for 'cargo'
Synchronizing submodule url for 'src/compiler-rt'
Synchronizing submodule url for 'src/doc/nomicon'
Synchronizing submodule url for 'src/doc/reference'
Synchronizing submodule url for 'src/jemalloc'
Synchronizing submodule url for 'src/liblibc'
Synchronizing submodule url for 'src/llvm'
Synchronizing submodule url for 'src/rt/hoedown'
Synchronizing submodule url for 'src/rust-installer'
HEAD is now at 5f3b9c4c Auto merge of #3807 - nerdrew:nerdrew/_cargo, r=alexcrichton
HEAD is now at d30da544 Merge pull request #30 from japaric/msan
HEAD is now at d08fe97 Add Gankro's table to nomicon/src/phantom-data.md
HEAD is now at 2d23ea6 doc: Use "macOS" terminology consistently
HEAD is now at 11bfb0d Merge pull request #16 from glandium/rust
HEAD is now at 64d954c6 Auto merge of #533 - raphlinus:master, r=alexcrichton
HEAD is now at 859fb269364 Merge pull request #64 from pftbest/sret
Removing utils/llvm-build/llvmbuild/__init__.pyc
Removing utils/llvm-build/llvmbuild/componentinfo.pyc
Removing utils/llvm-build/llvmbuild/configutil.pyc
Removing utils/llvm-build/llvmbuild/main.pyc
Removing utils/llvm-build/llvmbuild/util.pyc
HEAD is now at da282f1 Merge pull request #8 from GuillaumeGomez/line_information
HEAD is now at 4f99485 Merge pull request #54 from brson/docdir
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libcore)
   Compiling alloc v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/liballoc)
   Compiling compiler_builtins v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libcompiler_builtins)
   Compiling std_unicode v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libstd_unicode)
   Compiling rand v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/librand)
   Compiling libc v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/rustc/libc_shim)
   Compiling collections v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libcollections)
   Compiling alloc_jemalloc v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/liballoc_jemalloc)
   Compiling unwind v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libunwind)
   Compiling panic_unwind v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libpanic_unwind)
   Compiling panic_abort v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libpanic_abort)
   Compiling alloc_system v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/liballoc_system)
   Compiling rustc_lsan v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/librustc_lsan)
   Compiling rustc_tsan v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/librustc_tsan)
   Compiling rustc_msan v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/librustc_msan)
   Compiling rustc_asan v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/librustc_asan)
   Compiling std v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libstd)
    Finished release [optimized] target(s) in 39.7 secs
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling getopts v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libgetopts)
   Compiling term v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libterm)
   Compiling test v0.0.0 (file:///home/kfairmasterz/upstream/rust/src/libtest)
    Finished release [optimized] target(s) in 12.52 secs
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/home/kfairmasterz/upstream/rust/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;JSBackend;MSP430;Sparc;NVPTX" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_INSTALL_PREFIX=/home/kfairmasterz/upstream/rust/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
System is unknown to cmake, create:
Platform/[proxychains] DLL init: proxychains-ng 4.12
[proxychains] DLL init: proxychains-ng 4.12
Linux to use this system, please send your config file to cmake@www.cmake.org so it can be added to cmake
Your CMakeCache.txt file was copied to CopyOfCMakeCache.txt. Please send that file to cmake@www.cmake.org.
CMake Warning at CMakeLists.txt:73 (message):
  Job pooling is only available with Ninja generators.


-- Target triple: x86_64-unknown-linux-gnu
-- Native target architecture is X86
-- Threads enabled.
-- Doxygen disabled.
-- Sphinx disabled.
-- Go bindings enabled.
-- OCaml bindings disabled, need ctypes >=0.4.
CMake Error at cmake/modules/HandleLLVMOptions.cmake:88 (MESSAGE):
  Unable to determine platform
Call Stack (most recent call first):
  CMakeLists.txt:474 (include)


-- Building with -fPIC
-- Constructing LLVMBuild project information
-- Targeting X86
-- Targeting ARM
-- Targeting AArch64
-- Targeting Mips
-- Targeting PowerPC
-- Targeting SystemZ
-- Targeting JSBackend
-- Targeting MSP430
-- Targeting Sparc
-- Targeting NVPTX
-- Configuring incomplete, errors occurred!
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1

build script failed, must exit now', /home/kfairmasterz/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.21/src/lib.rs:605
note: Run with `RUST_BACKTRACE=1` for a backtrace.
Build completed unsuccessfully in 0:01:15
