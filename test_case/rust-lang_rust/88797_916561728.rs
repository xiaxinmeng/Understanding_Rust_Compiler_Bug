plain
Updating files:  98% (29642/30246)
Updating files:  99% (29944/30246)
Updating files: 100% (30246/30246)
Updating files: 100% (30246/30246), done.
Note: switching to 'refs/remotes/pull/88797/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at ad1733c4 Merge 119db9eb2a7f86d977d3f8072f6c5f971097cc96 into 497ee321af3b8496eaccd7af7b437f18bab81abf
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'ad1733c41a952cd07b7a013e6702ae9cbb4a5fc2'
'ad1733c41a952cd07b7a013e6702ae9cbb4a5fc2'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
  EXTRA_VARIABLES: {
 "RUST_CONFIGURE_ARGS": "--build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler",
 "SCRIPT": "python x.py dist",
 "DIST_REQUIRE_ALL_TOOLS": 0,
 "CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION": "10.0.19041.0"
##[endgroup]
##[endgroup]
adding extra environment variable CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION
adding extra environment variable RUST_CONFIGURE_ARGS
adding extra environment variable SCRIPT
normal dist builder detected, setting DEPLOY=1
##[group]Run src/ci/scripts/should-skip-this.sh
##[group]Run src/ci/scripts/should-skip-this.sh
src/ci/scripts/should-skip-this.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  DEPLOY: 1
##[endgroup]
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  DEPLOY: 1
##[endgroup]
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  DEPLOY: 1
##[endgroup]
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  DEPLOY: 1
##[endgroup]
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  DEPLOY: 1
##[endgroup]
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  DEPLOY: 1
##[endgroup]
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  DEPLOY: 1
##[endgroup]
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  DEPLOY: 1
##[endgroup]
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
---
   Compiling test v0.0.0 (D:\a\rust\rust\library\test)
    Finished release [optimized + debuginfo] target(s) in 1m 27s
Copying stage0 std from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Building LLVM for x86_64-pc-windows-msvc
running: "cmake" "D:\\a\\rust\\rust\\src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=8" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-pc-windows-msvc" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZLIB=OFF" "-DLLVM_USE_CRT_DEBUG=MT" "-DLLVM_USE_CRT_RELEASE=MT" "-DLLVM_USE_CRT_RELWITHDEBINFO=MT" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.57.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION=10.0.19041.0" "-DCMAKE_C_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_CXX_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_C_FLAGS=-nologo -MT -Brepro --target=x86_64-pc-windows-msvc" "-DCMAKE_CXX_FLAGS=-nologo -MT -Brepro --target=x86_64-pc-windows-msvc" "-DCMAKE_INSTALL_PREFIX=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm" "-DCMAKE_ASM_FLAGS= -nologo -MT -Brepro" "-DCMAKE_ASM_COMPILER=C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64/cl.exe" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 12.0.0 with MSVC-like command-line
-- The ASM compiler identification is MSVC
-- Found assembler: C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64/cl.exe
-- Detecting C compiler ABI info
---
-- Generating done
CMake Warning:
  Manually-specified variables were not used by the project:

    CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION

-- Build files have been written to: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/build
running: "cmake" "--build" "." "--target" "install" "--config" "Release" "--" "-j" "8"
[1/2887] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\ABIBreak.cpp.obj
---
   Compiling rustc-main v0.0.0 (D:\a\rust\rust\compiler\rustc)
    Finished release [optimized] target(s) in 9m 33s
Copying stage0 rustc from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Building LLD for x86_64-pc-windows-msvc
running: "cmake" "D:\\a\\rust\\rust\\src/llvm-project/lld" "-G" "Ninja" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION=10.0.19041.0" "-DCMAKE_C_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_CXX_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_C_FLAGS=-nologo -MT -Brepro --target=x86_64-pc-windows-msvc" "-DCMAKE_CXX_FLAGS=-nologo -MT -Brepro --target=x86_64-pc-windows-msvc" "-DLLVM_CONFIG_PATH=D:\\a\\rust\\rust\\build\\bootstrap\\debug\\llvm-config-wrapper" "-DLLVM_INCLUDE_TESTS=OFF" "-DCMAKE_CXX_STANDARD=14" "-DCMAKE_INSTALL_PREFIX=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\lld" "-DCMAKE_ASM_FLAGS= -nologo -MD -Brepro" "-DCMAKE_ASM_COMPILER=C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64/cl.exe" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 12.0.0 with MSVC-like command-line
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info - done
-- Check for working C compiler: D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe - skipped
---
  Manually-specified variables were not used by the project:

    CMAKE_ASM_COMPILER
    CMAKE_ASM_FLAGS
    CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION

-- Build files have been written to: D:/a/rust/rust/build/x86_64-pc-windows-msvc/lld/build
running: "cmake" "--build" "." "--target" "install" "--config" "Release" "--" "-j" "8"
[1/157] Generating VCSVersion.inc
---
warning: 26 warnings generated.
warning: In file included from ../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingRuntime.cpp:11:
warning: In file included from ../../src/llvm-project/compiler-rt\lib\profile/InstrProfiling.h:12:
warning: In file included from ../../src/llvm-project/compiler-rt\lib\profile/InstrProfilingPort.h:65:
warning: In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\windows.h:171:
warning: In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\shared\windef.h:24:
warning: In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\shared\minwindef.h:182:
warning: C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\winnt.h(6370,20): error: use of undeclared identifier '__umulh'
warning:     *HighProduct = UnsignedMultiplyHigh(Multiplier, Multiplicand);
warning:                    ^
warning: C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\winnt.h(6236,30): note: expanded from macro 'UnsignedMultiplyHigh'
warning: #define UnsignedMultiplyHigh __umulh
warning: 1 error generated.

error: failed to run custom build command for `profiler_builtins v0.0.0 (D:\a\rust\rust\library\profiler_builtins)`


Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-std\release\build\profiler_builtins-5d2a280ac529e14b\build-script-build` (exit code: 1)
  TARGET = Some("aarch64-pc-windows-msvc")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-pc-windows-msvc")
  CC_aarch64-pc-windows-msvc = None
---
  CFLAGS_aarch64_pc_windows_msvc = None
  TARGET_CFLAGS = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("crt-static,fp,neon")
  DEBUG = Some("true")
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\GCDAProfiling.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\GCDAProfiling.c"
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(310,8): warning: '_open' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _open. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  fd = open(filename, O_RDWR | O_BINARY);
  cargo:warning=       ^
  cargo:warning=<command line>(4,14): note: expanded from here
  cargo:warning=#define open _open
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt_io.h(517,24): note: '_open' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_ _CRT_NONSTDC_DEPRECATE(_open) _CRT_INSECURE_DEPRECATE(_sopen_s)
  cargo:warning=                       ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(314,10): warning: '_open' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _open. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    fd = open(filename, O_RDWR | O_CREAT | O_EXCL | O_BINARY, 0644);
  cargo:warning=         ^
  cargo:warning=<command line>(4,14): note: expanded from here
  cargo:warning=#define open _open
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt_io.h(517,24): note: '_open' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_ _CRT_NONSTDC_DEPRECATE(_open) _CRT_INSECURE_DEPRECATE(_sopen_s)
  cargo:warning=                       ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(320,12): warning: '_open' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _open. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=      fd = open(filename, O_RDWR | O_CREAT | O_EXCL | O_BINARY, 0644);
  cargo:warning=           ^
  cargo:warning=<command line>(4,14): note: expanded from here
  cargo:warning=#define open _open
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt_io.h(517,24): note: '_open' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_ _CRT_NONSTDC_DEPRECATE(_open) _CRT_INSECURE_DEPRECATE(_sopen_s)
  cargo:warning=                       ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(326,14): warning: '_open' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _open. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=        fd = open(filename, O_RDWR | O_BINARY);
  cargo:warning=             ^
  cargo:warning=<command line>(4,14): note: expanded from here
  cargo:warning=#define open _open
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt_io.h(517,24): note: '_open' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_ _CRT_NONSTDC_DEPRECATE(_open) _CRT_INSECURE_DEPRECATE(_sopen_s)
  cargo:warning=                       ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(331,19): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=                  strerror(errnum));
  cargo:warning=                  ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(343,17): warning: '_fdopen' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fdopen. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  output_file = fdopen(fd, mode);
  cargo:warning=                ^
  cargo:warning=<command line>(5,16): note: expanded from here
  cargo:warning=#define fdopen _fdopen
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2431,28): note: '_fdopen' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fdopen)    _ACRTIMP FILE* __cdecl fdopen(_In_ int _FileHandle, _In_z_ char const* _Format);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=6 warnings generated.
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfiling.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfiling.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingBuffer.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingBuffer.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingFile.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingFile.c"
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(109,15): warning: '_fileno' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fileno. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=              fileno(File));
  cargo:warning=              ^
  cargo:warning=<command line>(7,16): note: expanded from here
  cargo:warning=#define fileno _fileno
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2433,28): note: '_fileno' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fileno)    _ACRTIMP int   __cdecl fileno(_In_ FILE* _Stream);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(181,17): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  BufferSzStr = getenv("LLVM_VP_BUFFER_SIZE");
  cargo:warning=                ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(196,14): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=             strerror(errno));
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(204,14): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=             strerror(errno));
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(225,25): warning: '_fileno' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fileno. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=                        fileno(ProfileFile), 0);
  cargo:warning=                        ^
  cargo:warning=<command line>(7,16): note: expanded from here
  cargo:warning=#define fileno _fileno
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2433,28): note: '_fileno' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fileno)    _ACRTIMP int   __cdecl fileno(_In_ FILE* _Stream);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(228,14): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=             strerror(errno));
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(272,9): warning: '_fileno' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fileno. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  (void)COMPILER_RT_FTRUNCATE(ProfileFile,
  cargo:warning=        ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile/InstrProfilingPort.h(23,44): note: expanded from macro 'COMPILER_RT_FTRUNCATE'
  cargo:warning=#define COMPILER_RT_FTRUNCATE(f,l) _chsize(_fileno(f),l)
  cargo:warning=                                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2433,28): note: '_fileno' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fileno)    _ACRTIMP int   __cdecl fileno(_In_ FILE* _Stream);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(286,5): warning: 'strncpy' is deprecated: This function or variable may be unsafe. Consider using strncpy_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    strncpy(Copy, Filename, Length + 1);
  cargo:warning=    ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(334,1): note: 'strncpy' has been explicitly marked deprecated here
  cargo:warning=__DEFINE_CPP_OVERLOAD_STANDARD_NFUNC_0_2_EX(
  cargo:warning=^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(1935,17): note: expanded from macro '__DEFINE_CPP_OVERLOAD_STANDARD_NFUNC_0_2_EX'
  cargo:warning=                _CRT_INSECURE_DEPRECATE(_SecureFuncName) _DeclSpec _ReturnType __cdecl _FuncName(_SalAttributeDst _DstType *_Dst, _TType1 _TArg1, _TType2 _TArg2);
  cargo:warning=                ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(314,29): warning: '_fileno' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fileno. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  if (rc || (!*MergeDone && COMPILER_RT_FTRUNCATE(ProfileFile, 0L)) ||
  cargo:warning=                            ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile/InstrProfilingPort.h(23,44): note: expanded from macro 'COMPILER_RT_FTRUNCATE'
  cargo:warning=#define COMPILER_RT_FTRUNCATE(f,l) _chsize(_fileno(f),l)
  cargo:warning=                                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2433,28): note: '_fileno' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fileno)    _ACRTIMP int   __cdecl fileno(_In_ FILE* _Stream);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(317,14): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=             strerror(errno));
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(331,10): warning: 'fopen' is deprecated: This function or variable may be unsafe. Consider using fopen_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  return fopen(OutputName, "ab");
  cargo:warning=         ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(212,20): note: 'fopen' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(fopen_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(372,16): warning: 'fopen' is deprecated: This function or variable may be unsafe. Consider using fopen_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  OutputFile = fopen(OutputName, "w");
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(212,20): note: 'fopen' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(fopen_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(405,21): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  int initialized = getenv(LPROF_INIT_ONCE_ENV) != NULL;
  cargo:warning=                    ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(423,10): warning: 'fopen' is deprecated: This function or variable may be unsafe. Consider using fopen_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  File = fopen(Filename, "w");
  cargo:warning=         ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(212,20): note: 'fopen' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(fopen_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(621,47): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    PROF_ERR("Failed to write profile: %s\n", strerror(errno));
  cargo:warning=                                              ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(631,59): warning: '_fileno' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fileno. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=      NULL, FileSize, PROT_READ | PROT_WRITE, MAP_SHARED, fileno(OutputFile), 0);
  cargo:warning=                                                          ^
  cargo:warning=<command line>(7,16): note: expanded from here
  cargo:warning=#define fileno _fileno
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2433,28): note: '_fileno' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fileno)    _ACRTIMP int   __cdecl fileno(_In_ FILE* _Stream);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(633,46): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    PROF_ERR("Unable to mmap profile: %s\n", strerror(errno));
  cargo:warning=                                             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(673,12): warning: 'fopen' is deprecated: This function or variable may be unsafe. Consider using fopen_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    File = fopen(Filename, "w+b");
  cargo:warning=           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(212,20): note: 'fopen' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(fopen_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(793,61): warning: '_getpid' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _getpid. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=          if (snprintf(PidChars, MAX_PID_SIZE, "%ld", (long)getpid()) <= 0) {
  cargo:warning=                                                            ^
  cargo:warning=<command line>(6,16): note: expanded from here
  cargo:warning=#define getpid _getpid
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\process.h(363,9): note: '_getpid' has been explicitly marked deprecated here
  cargo:warning=        _CRT_NONSTDC_DEPRECATE(_getpid)
  cargo:warning=        ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(809,35): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=        lprofCurFilename.TmpDir = getenv("TMPDIR");
  cargo:warning=                                  ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(875,9): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    if (getenv("LLVM_PROFILE_VERBOSE"))
  cargo:warning=        ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(879,9): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    if (getenv("LLVM_PROFILE_VERBOSE"))
  cargo:warning=        ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(957,29): warning: '_getpid' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _getpid. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=        int ProfilePoolId = getpid() % lprofCurFilename.MergePoolSize;
  cargo:warning=                            ^
  cargo:warning=<command line>(6,16): note: expanded from here
  cargo:warning=#define getpid _getpid
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\process.h(363,9): note: '_getpid' has been explicitly marked deprecated here
  cargo:warning=        _CRT_NONSTDC_DEPRECATE(_getpid)
  cargo:warning=        ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(976,26): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  const char *Filename = getenv("LLVM_PROFILE_FILE");
  cargo:warning=                         ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(1122,61): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    PROF_ERR("Failed to write file \"%s\": %s\n", Filename, strerror(errno));
  cargo:warning=                                                            ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(1184,61): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    PROF_ERR("Failed to write file \"%s\": %s\n", Filename, strerror(errno));
  cargo:warning=                                                            ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=26 warnings generated.
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingMerge.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingMerge.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingMergeFile.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingMergeFile.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingNameVar.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingNameVar.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformDarwin.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformDarwin.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformFuchsia.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformFuchsia.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformLinux.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformLinux.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformOther.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformOther.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformWindows.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformWindows.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingRuntime.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingRuntime.cpp"
  cargo:warning=In file included from ../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingRuntime.cpp:11:
  cargo:warning=In file included from ../../src/llvm-project/compiler-rt\lib\profile/InstrProfiling.h:12:
  cargo:warning=In file included from ../../src/llvm-project/compiler-rt\lib\profile/InstrProfilingPort.h:65:
  cargo:warning=In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\windows.h:171:
  cargo:warning=In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\shared\windef.h:24:
  cargo:warning=In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\shared\minwindef.h:182:
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\winnt.h(6370,20): error: use of undeclared identifier '__umulh'
  cargo:warning=    *HighProduct = UnsignedMultiplyHigh(Multiplier, Multiplicand);
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\winnt.h(6236,30): note: expanded from macro 'UnsignedMultiplyHigh'
  cargo:warning=#define UnsignedMultiplyHigh __umulh
  cargo:warning=                             ^
  cargo:warning=1 error generated.

  --- stderr



  error occurred: Command "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingRuntime.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingRuntime.cpp" with args "clang-cl.exe" did not execute successfully (status code exit code: 1).

warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:31:19
