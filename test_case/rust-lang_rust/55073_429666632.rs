plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0b85949e
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---

[00:46:59] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, emscripten)
[00:46:59]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:46:59]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
 AttributeSet get(LLVMContext &C, unsigned Index, const AttrBuilder &B);
[00:47:03] cargo:warning=                      ^
[00:47:03] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Attributes.h:224:23: note: candidate function not viable: requires 2 arguments, but 4 were provided
[00:47:03] cargo:warning=  static AttributeSet get(LLVMContext &C,
[00:47:03] cargo:warning=                      ^
[00:47:03] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Attributes.h:226:23: note: candidate function not viable: requires 2 arguments, but 4 were provided
[00:47:03] cargo:warning=  static AttributeSet get(LLVMContext &C,
[00:47:03] cargo:warning=                      ^
[00:47:03] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Attributes.h:244:23: note: candidate function not viable: requires 2 arguments, but 4 were provided
[00:47:03] cargo:warning=  static AttributeSet get(LLVMContext &C, ArrayRef<AttributeSet> Attrs);
[00:47:03] cargo:warning=                      ^
[00:47:03] cargo:warning=6 errors generated.
[00:47:03] 
[00:47:03] --- stderr
[00:47:03] thread 'main' panicked at '
[00:47:03] 
[00:47:03] 
[00:47:03] Internal error occurred: Command "sccache" "clang++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "-fdebug-prefix-map=/checkout=/rustc/5c1be1a1d2e33344a26a3d02966b664d87997235" "--target=x86_64-unknown-linux-gnu" "-I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "-fdebug-prefix-map=/checkout=/rustc/llvm" "-fPIC" "-fvisibility-inlines-hidden" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wmissing-field-initializers" "-pedantic" "-Wno-long-long" "-Wcovered-switch-default" "-Wnon-virtual-dtor" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-Wstring-conversion" "-Werror=date-time" "-std=c++11" "-ffunction-sections" "-fdata-sections" "-O3" "-DNDEBUG" "-fno-exceptions" "-fno-rtti" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_JSBACKEND" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_RUSTLLVM" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-12f72c3c885a019d/out/../rustllvm/DemoteSimd.o" "-c" "../rustllvm/DemoteSimd.cpp" with args "clang++" did not execute successfully (status code exit code: 1).
[00:47:04] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.25/src/lib.rs:2260:5
[00:47:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:04] 
[00:47:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/li
