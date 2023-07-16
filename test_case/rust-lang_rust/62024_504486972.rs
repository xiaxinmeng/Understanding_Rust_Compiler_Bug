plain
[00:00:06] Submodule path 'src/tools/miri': checked out '285e9a65cd61520a4da1f912d1b41e523cd3a5e4'
[00:00:07] Submodule path 'src/tools/rls': checked out '3e519650cea91a4b785cd773a3e5965553f74249'
[00:00:07] Submodule path 'src/tools/rust-installer': checked out 'ccdc47b657a7600cbd0c2858eb52a8d712cfce18'
[00:00:07] curl: (22) The requested URL returned error: 500 Internal Server Error
[00:00:07] +[[ 1 -lt 5 ]]
[00:00:07] +sleep 1
[00:00:07] +break
[00:00:07] +wait
[00:00:07] +rm download-src-llvm-emscripten.tar.gz
[00:00:07] +rm download-src-llvm-emscripten.tar.gz
[00:00:08] +(( n++ ))
[00:00:08] +echo 'Command failed. Attempt 2/5:'
[00:00:08] Command failed. Attempt 2/5:
[00:00:08] +sh -c 'rm -f download-src-llvm-project.tar.gz &&         curl -f -sSL -o download-src-llvm-project.tar.gz https://github.com/rust-lang/llvm-project/archive/788592fb2740d560d0614a77035b319b9d07aa38.tar.gz'
[00:00:15] curl: (22) The requested URL returned error: 500 Internal Server Error
[00:00:15] curl: (22) The requested URL returned error: 500 Internal Server Error
[00:00:15] +[[ 2 -lt 5 ]]
[00:00:15] +sleep 2
[00:00:17] +(( n++ ))
[00:00:17] +echo 'Command failed. Attempt 3/5:'
[00:00:17] Command failed. Attempt 3/5:
[00:00:17] +sh -c 'rm -f download-src-llvm-project.tar.gz &&         curl -f -sSL -o download-src-llvm-project.tar.gz https://github.com/rust-lang/llvm-project/archive/788592fb2740d560d0614a77035b319b9d07aa38.tar.gz'
[00:00:24] curl: (22) The requested URL returned error: 500 Internal Server Error
[00:00:24] curl: (22) The requested URL returned error: 500 Internal Server Error
[00:00:24] +[[ 3 -lt 5 ]]
[00:00:24] +sleep 3
[00:00:27] +(( n++ ))
[00:00:27] +echo 'Command failed. Attempt 4/5:'
[00:00:27] Command failed. Attempt 4/5:
[00:00:27] +sh -c 'rm -f download-src-llvm-project.tar.gz &&         curl -f -sSL -o download-src-llvm-project.tar.gz https://github.com/rust-lang/llvm-project/archive/788592fb2740d560d0614a77035b319b9d07aa38.tar.gz'
[00:00:34] curl: (22) The requested URL returned error: 500 Internal Server Error
[00:00:34] curl: (22) The requested URL returned error: 500 Internal Server Error
[00:00:34] +[[ 4 -lt 5 ]]
[00:00:34] +sleep 4
[00:00:38] +(( n++ ))
[00:00:38] +echo 'Command failed. Attempt 5/5:'
[00:00:38] +true
[00:00:38] +sh -c 'rm -f download-src-llvm-project.tar.gz &&         curl -f -sSL -o download-src-llvm-project.tar.gz https://github.com/rust-lang/llvm-project/archive/788592fb2740d560d0614a77035b319b9d07aa38.tar.gz'
[00:00:45] curl: (22) The requested URL returned error: 500 Internal Server Error
[00:00:45] curl: (22) The requested URL returned error: 500 Internal Server Error
[00:00:45] +[[ 5 -lt 5 ]]
[00:00:45] +echo 'The command has failed after 5 attempts.'
[00:00:45] +return 1
[00:00:45] +travis_fold end init_repo
[00:00:45] +echo -en 'travis_fold:end:init_repo\r\033[0K'
[00:00:45] +travis_time_finish
[00:00:45] ++travis_nanoseconds
---
[00:20:02] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-gnu
[00:20:02] running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.37.0-nightly" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:20:02] CMake Error: The source directory "/checkout/src/llvm-project/llvm" does not exist.
[00:20:02] Specify --help for usage, or press the help button on the CMake GUI.
[00:20:02] command did not execute successfully, got: exit code: 1
[00:20:02] 
[00:20:02] 
[00:20:02] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
[00:20:02]  finished in 0.017
[00:20:02] travis_fold:end:llvm

[00:20:02] travis_time:end:llvm:start=1561134340079810214,finish=1561134340097373017,duration=17562803
---
travis_time:end:19423e0c:start=1561134340888722624,finish=1561134340896287622,duration=7564998
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c71bd6c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:125a7aaf
travis_time:start:125a7aaf
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00f3e055
$ dmesg | grep -i kill
