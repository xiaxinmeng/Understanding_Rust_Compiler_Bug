plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:168a66ac
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:47:07]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:47:12] error: failed to run custom build command for `rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)`
[00:47:12] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/release/build/rustc_llvm-3168d95e0c349830/build-script-build` (exit code: 101)
[00:47:12] --- stdout
[00:47:12] cargo:rerun-if-changed=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/bin/llvm-config
[00:47:12] cargo:rerun-if-env-changed=LLVM_CONFIG
[00:47:12] cargo:rustc-cfg=llvm_component="asmparser"
[00:47:12] cargo:rustc-cfg=llvm_component="bitreader"
[00:47:12] cargo:rustc-cfg=llvm_component="bitwriter"
[00:47:12] cargo:rustc-cfg=llvm_component="instrumentation"
[00:47:12] cargo:rustc-cfg=llvm_component="interpreter"
[00:47:12] cargo:rustc-cfg=llvm_component="ipo"
[00:47:12] cargo:rustc-cfg=llvm_component="jsbackend"
[00:47:12] cargo:rustc-cfg=llvm_component="linker"
[00:47:12] cargo:rustc-cfg=llvm_component="lto"
[00:47:12] cargo:rustc-cfg=llvm_component="mcjit"
[00:47:12] cargo:rerun-if-changed-env=LLVM_RUSTLLVM
[00:47:12] cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
[00:47:12] cargo:rerun-if-changed=../rustllvm/Linker.cpp
[00:47:12] cargo:rerun-if-changed=../rustllvm/llvm-rebuild-trigger
[00:47:12] cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
[00:47:12] cargo:rerun-if-changed=../rustllvm/rustllvm.h
[00:47:12] cargo:rerun-if-changed=../rustllvm/README
[00:47:12] cargo:rerun-if-changed=../rustllvm/.editorconfig
[00:47:12] cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
[00:47:12] OPT_LEVEL = Some("2")
[00:47:12] TARGET = Some("x86_64-unknown-linux-gnu")
[00:47:12] HOST = Some("x86_64-unknown-linux-gnu")
[00:47:12] TARGET = Some("x86_64-unknown-linux-gnu")
[00:47:12] TARGET = Some("x86_64-unknown-linux-gnu")
[00:47:12] TARGET = Some("x86_64-unknown-linux-gnu")
[00:47:12] HOST = Some("x86_64-unknown-linux-gnu")
[00:47:12] CXX_x86_64-unknown-linux-gnu = Some("sccache clang++")
[00:47:12] HOST = Some("x86_64-unknown-linux-gnu")
[00:47:12] CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu")
[00:47:12] DEBUG = Some("false")
[00:47:12] DEBUG = Some("false")
[00:47:12] running: "sccache" "clang++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "--target=x86_64-unknown-linux-gnu" "-I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "-fPIC" "-fvisibility-inlines-hidden" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wmissing-field-initializers" "-pedantic" "-Wno-long-long" "-Wcovered-switch-default" "-Wnon-virtual-dtor" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-Wstring-conversion" "-Werror=date-time" "-std=c++11" "-ffunction-sections" "-fdata-sections" "-O3" "-DNDEBUG" "-fno-exceptions" "-fno-rtti" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_JSBACKEND" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_RUSTLLVM" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-561a3eb881c7aaef/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp"
[00:47:12] cargo:warning=../rustllvm/PassWrapper.cpp:876:21: error: use of undeclared identifier 'readModuleSummaryIndex'
[00:47:12] cargo:warning=    if (Error Err = readModuleSummaryIndex(mem_buffer, Ret->Index, i)) {
[00:47:12] cargo:warning=                    ^
[00:47:12] cargo:warning=../rustllvm/PassWrapper.cpp:919:18: error: no member named 'SummaryList' in 'std::vector<std::unique_ptr<llvm::GlobalValueSummary, std::default_delete<llvm::GlobalValueSummary> >, std::allocator<std::unique_ptr<llvm::GlobalValueSummary, std::default_delete<llvm::GlobalValueSummary> > > >'
[00:47:12] cargo:warning=    if (I.second.SummaryList.size() > 1)
[00:47:12] cargo:warning=        ~~~~~~~~ ^
[00:47:12] cargo:warning=../rustllvm/PassWrapper.cpp:920:70: error: no member named 'SummaryList' in 'std::vector<std::unique_ptr<llvm::GlobalValueSummary, std::default_delete<llvm::GlobalValueSummary> >, std::allocator<std::unique_ptr<llvm::GlobalValueSummary, std::default_delete<llvm::GlobalValueSummary> > > >'
[00:47:12] cargo:warning=      PrevailingCopy[I.first] = getFirstDefinitionForLinker(I.second.SummaryList);
[00:47:12] cargo:warning=                                                            ~~~~~~~~ ^
[00:47:12] cargo:warning=../rustllvm/PassWrapper.cpp:942:33: error: no member named 'SummaryList' in 'std::vector<std::unique_ptr<llvm::GlobalValueSummary, std::default_delete<llvm::GlobalValueSummary> >, std::allocator<std::unique_ptr<llvm::GlobalValueSummary, std::default_delete<llvm::GlobalValueSummary> > > >'
[00:47:12] cargo:warning=    for (auto &GVS: List.second.SummaryList) {
[00:47:12] cargo:warning=                    ~~~~~~~~~~~ ^
[00:47:12] cargo:warning=4 errors generated.
[00:47:12] 
[00:47:12] --- stderr
[00:47:12] thread 'main' panicked at '
[00:47:12] 
[00:47:12] 
[00:47:12] Internal error occurred: Command "sccache" "clang++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "--target=x86_64-unknown-linux-gnu" "-I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "-fPIC" "-fvisibility-inlines-hidden" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wmissing-field-initializers" "-pedantic" "-Wno-long-long" "-Wcovered-switch-default" "-Wnon-virtual-dtor" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-Wstring-conversion" "-Werror=date-time" "-std=c++11" "-ffunction-sections" "-fdata-sections" "-O3" "-DNDEBUG" "-fno-exceptions" "-fno-rtti" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_JSBACKEND" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_RUSTLLVM" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-561a3eb881c7aaef/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp" with args "clang++" did not execute successfully (status code exit code: 1).
[00:47:12] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.22/src/lib.rs:2242:5
[00:47:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:12] 
[00:47:12] 
[00:47:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " emscripten" "--message-format" "json" "--" "-Clink-arg=-fuse-ld=lld" "-Clink-arg=-flto=thin" "-Clink-arg=-O2" "-Clink-arg=-Wl,--thinlto-jobs=4"
[00:47:12] expected success, got: exit code: 101
[00:47:12] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:47:12] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm

---
travis_time:end:0f2d57b7:start=1535792452837623043,finish=1535792452844121210,duration=6498167
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d43064a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a2266b8
travis_time:start:0a2266b8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1669067a
$ dmesg | grep -i kill
