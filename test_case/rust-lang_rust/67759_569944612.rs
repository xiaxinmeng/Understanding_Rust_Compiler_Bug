plain
2019-12-31T15:00:13.5244805Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T15:00:14.1661198Z ##[command]git config gc.auto 0
2019-12-31T15:00:14.1666994Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T15:00:14.1671697Z ##[command]git config --get-all http.proxy
2019-12-31T15:00:14.1677709Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67759/merge:refs/remotes/pull/67759/merge
---
2019-12-31T15:07:48.9476380Z    Compiling memoffset v0.5.1
2019-12-31T15:07:49.9020375Z error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
2019-12-31T15:07:49.9032390Z 
2019-12-31T15:07:49.9032496Z Caused by:
2019-12-31T15:07:49.9033044Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/rustc_llvm-12ceb52d5dc2c5b7/build-script-build` (exit code: 1)
2019-12-31T15:07:49.9033287Z --- stdout
2019-12-31T15:07:49.9033654Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
2019-12-31T15:07:49.9033853Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
2019-12-31T15:07:49.9034087Z cargo:rerun-if-changed=/usr/lib/llvm-7/bin/llvm-config
2019-12-31T15:07:49.9034275Z cargo:rerun-if-env-changed=LLVM_CONFIG
2019-12-31T15:07:49.9034616Z cargo:rustc-cfg=llvm_component="aarch64"
2019-12-31T15:07:49.9034969Z cargo:rustc-cfg=llvm_component="amdgpu"
2019-12-31T15:07:49.9035492Z cargo:rustc-cfg=llvm_component="arm"
2019-12-31T15:07:49.9035914Z cargo:rustc-cfg=llvm_component="asmparser"
2019-12-31T15:07:49.9036624Z cargo:rustc-cfg=llvm_component="bitreader"
2019-12-31T15:07:49.9036814Z cargo:rustc-cfg=llvm_component="bitwriter"
2019-12-31T15:07:49.9037179Z cargo:rustc-cfg=llvm_component="hexagon"
2019-12-31T15:07:49.9037729Z cargo:rustc-cfg=llvm_component="instrumentation"
2019-12-31T15:07:49.9038138Z cargo:rustc-cfg=llvm_component="ipo"
2019-12-31T15:07:49.9038510Z cargo:rustc-cfg=llvm_component="linker"
2019-12-31T15:07:49.9038868Z cargo:rustc-cfg=llvm_component="lto"
2019-12-31T15:07:49.9039205Z cargo:rustc-cfg=llvm_component="mips"
2019-12-31T15:07:49.9039567Z cargo:rustc-cfg=llvm_component="msp430"
2019-12-31T15:07:49.9039905Z cargo:rustc-cfg=llvm_component="nvptx"
2019-12-31T15:07:49.9040241Z cargo:rustc-cfg=llvm_component="powerpc"
2019-12-31T15:07:49.9040605Z cargo:rustc-cfg=llvm_component="sparc"
2019-12-31T15:07:49.9040944Z cargo:rustc-cfg=llvm_component="systemz"
2019-12-31T15:07:49.9041442Z cargo:rustc-cfg=llvm_component="webassembly"
2019-12-31T15:07:49.9041968Z cargo:rustc-cfg=llvm_component="x86"
2019-12-31T15:07:49.9042296Z cargo:rerun-if-changed-env=LLVM_RUSTLLVM
2019-12-31T15:07:49.9042650Z cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
2019-12-31T15:07:49.9042998Z cargo:rerun-if-changed=../rustllvm/.editorconfig
2019-12-31T15:07:49.9043339Z cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
2019-12-31T15:07:49.9043696Z cargo:rerun-if-changed=../rustllvm/Linker.cpp
2019-12-31T15:07:49.9044378Z cargo:rerun-if-changed=../rustllvm/rustllvm.h
2019-12-31T15:07:49.9044717Z cargo:rerun-if-changed=../rustllvm/README
2019-12-31T15:07:49.9045335Z cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
2019-12-31T15:07:49.9045691Z TARGET = Some("x86_64-unknown-linux-gnu")
2019-12-31T15:07:49.9045872Z OPT_LEVEL = Some("2")
2019-12-31T15:07:49.9046141Z HOST = Some("x86_64-unknown-linux-gnu")
2019-12-31T15:07:49.9046336Z CXX_x86_64-unknown-linux-gnu = Some("sccache c++")
2019-12-31T15:07:49.9046578Z CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
2019-12-31T15:07:49.9046625Z CRATE_CC_NO_DEFAULTS = None
2019-12-31T15:07:49.9046666Z DEBUG = Some("false")
2019-12-31T15:07:49.9046722Z CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
2019-12-31T15:07:49.9048439Z running: "sccache" "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-7/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-Wextra" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-20b5b3b7a5d2cc79/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp"
2019-12-31T15:07:49.9049232Z cargo:warning=../rustllvm/PassWrapper.cpp: In function 'LLVMRustThinLTOData* LLVMRustCreateThinLTOData(LLVMRustThinLTOModule*, int, const char**, int)':
2019-12-31T15:07:49.9049623Z cargo:warning=../rustllvm/PassWrapper.cpp:866:19: error: 'make_unique' is not a member of 'std'
2019-12-31T15:07:49.9049810Z cargo:warning=   auto Ret = std::make_unique<LLVMRustThinLTOData>();
2019-12-31T15:07:49.9049886Z cargo:warning=                   ^~~~~~~~~~~
2019-12-31T15:07:49.9050168Z cargo:warning=../rustllvm/PassWrapper.cpp:866:19: note: suggested alternative: '__unique'
2019-12-31T15:07:49.9050452Z cargo:warning=   auto Ret = std::make_unique<LLVMRustThinLTOData>();
2019-12-31T15:07:49.9050543Z cargo:warning=                   ^~~~~~~~~~~
2019-12-31T15:07:49.9050602Z cargo:warning=                   __unique
2019-12-31T15:07:49.9050959Z cargo:warning=../rustllvm/PassWrapper.cpp:866:50: error: expected primary-expression before '>' token
2019-12-31T15:07:49.9051140Z cargo:warning=   auto Ret = std::make_unique<LLVMRustThinLTOData>();
2019-12-31T15:07:49.9051821Z cargo:warning=                                                  ^
2019-12-31T15:07:49.9052189Z cargo:warning=../rustllvm/PassWrapper.cpp:866:52: error: expected primary-expression before ')' token
2019-12-31T15:07:49.9052365Z cargo:warning=   auto Ret = std::make_unique<LLVMRustThinLTOData>();
2019-12-31T15:07:49.9052461Z cargo:warning=                                                    ^
2019-12-31T15:07:49.9052780Z cargo:warning=../rustllvm/PassWrapper.cpp: In function 'LLVMRustThinLTOBuffer* LLVMRustThinLTOBufferCreate(LLVMModuleRef)':
2019-12-31T15:07:49.9053209Z cargo:warning=../rustllvm/PassWrapper.cpp:1108:19: error: 'make_unique' is not a member of 'std'
2019-12-31T15:07:49.9053555Z cargo:warning=   auto Ret = std::make_unique<LLVMRustThinLTOBuffer>();
2019-12-31T15:07:49.9053845Z cargo:warning=                   ^~~~~~~~~~~
2019-12-31T15:07:49.9054388Z cargo:warning=../rustllvm/PassWrapper.cpp:1108:19: note: suggested alternative: '__unique'
2019-12-31T15:07:49.9054442Z cargo:warning=   auto Ret = std::make_unique<LLVMRustThinLTOBuffer>();
2019-12-31T15:07:49.9054634Z cargo:warning=                   ^~~~~~~~~~~
2019-12-31T15:07:49.9054713Z cargo:warning=                   __unique
2019-12-31T15:07:49.9055027Z cargo:warning=../rustllvm/PassWrapper.cpp:1108:52: error: expected primary-expression before '>' token
2019-12-31T15:07:49.9055229Z cargo:warning=   auto Ret = std::make_unique<LLVMRustThinLTOBuffer>();
2019-12-31T15:07:49.9055309Z cargo:warning=                                                    ^
2019-12-31T15:07:49.9055655Z cargo:warning=../rustllvm/PassWrapper.cpp:1108:54: error: expected primary-expression before ')' token
2019-12-31T15:07:49.9055856Z cargo:warning=   auto Ret = std::make_unique<LLVMRustThinLTOBuffer>();
2019-12-31T15:07:49.9055947Z cargo:warning=                                                      ^
2019-12-31T15:07:49.9056032Z cargo:warning=../rustllvm/PassWrapper.cpp: At global scope:
2019-12-31T15:07:49.9056455Z cargo:warning=../rustllvm/PassWrapper.cpp:838:1: warning: 'const llvm::GlobalValueSummary* getFirstDefinitionForLinker(const GlobalValueSummaryList&)' defined but not used [-Wunused-function]
2019-12-31T15:07:49.9056690Z cargo:warning= getFirstDefinitionForLinker(const GlobalValueSummaryList &GVSummaryList) {
2019-12-31T15:07:49.9057032Z cargo:warning= ^~~~~~~~~~~~~~~~~~~~~~~~~~~
2019-12-31T15:07:49.9057149Z 
2019-12-31T15:07:49.9057374Z --- stderr
2019-12-31T15:07:49.9057401Z 
2019-12-31T15:07:49.9058329Z 
2019-12-31T15:07:49.9058329Z 
2019-12-31T15:07:49.9060058Z error occurred: Command "sccache" "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-7/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-Wextra" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-20b5b3b7a5d2cc79/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp" with args "c++" did not execute successfully (status code exit code: 1).
2019-12-31T15:07:49.9060298Z 
2019-12-31T15:07:49.9060320Z 
2019-12-31T15:07:49.9061220Z warning: build failed, waiting for other jobs to finish...
2019-12-31T15:07:50.0699244Z error: build failed
---
2019-12-31T15:07:50.8023825Z   local time: Tue Dec 31 15:07:50 UTC 2019
2019-12-31T15:07:50.8023889Z   network time: Tue, 31 Dec 2019 15:07:50 GMT
2019-12-31T15:07:50.8023940Z == end clock drift check ==
2019-12-31T15:07:52.2523357Z 
2019-12-31T15:07:52.2583057Z ##[error]Bash exited with code '1'.
2019-12-31T15:07:52.2606973Z ##[section]Starting: Checkout
2019-12-31T15:07:52.2608333Z ==============================================================================
2019-12-31T15:07:52.2608377Z Task         : Get sources
2019-12-31T15:07:52.2608427Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
