plain
[00:25:13]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:25:17] error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
[00:25:17] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/release/build/rustc_llvm-b767892f5ba00bd7/build-script-build` (exit code: 101)
[00:25:17] --- stdout
[00:25:17] cargo:rerun-if-changed=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/bin/llvm-config
[00:25:17] cargo:rerun-if-env-changed=LLVM_CONFIG
[00:25:17] cargo:rustc-cfg=llvm_component="asmparser"
[00:25:17] cargo:rustc-cfg=llvm_component="bitreader"
[00:25:17] cargo:rustc-cfg=llvm_component="bitwriter"
[00:25:17] cargo:rustc-cfg=llvm_component="instrumentation"
[00:25:17] cargo:rustc-cfg=llvm_component="interpreter"
[00:25:17] cargo:rustc-cfg=llvm_component="ipo"
[00:25:17] cargo:rustc-cfg=llvm_component="jsbackend"
[00:25:17] cargo:rustc-cfg=llvm_component="linker"
[00:25:17] cargo:rustc-cfg=llvm_component="lto"
[00:25:17] cargo:rustc-cfg=llvm_component="mcjit"
[00:25:17] cargo:rerun-if-changed-env=LLVM_RUSTLLVM
[00:25:17] cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
[00:25:17] cargo:rerun-if-changed=../rustllvm/Linker.cpp
[00:25:17] cargo:rerun-if-changed=../rustllvm/llvm-rebuild-trigger
[00:25:17] cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
[00:25:17] cargo:rerun-if-changed=../rustllvm/rustllvm.h
[00:25:17] cargo:rerun-if-changed=../rustllvm/README
[00:25:17] cargo:rerun-if-changed=../rustllvm/DemoteSimd.cpp
[00:25:17] cargo:rerun-if-changed=../rustllvm/.editorconfig
[00:25:17] cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
[00:25:17] OPT_LEVEL = Some("2")
[00:25:17] HOST = Some("x86_64-unknown-linux-gnu")
[00:25:17] HOST = Some("x86_64-unknown-linux-gnu")
[00:25:17] CXX_x86_64-unknown-linux-gnu = Some("sccache c++")
[00:25:17] CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
[00:25:17] DEBUG = Some("false")
[00:25:17] running: "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-m64" "-I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-fPIC" "-fvisibility-inlines-hidden" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-Werror=date-time" "-std=c++11" "-ffunction-sections" "-fdata-sections" "-O3" "-DNDEBUG" "-fno-exceptions" "-fno-rtti" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_JSBACKEND" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_RUSTLLVM" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-12f72c3c885a019d/out/../rustllvm/DemoteSimd.o" "-c" "../rustllvm/DemoteSimd.cpp"
[00:25:17] cargo:warning=../rustllvm/DemoteSimd.cpp: In function 'void LLVMRustDemoteSimdArguments(LLVMModuleRef)':
[00:25:17] cargo:warning=../rustllvm/DemoteSimd.cpp:86:23: error: 'AttributeList' has not been declared
[00:25:17] cargo:warning=     NF->setAttributes(AttributeList::get(F->getContext(),
[00:25:17] cargo:warning=                       ^
[00:25:17] cargo:warning=../rustllvm/DemoteSimd.cpp:105:13: error: 'AttributeList' does not name a type
[00:25:17] cargo:warning=       const AttributeList &CallPAL = CS.getAttributes();
[00:25:17] cargo:warning=             ^
[00:25:17] cargo:warning=../rustllvm/DemoteSimd.cpp:119:81: error: no matching function for call to 'llvm::AllocaInst::AllocaInst(llvm::Type*, int, std::nullptr_t, const char [1], llvm::Instruction*&)'
[00:25:17] cargo:warning=           AllocaInst *AllocA = new AllocaInst(I->getType(), 0, nullptr, "", Call);
[00:25:17] cargo:warning=                                                                                 ^
[00:25:17] cargo:warning=In file included from /checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/IRBuilder.h:33:0,
[00:25:17] cargo:warning=                 from ../rustllvm/rustllvm.h:23,
[00:25:17] cargo:warning=                 from ../rustllvm/DemoteSimd.cpp:14:
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:81:3: note: candidate: llvm::AllocaInst::AllocaInst(llvm::Type*, llvm::Value*, unsigned int, const llvm::Twine&, llvm::BasicBlock*)
[00:25:17] cargo:warning=   AllocaInst(Type *Ty, Value *ArraySize, unsigned Align,
[00:25:17] cargo:warning=   ^
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:81:3: note:   no known conversion for argument 3 from 'std::nullptr_t' to 'unsigned int'
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:79:3: note: candidate: llvm::AllocaInst::AllocaInst(llvm::Type*, llvm::Value*, unsigned int, const llvm::Twine&, llvm::Instruction*)
[00:25:17] cargo:warning=   AllocaInst(Type *Ty, Value *ArraySize, unsigned Align,
[00:25:17] cargo:warning=   ^
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:79:3: note:   no known conversion for argument 3 from 'std::nullptr_t' to 'unsigned int'
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:77:3: note: candidate: llvm::AllocaInst::AllocaInst(llvm::Type*, const llvm::Twine&, llvm::BasicBlock*)
[00:25:17] cargo:warning=   AllocaInst(Type *Ty, const Twine &Name, BasicBlock *InsertAtEnd);
[00:25:17] cargo:warning=   ^
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:77:3: note:   candidate expects 3 arguments, 5 provided
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:76:3: note: candidate: llvm::AllocaInst::AllocaInst(llvm::Type*, const llvm::Twine&, llvm::Instruction*)
[00:25:17] cargo:warning=   AllocaInst(Type *Ty, const Twine &Name, Instruction *InsertBefore = nullptr);
[00:25:17] cargo:warning=   ^
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:76:3: note:   candidate expects 3 arguments, 5 provided
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:73:3: note: candidate: llvm::AllocaInst::AllocaInst(llvm::Type*, llvm::Value*, const llvm::Twine&, llvm::BasicBlock*)
[00:25:17] cargo:warning=   AllocaInst(Type *Ty, Value *ArraySize,
[00:25:17] cargo:warning=   ^
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:73:3: note:   candidate expects 4 arguments, 5 provided
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:70:12: note: candidate: llvm::AllocaInst::AllocaInst(llvm::Type*, llvm::Value*, const llvm::Twine&, llvm::Instruction*)
[00:25:17] cargo:warning=   explicit AllocaInst(Type *Ty, Value *ArraySize = nullptr,
[00:25:17] cargo:warning=            ^
[00:25:17] cargo:warning=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include/llvm/IR/Instructions.h:70:12: note:   candidate expects 4 arguments, 5 provided
[00:25:17] cargo:warning=../rustllvm/DemoteSimd.cpp:125:32: error: 'CallPAL' was not declared in this scope
[00:25:17] cargo:warning=           ArgAttrVec.push_back(CallPAL.getParamAttributes(ArgNo));
[00:25:17] cargo:warning=                                ^
[00:25:17] cargo:warning=../rustllvm/DemoteSimd.cpp:146:11: error: 'AttributeList' has not been declared
[00:25:17] cargo:warning=           AttributeList::get(F->getContext(), CallPAL.getFnAttributes(),
[00:25:17] cargo:warning=           ^
[00:25:17] cargo:warning=../rustllvm/DemoteSimd.cpp:146:47: error: 'CallPAL' was not declared in this scope
[00:25:17] cargo:warning=           AttributeList::get(F->getContext(), CallPAL.getFnAttributes(),
[00:25:17] cargo:warning=                                               ^
[00:25:17] 
[00:25:17] --- stderr
[00:25:17] thread 'main' panicked at '
[00:25:17] 
[00:25:17] 
[00:25:17] Internal error occurred: Command "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-m64" "-I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten/include" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-fPIC" "-fvisibility-inlines-hidden" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-Werror=date-time" "-std=c++11" "-ffunction-sections" "-fdata-sections" "-O3" "-DNDEBUG" "-fno-exceptions" "-fno-rtti" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_JSBACKEND" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_RUSTLLVM" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-12f72c3c885a019d/out/../rustllvm/DemoteSimd.o" "-c" "../rustllvm/DemoteSimd.cpp" with args "c++" did not execute successfully (status code exit code: 1).
[00:25:17] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.25/src/lib.rs:2260:5
[00:25:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:25:17] 
[00:25:17] 
[00:25:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " emscripten" "--message-format" "json"
[00:25:17] expected success, got: exit code: 101
[00:25:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:25:17] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm

---
travis_time:end:2a60770d:start=1539898846826911737,finish=1539898846832556960,duration=5645223
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:20546850
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06f2eae8
travis_time:start:06f2eae8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1837573b
$ dmesg | grep -i kill
