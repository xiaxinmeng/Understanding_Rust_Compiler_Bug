plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:023bee34
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:26:37]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:26:42] error: failed to run custom build command for `rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)`
[00:26:42] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/rustc_llvm-42df884059483cad/build-script-build` (exit code: 101)
[00:26:42] --- stdout
[00:26:42] cargo:rerun-if-changed=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/bin/llvm-config
[00:26:42] cargo:rerun-if-env-changed=LLVM_CONFIG
[00:26:42] cargo:rustc-cfg=llvm_component="asmparser"
[00:26:42] cargo:rustc-cfg=llvm_component="bitreader"
[00:26:42] cargo:rustc-cfg=llvm_component="bitwriter"
[00:26:42] cargo:rustc-cfg=llvm_component="instrumentation"
[00:26:42] cargo:rustc-cfg=llvm_component="interpreter"
[00:26:42] cargo:rustc-cfg=llvm_component="ipo"
[00:26:42] cargo:rustc-cfg=llvm_component="jsbackend"
[00:26:42] cargo:rustc-cfg=llvm_component="linker"
[00:26:42] cargo:rustc-cfg=llvm_component="lto"
[00:26:42] cargo:rustc-cfg=llvm_component="mcjit"
[00:26:42] cargo:rerun-if-changed-env=LLVM_RUSTLLVM
[00:26:42] cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
[00:26:42] cargo:rerun-if-changed=../rustllvm/Linker.cpp
[00:26:42] cargo:rerun-if-changed=../rustllvm/llvm-rebuild-trigger
[00:26:42] cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
[00:26:42] cargo:rerun-if-changed=../rustllvm/rustllvm.h
[00:26:42] cargo:rerun-if-changed=../rustllvm/README
[00:26:42] cargo:rerun-if-changed=../rustllvm/.editorconfig
[00:26:42] cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
[00:26:42] OPT_LEVEL = Some("2")
[00:26:42] TARGET = Some("x86_64-unknown-linux-gnu")
[00:26:42] HOST = Some("x86_64-unknown-linux-gnu")
[00:26:42] TARGET = Some("x86_64-unknown-linux-gnu")
[00:26:42] TARGET = Some("x86_64-unknown-linux-gnu")
[00:26:42] TARGET = Some("x86_64-unknown-linux-gnu")
[00:26:42] HOST = Some("x86_64-unknown-linux-gnu")
[00:26:42] CXX_x86_64-unknown-linux-gnu = Some("sccache clang++")
[00:26:42] HOST = Some("x86_64-unknown-linux-gnu")
[00:26:42] CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu")
[00:26:42] DEBUG = Some("false")
[00:26:42] DEBUG = Some("false")
[00:26:42] running: "sccache" "clang++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "--target=x86_64-unknown-linux-gnu" "-I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "-fPIC" "-fvisibility-inlines-hidden" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wmissing-field-initializers" "-pedantic" "-Wno-long-long" "-Wcovered-switch-default" "-Wnon-virtual-dtor" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-Wstring-conversion" "-Werror=date-time" "-std=c++11" "-ffunction-sections" "-fdata-sections" "-O3" "-DNDEBUG" "-fno-exceptions" "-fno-rtti" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_JSBACKEND" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_RUSTLLVM" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-d574027f8db565f3/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp"
[00:26:42] cargo:warning=../rustllvm/PassWrapper.cpp:1133:50: error: no member named 'keys' in 'llvm::StringMap<std::map<unsigned long, unsigned int, std::less<unsigned long>, std::allocator<std::pair<const unsigned long, unsigned int> > >, llvm::MallocAllocator>'
[00:26:42] cargo:warning=    for (const auto imported_module_id : imports.keys()) {
[00:26:42] cargo:warning=                                         ~~~~~~~ ^
[00:26:42] cargo:warning=1 error generated.
[00:26:42] 
[00:26:42] --- stderr
[00:26:42] thread 'main' panicked at '
[00:26:42] 
[00:26:42] 
[00:26:42] Internal error occurred: Command "sccache" "clang++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "--target=x86_64-unknown-linux-gnu" "-I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "-fPIC" "-fvisibility-inlines-hidden" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wmissing-field-initializers" "-pedantic" "-Wno-long-long" "-Wcovered-switch-default" "-Wnon-virtual-dtor" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-Wstring-conversion" "-Werror=date-time" "-std=c++11" "-ffunction-sections" "-fdata-sections" "-O3" "-DNDEBUG" "-fno-exceptions" "-fno-rtti" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_JSBACKEND" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_RUSTLLVM" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-d574027f8db565f3/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp" with args "clang++" did not execute successfully (status code exit code: 1).
[00:26:42] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.17/src/lib.rs:2180:5
[00:26:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:26:42] 
[00:26:42] 
[00:26:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc emscripten" "--message-format" "json"
[00:26:42] expected success, got: exit code: 101
[00:26:42] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm

[00:26:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
travis_time:end:0d938ba0:start=1531416772331973760,finish=1531416772338225524,duration=6251764
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f685200
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25811a6c
$ dmesg | grep -i kill
