plain
[00:02:47]       Memory: 8 GB
[00:02:47]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:47]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:47]       SMC Version (system): 2.8f0
[00:02:47]       Serial Number (system): VMtPeVMSXnMA
[00:02:47] 
[00:02:47] hw.ncpu: 4
[00:02:47] hw.byteorder: 1234
[00:02:47] hw.memsize: 8589934592
---
[00:05:51] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for i686-apple-darwin
[00:05:51] running: "cmake" "/Users/travis/build/rust-lang/rust/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=i686" "-DLLVM_DEFAULT_TARGET_TRIPLE=i686-apple-darwin" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_BUILD_32_BITS=ON" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=/Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=/Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=i686-apple-darwin -stdlib=libc++" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=i686-apple-darwin -stdlib=libc++" "-DCMAKE_INSTALL_PREFIX=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:05:51] CMake Error: The source directory "/Users/travis/build/rust-lang/rust/src/llvm" does not appear to contain CMakeLists.txt.
[00:05:51] Specify --help for usage, or press the help button on the CMake GUI.
[00:05:51] command did not execute successfully, got: exit code: 1
[00:05:51] 
[00:05:51] 
[00:05:51] build script failed, must exit now', /Users/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.30/src/lib.rs:643:5
[00:05:51]  finished in 0.066
[00:05:51] travis_fold:end:llvm

[00:05:51] travis_time:end:llvm:start=1528148261527287000,finish=1528148261593856000,duration=66569000
[00:05:51] travis_time:end:llvm:start=1528148261527287000,finish=1528148261593856000,duration=66569000

[00:05:51] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:51] Build completed unsuccessfully in 0:03:03
[00:05:51] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1dccd9d5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:1cdc29b3
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25 19:20 ..
drwx------   2 travis  staff   68 Dec  6 11:00 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0067f5fc
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:0067f5fc:start=1528148262817992000,finish=1528148262836848000,duration=18856000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b89a414
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.4

Done. Your build exited with 1.
