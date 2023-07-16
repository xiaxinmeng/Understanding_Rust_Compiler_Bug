plain
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-book.tar.gz &&         curl -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/88cdde350fd3a90c93f3bac8b4f168f105d28060.tar.gz
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/e3719fc78ff4a21dfd13cfcc9e2ca42cb5de29f4.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/lldb src/clang &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/lldb src/clang
[00:00:00] Cleared directory 'src/clang'
[00:00:00] Cleared directory 'src/dlmalloc'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
---
[00:00:00] Cleared directory 'src/tools/miri'
[00:00:00] Cleared directory 'src/tools/rls'
[00:00:00] Cleared directory 'src/tools/rust-installer'
[00:00:00] Cleared directory 'src/tools/rustfmt'
[00:00:00] Submodule 'src/clang' (https://github.com/rust-lang-nursery/clang/) registered for path 'src/clang'
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/jemalloc' (https://github.com/rust-lang/jemalloc.git) registered for path 'src/jemalloc'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:00] Submodule 'src/lldb' (https://github.com/rust-lang-nursery/lldb/) registered for path 'src/lldb'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
[00:00:00] Submodule 'src/tools/miri' (https://github.com/solson/miri.git) registered for path 'src/tools/miri'
---
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rustfmt'...
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/liblibc'...
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/tools/cargo'...
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/tools/lld'...
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/lldb'...
[00:00:54] Submodule path 'src/clang': checked out '6fda594059bd48b6b2ddcb34eda0a278aee2214e'
[00:00:54] Submodule path 'src/doc/nomicon': checked out '790e96b87f4b5817cac310e73a524d25c3d076d8'
[00:00:54] Submodule path 'src/doc/reference': checked out '219e261ddb833a5683627b0a9be87a0f4486abb9'
[00:00:54] Submodule path 'src/jemalloc': checked out '1f5a28755e301ac581e2048011e4e0ff3da482ef'
[00:00:54] Submodule path 'src/libbacktrace': checked out 'f4d02bbdbf8a2c5a31f0801dfef597a86caad9e3'
---
[00:00:57] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/libm'...
[00:00:58] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '40151c4c1cf77e593e3654e66e25ea423116aaae'
[00:00:58] Submodule path 'src/libcompiler_builtins/libm': checked out '96e36ea2620f9fbbaa46a01694a2fa3ef6c2fb7e'
[00:00:58] Submodule path 'src/liblibc': checked out 'b6d23ed45d72918239c0bfac11dc547895e59b81'
[00:00:58] Submodule path 'src/lldb': checked out '3dbe998969d457c5cef245f61b48bdaed0f5c059'
[00:00:59] Submodule path 'src/tools/cargo': checked out '2cd36b4ed1aef1ae39a30783e006411d1a4218ac'
[00:00:59] Submodule path 'src/tools/clippy': checked out 'b0dabce47803c18b935ec5390de69e04ad5304c2'
[00:00:59] Submodule path 'src/tools/lld': checked out '8214ccf861d538671b0a1436dbf4538dc4a64d09'
[00:00:59] Submodule path 'src/tools/miri': checked out 'e6f1e15676c26fdc7c4713647fe007b26f361a8e'
---
[00:18:45] [TIMING] Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 722.297
[00:18:45] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-gnu
[00:18:45] running: "cmake" "/checkout/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DLLVM_ENABLE_PROJECTS=clang;lldb" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:18:46] -- The CXX compiler identification is Clang 6.0.0
[00:18:46] -- The ASM compiler identification is Clang
[00:18:46] -- Found assembler: /usr/local/bin/sccache
[00:18:46] -- Check for working C compiler: /usr/local/bin/sccache
---
[00:18:59] -- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
[00:18:59] -- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG - Success
[00:18:59] -- Performing Test CXX_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
[00:19:00] -- Performing Test CXX_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG - Success
[00:19:00] -- Performing Test CXX_SUPPORTS_CLASS_MEMACCESS_FLAG
[00:19:00] -- Performing Test CXX_SUPPORTS_CLASS_MEMACCESS_FLAG - Failed
[00:19:00] -- Performing Test CXX_WONT_WARN_ON_FINAL_NONVIRTUALDTOR - Success
[00:19:00] -- Performing Test C_SUPPORTS_DELETE_NON_VIRTUAL_DTOR_FLAG
[00:19:00] -- Performing Test C_SUPPORTS_DELETE_NON_VIRTUAL_DTOR_FLAG - Success
[00:19:00] -- Performing Test CXX_SUPPORTS_DELETE_NON_VIRTUAL_DTOR_FLAG
---
[00:19:04] -- Targeting WebAssembly
[00:19:04] -- Targeting RISCV
[00:19:04] -- Looking for sys/resource.h
[00:19:04] -- Looking for sys/resource.h - found
[00:19:04] -- Clang version: 7.0.0
[00:19:04] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG
[00:19:04] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG - Success
[00:19:06] -- Found PythonLibs: /rustroot/lib/libpython2.7.a (found version "2.7.12") 
[00:19:06] -- Performing Test CXX_SUPPORTS_NO_DEPRECATED_DECLARATIONS
[00:19:06] -- Performing Test CXX_SUPPORTS_NO_DEPRECATED_DECLARATIONS - Success
[00:19:06] -- Performing Test CXX_SUPPORTS_NO_UNKNOWN_PRAGMAS
[00:19:06] -- Performing Test CXX_SUPPORTS_NO_UNKNOWN_PRAGMAS - Success
[00:19:06] -- Performing Test CXX_SUPPORTS_NO_STRICT_ALIASING
[00:19:06] -- Performing Test CXX_SUPPORTS_NO_STRICT_ALIASING - Success
[00:19:06] -- Performing Test CXX_SUPPORTS_NO_DEPRECATED_REGISTER
[00:19:06] -- Performing Test CXX_SUPPORTS_NO_DEPRECATED_REGISTER - Success
[00:19:06] -- Performing Test CXX_SUPPORTS_NO_VLA_EXTENSION
[00:19:07] -- Performing Test CXX_SUPPORTS_NO_VLA_EXTENSION - Success
[00:19:07] -- Performing Test CXX_SUPPORTS_NO_GNU_ANONYMOUS_STRUCT
[00:19:07] -- Performing Test CXX_SUPPORTS_NO_GNU_ANONYMOUS_STRUCT - Success
[00:19:07] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES
[00:19:07] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES - Success
[00:19:07] -- LLDB version: 7.0.0
[00:19:07] -- Could NOT find LibXml2 (missing:  LIBXML2_LIBRARIES LIBXML2_INCLUDE_DIR) 
[00:19:07] CMake Error at /rustroot/share/cmake-3.6/Modules/FindPackageHandleStandardArgs.cmake:148 (message):
[00:19:07]   Could NOT find Curses (missing: CURSES_LIBRARY CURSES_INCLUDE_PATH)
[00:19:07] Call Stack (most recent call first):
[00:19:07]   /rustroot/share/cmake-3.6/Modules/FindPackageHandleStandardArgs.cmake:388 (_FPHSA_FAILURE_MESSAGE)
[00:19:07]   /rustroot/share/cmake-3.6/Modules/FindCurses.cmake:206 (FIND_PACKAGE_HANDLE_STANDARD_ARGS)
[00:19:07]   /checkout/src/lldb/cmake/modules/LLDBConfig.cmake:378 (find_package)
[00:19:07]   /checkout/src/lldb/CMakeLists.txt:11 (include)
[00:19:07] 
[00:19:07] 
[00:19:08] -- Configuring incomplete, errors occurred!
[00:19:08] See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
[00:19:08] See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
[00:19:08] command did not execute successfully, got: exit code: 1
[00:19:08] 
[00:19:08] 
[00:19:08] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.31/src/lib.rs:643:5
[00:19:08]  finished in 23.006
[00:19:08] travis_fold:end:llvm

[00:19:08] travis_time:end:llvm:start=1533237350540057808,finish=1533237373546738304,duration=23006680496
---
travis_time:end:1009a448:start=1533237374190748353,finish=1533237374196163784,duration=5415431
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d097c28
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01832311
travis_time:start:01832311
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1602fe34
$ dmesg | grep -i kill
