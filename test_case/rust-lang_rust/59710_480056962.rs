plain
travis_time:end:21578cd6:start=1554409316947974927,finish=1554409317826173340,duration=878198413
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:23:24]    Compiling memmap v0.6.2
[00:23:30] error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
[00:23:30] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/release/build/rustc_llvm-ce6334ff013f1c9d/build-script-build` (exit code: 101)
[00:23:30] --- stdout
[00:23:30] cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
[00:23:30] cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
[00:23:30] cargo:rerun-if-changed=/usr/lib/llvm-6.0/bin/llvm-config
[00:23:30] cargo:rerun-if-env-changed=LLVM_CONFIG
[00:23:30] cargo:rustc-cfg=llvm_component="aarch64"
[00:23:30] cargo:rustc-cfg=llvm_component="amdgpu"
[00:23:30] cargo:rustc-cfg=llvm_component="arm"
[00:23:30] cargo:rustc-cfg=llvm_component="asmparser"
[00:23:30] cargo:rustc-cfg=llvm_component="bitreader"
[00:23:30] cargo:rustc-cfg=llvm_component="bitwriter"
[00:23:30] cargo:rustc-cfg=llvm_component="hexagon"
[00:23:30] cargo:rustc-cfg=llvm_component="instrumentation"
[00:23:30] cargo:rustc-cfg=llvm_component="interpreter"
[00:23:30] cargo:rustc-cfg=llvm_component="ipo"
[00:23:30] cargo:rustc-cfg=llvm_component="linker"
[00:23:30] cargo:rustc-cfg=llvm_component="lto"
[00:23:30] cargo:rustc-cfg=llvm_component="mcjit"
[00:23:30] cargo:rustc-cfg=llvm_component="mips"
[00:23:30] cargo:rustc-cfg=llvm_component="msp430"
[00:23:30] cargo:rustc-cfg=llvm_component="nvptx"
[00:23:30] cargo:rustc-cfg=llvm_component="powerpc"
[00:23:30] cargo:rustc-cfg=llvm_component="sparc"
[00:23:30] cargo:rustc-cfg=llvm_component="systemz"
[00:23:30] cargo:rustc-cfg=llvm_component="x86"
[00:23:30] cargo:rerun-if-changed-env=LLVM_RUSTLLVM
[00:23:30] cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
[00:23:30] cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
[00:23:30] cargo:rerun-if-changed=../rustllvm/README
[00:23:30] cargo:rerun-if-changed=../rustllvm/Linker.cpp
[00:23:30] cargo:rerun-if-changed=../rustllvm/.editorconfig
[00:23:30] cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
[00:23:30] cargo:rerun-if-changed=../rustllvm/rustllvm.h
[00:23:30] OPT_LEVEL = Some("2")
[00:23:30] HOST = Some("x86_64-unknown-linux-gnu")
[00:23:30] HOST = Some("x86_64-unknown-linux-gnu")
[00:23:30] CXX_x86_64-unknown-linux-gnu = Some("sccache c++")
[00:23:30] CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
[00:23:30] DEBUG = Some("false")
[00:23:30] running: "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-m64" "-I/usr/lib/llvm-6.0/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_X86" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-7835821f8c6f3f86/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp"
[00:23:30] exit code: 0
[00:23:30] running: "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-m64" "-I/usr/lib/llvm-6.0/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_X86" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-7835821f8c6f3f86/out/../rustllvm/RustWrapper.o" "-c" "../rustllvm/RustWrapper.cpp"
[00:23:30] cargo:warning=../rustllvm/RustWrapper.cpp: In function 'LLVMOpaqueValue* LLVMRustGetOrInsertFunction(LLVMModuleRef, const char*, LLVMTypeRef)':
[00:23:30] cargo:warning=../rustllvm/RustWrapper.cpp:120:78: error: request for member 'getCallee' in 'llvm::unwrap(M)->llvm::Module::getOrInsertFunction(llvm::StringRef(Name), llvm::unwrap<llvm::FunctionType>(FunctionTy))', which is of pointer type 'llvm::Constant*' (maybe you meant to use '->' ?)
[00:23:30] cargo:warning=       unwrap(M)->getOrInsertFunction(Name, unwrap<FunctionType>(FunctionTy)).getCallee());
[00:23:30] cargo:warning=                                                                              ^
[00:23:30] 
[00:23:30] --- stderr
[00:23:30] thread 'main' panicked at '
[00:23:30] 
[00:23:30] 
[00:23:30] Internal error occurred: Command "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-m64" "-I/usr/lib/llvm-6.0/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_X86" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-7835821f8c6f3f86/out/../rustllvm/RustWrapper.o" "-c" "../rustllvm/RustWrapper.cpp" with args "c++" did not execute successfully (status code exit code: 1).
[00:23:30] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.28/src/lib.rs:2314:5
[00:23:30] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:23:30] 
[00:23:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json"
---
travis_time:end:22632a20:start=1554410740065549994,finish=1554410740069981752,duration=4431758
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f74898c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:37f0f125
travis_time:start:37f0f125
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:030af7d6
$ dmesg | grep -i kill
