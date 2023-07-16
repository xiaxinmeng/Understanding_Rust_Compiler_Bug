plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0d1461e0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli)
Collecting botocore==1.11.5 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/9e/ef/5d6d9995946379e018bca8353271a3d7068daf5892f4bcdddd59faded358/botocore-1.11.5-py2.py3-none-any.whl (4.6MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/50/09/c53398e0005b11f7ffb27b7aa720c617aba53be4fb4f4f3f06b9b5c60f28/docutils-0.14-py2-none-any.whl (543kB)
Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
---
[00:04:28] configure: target.x86_64-unknown-linux-gnu.linker := clang
[00:04:28] configure: target.x86_64-unknown-linux-gnu.ar := /rustroot/bin/llvm-ar
[00:04:28] configure: target.x86_64-unknown-linux-gnu.ranlib := /rustroot/bin/llvm-ranlib
[00:04:28] configure: llvm.thin-lto        := True
[00:04:28] configure: rust.remap-debuginfo := True
[00:04:28] configure: build.submodules     := False
[00:04:28] configure: build.compiler-docs  := True
[00:04:28] configure: build.profiler       := True
[00:04:28] configure: build.locked-deps    := True
---

[00:20:39] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-gnu
[00:20:39] running: "cmake" "/checkout/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/449328a8a8cf8e5b49cef91f38306d10036aeead" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/449328a8a8cf8e5b49cef91f38306d10036aeead" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:20:39] -- The CXX compiler identification is Clang 6.0.0
[00:20:39] -- The ASM compiler identification is Clang
[00:20:39] -- Found assembler: /usr/local/bin/sccache
[00:20:39] -- Check for working C compiler: /usr/local/bin/sccache
---

[01:16:44] travis_fold:start:llvm
travis_time:start:llvm
Building Emscripten LLVM for x86_64-unknown-linux-gnu
[01:16:44] running: "cmake" "/checkout/src/llvm-emscripten" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=JSBackend" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/449328a8a8cf8e5b49cef91f38306d10036aeead" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/449328a8a8cf8e5b49cef91f38306d10036aeead" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten" "-DCMAKE_BUILD_TYPE=Release"
[01:16:44] -- The CXX compiler identification is Clang 6.0.0
[01:16:44] -- The ASM compiler identification is Clang
[01:16:44] -- Found assembler: /usr/local/bin/sccache
[01:16:44] -- Check for working C compiler: /usr/local/bin/sccache
---

[01:38:40] travis_fold:start:lld
travis_time:start:lld
Building LLD for x86_64-unknown-linux-gnu
[01:38:40] running: "cmake" "/checkout/src/tools/lld" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/449328a8a8cf8e5b49cef91f38306d10036aeead" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/449328a8a8cf8e5b49cef91f38306d10036aeead -static-libstdc++" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DLLVM_CONFIG_PATH=/checkout/obj/build/bootstrap/debug/llvm-config-wrapper" "-DLLVM_INCLUDE_TESTS=OFF" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/lld" "-DCMAKE_BUILD_TYPE=Release"
[01:38:40] -- The CXX compiler identification is Clang 6.0.0
[01:38:40] -- Check for working C compiler: /usr/local/bin/sccache
[01:38:40] -- Check for working C compiler: /usr/local/bin/sccache -- works
[01:38:40] -- Detecting C compiler ABI info
