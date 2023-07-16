plain
2020-01-21T19:01:17.2118367Z ========================== Starting Command Output ===========================
2020-01-21T19:01:17.2134578Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/efba20dd-0aa7-4fe3-b302-5f5bbd2b3c69.sh
2020-01-21T19:01:17.2315893Z 
2020-01-21T19:01:17.2376884Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T19:01:17.2383267Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68406/merge to s
2020-01-21T19:01:17.2384974Z Task         : Get sources
2020-01-21T19:01:17.2385051Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T19:01:17.2385087Z Version      : 1.0.0
2020-01-21T19:01:17.2385122Z Author       : Microsoft
---
2020-01-21T19:01:18.0916030Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T19:01:18.1001746Z ##[command]git config gc.auto 0
2020-01-21T19:01:18.1076346Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T19:01:18.1133259Z ##[command]git config --get-all http.proxy
2020-01-21T19:01:18.1264191Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68406/merge:refs/remotes/pull/68406/merge
---
2020-01-21T19:08:19.8751858Z    Compiling jobserver v0.1.16
2020-01-21T19:08:21.0428035Z error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
2020-01-21T19:08:21.0441039Z 
2020-01-21T19:08:21.0441295Z Caused by:
2020-01-21T19:08:21.0490935Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/rustc_llvm-180f5513e9eeadc0/build-script-build` (exit code: 1)
2020-01-21T19:08:21.0492868Z --- stdout
2020-01-21T19:08:21.0493132Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
2020-01-21T19:08:21.0494415Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
2020-01-21T19:08:21.0494696Z cargo:rerun-if-changed=/usr/lib/llvm-7/bin/llvm-config
2020-01-21T19:08:21.0494979Z cargo:rerun-if-env-changed=LLVM_CONFIG
2020-01-21T19:08:21.0495219Z cargo:rustc-cfg=llvm_component="aarch64"
2020-01-21T19:08:21.0495452Z cargo:rustc-cfg=llvm_component="amdgpu"
2020-01-21T19:08:21.0495712Z cargo:rustc-cfg=llvm_component="arm"
2020-01-21T19:08:21.0496070Z cargo:rustc-cfg=llvm_component="asmparser"
2020-01-21T19:08:21.0496307Z cargo:rustc-cfg=llvm_component="bitreader"
2020-01-21T19:08:21.0496549Z cargo:rustc-cfg=llvm_component="bitwriter"
2020-01-21T19:08:21.0496879Z cargo:rustc-cfg=llvm_component="hexagon"
2020-01-21T19:08:21.0497131Z cargo:rustc-cfg=llvm_component="instrumentation"
2020-01-21T19:08:21.0497362Z cargo:rustc-cfg=llvm_component="ipo"
2020-01-21T19:08:21.0497711Z cargo:rustc-cfg=llvm_component="linker"
2020-01-21T19:08:21.0497949Z cargo:rustc-cfg=llvm_component="lto"
2020-01-21T19:08:21.0500322Z cargo:rustc-cfg=llvm_component="mips"
2020-01-21T19:08:21.0500737Z cargo:rustc-cfg=llvm_component="msp430"
2020-01-21T19:08:21.0501288Z cargo:rustc-cfg=llvm_component="nvptx"
2020-01-21T19:08:21.0501540Z cargo:rustc-cfg=llvm_component="powerpc"
2020-01-21T19:08:21.0501782Z cargo:rustc-cfg=llvm_component="sparc"
2020-01-21T19:08:21.0502008Z cargo:rustc-cfg=llvm_component="systemz"
2020-01-21T19:08:21.0502234Z cargo:rustc-cfg=llvm_component="webassembly"
2020-01-21T19:08:21.0502459Z cargo:rustc-cfg=llvm_component="x86"
2020-01-21T19:08:21.0502701Z cargo:rerun-if-changed-env=LLVM_RUSTLLVM
2020-01-21T19:08:21.0502933Z cargo:rerun-if-changed=../rustllvm/rustllvm.h
2020-01-21T19:08:21.0503158Z cargo:rerun-if-changed=../rustllvm/README
2020-01-21T19:08:21.0503402Z cargo:rerun-if-changed=../rustllvm/Linker.cpp
2020-01-21T19:08:21.0503655Z cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
2020-01-21T19:08:21.0503891Z cargo:rerun-if-changed=../rustllvm/.editorconfig
2020-01-21T19:08:21.0504147Z cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
2020-01-21T19:08:21.0504381Z cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
2020-01-21T19:08:21.0504608Z TARGET = Some("x86_64-unknown-linux-gnu")
2020-01-21T19:08:21.0504687Z OPT_LEVEL = Some("2")
2020-01-21T19:08:21.0504915Z HOST = Some("x86_64-unknown-linux-gnu")
2020-01-21T19:08:21.0505150Z CXX_x86_64-unknown-linux-gnu = Some("sccache c++")
2020-01-21T19:08:21.0505443Z CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
2020-01-21T19:08:21.0505501Z CRATE_CC_NO_DEFAULTS = None
2020-01-21T19:08:21.0505551Z DEBUG = Some("false")
2020-01-21T19:08:21.0505616Z CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
2020-01-21T19:08:21.0507440Z running: "sccache" "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-7/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-Wextra" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-30250cab0b252f3a/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp"
2020-01-21T19:08:21.0508014Z cargo:warning=../rustllvm/PassWrapper.cpp:630:50: error: 'Any' in namespace 'llvm' does not name a type
2020-01-21T19:08:21.0508080Z cargo:warning= std::string LLVMRustwrappedIrGetName(const llvm::Any &WrappedIr) {
2020-01-21T19:08:21.0508140Z cargo:warning=                                                  ^~~
2020-01-21T19:08:21.0508461Z cargo:warning=../rustllvm/PassWrapper.cpp: In function 'std::__cxx11::string LLVMRustwrappedIrGetName(const int&)':
2020-01-21T19:08:21.0508751Z cargo:warning=../rustllvm/PassWrapper.cpp:631:7: error: 'any_isa' was not declared in this scope
2020-01-21T19:08:21.0508812Z cargo:warning=   if (any_isa<const Module *>(WrappedIr))
2020-01-21T19:08:21.0508878Z cargo:warning=       ^~~~~~~
2020-01-21T19:08:21.0509171Z cargo:warning=../rustllvm/PassWrapper.cpp:631:15: error: expected primary-expression before 'const'
2020-01-21T19:08:21.0509229Z cargo:warning=   if (any_isa<const Module *>(WrappedIr))
2020-01-21T19:08:21.0509297Z cargo:warning=               ^~~~~
2020-01-21T19:08:21.0509576Z cargo:warning=../rustllvm/PassWrapper.cpp:631:15: error: expected ')' before 'const'
2020-01-21T19:08:21.0509864Z cargo:warning=../rustllvm/PassWrapper.cpp:632:12: error: 'any_cast' was not declared in this scope
2020-01-21T19:08:21.0510146Z cargo:warning=     return any_cast<const Module *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0510198Z cargo:warning=            ^~~~~~~~
2020-01-21T19:08:21.0510479Z cargo:warning=../rustllvm/PassWrapper.cpp:632:21: error: expected primary-expression before 'const'
2020-01-21T19:08:21.0510759Z cargo:warning=     return any_cast<const Module *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0510812Z cargo:warning=                     ^~~~~
2020-01-21T19:08:21.0511090Z cargo:warning=../rustllvm/PassWrapper.cpp:632:21: error: expected ';' before 'const'
2020-01-21T19:08:21.0511414Z cargo:warning=../rustllvm/PassWrapper.cpp:631:3: warning: this 'if' clause does not guard... [-Wmisleading-indentation]
2020-01-21T19:08:21.0511472Z cargo:warning=   if (any_isa<const Module *>(WrappedIr))
2020-01-21T19:08:21.0511520Z cargo:warning=   ^~
2020-01-21T19:08:21.0511881Z cargo:warning=../rustllvm/PassWrapper.cpp:632:21: note: ...this statement, but the latter is misleadingly indented as if it were guarded by the 'if'
2020-01-21T19:08:21.0512154Z cargo:warning=     return any_cast<const Module *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0512224Z cargo:warning=                     ^~~~~
2020-01-21T19:08:21.0512507Z cargo:warning=../rustllvm/PassWrapper.cpp:632:35: error: expected unqualified-id before '>' token
2020-01-21T19:08:21.0512774Z cargo:warning=     return any_cast<const Module *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0512854Z cargo:warning=                                   ^
2020-01-21T19:08:21.0513136Z cargo:warning=../rustllvm/PassWrapper.cpp:632:35: error: expected initializer before '>' token
2020-01-21T19:08:21.0513420Z cargo:warning=../rustllvm/PassWrapper.cpp:633:7: error: 'any_isa' was not declared in this scope
2020-01-21T19:08:21.0513494Z cargo:warning=   if (any_isa<const Function *>(WrappedIr))
2020-01-21T19:08:21.0513598Z cargo:warning=       ^~~~~~~
2020-01-21T19:08:21.0513896Z cargo:warning=../rustllvm/PassWrapper.cpp:633:15: error: expected primary-expression before 'const'
2020-01-21T19:08:21.0513971Z cargo:warning=   if (any_isa<const Function *>(WrappedIr))
2020-01-21T19:08:21.0514022Z cargo:warning=               ^~~~~
2020-01-21T19:08:21.0514290Z cargo:warning=../rustllvm/PassWrapper.cpp:633:15: error: expected ')' before 'const'
2020-01-21T19:08:21.0514590Z cargo:warning=../rustllvm/PassWrapper.cpp:634:12: error: 'any_cast' was not declared in this scope
2020-01-21T19:08:21.0514859Z cargo:warning=     return any_cast<const Function *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0514965Z cargo:warning=            ^~~~~~~~
2020-01-21T19:08:21.0515272Z cargo:warning=../rustllvm/PassWrapper.cpp:634:21: error: expected primary-expression before 'const'
2020-01-21T19:08:21.0515541Z cargo:warning=     return any_cast<const Function *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0515604Z cargo:warning=                     ^~~~~
2020-01-21T19:08:21.0515888Z cargo:warning=../rustllvm/PassWrapper.cpp:634:21: error: expected ';' before 'const'
2020-01-21T19:08:21.0516195Z cargo:warning=../rustllvm/PassWrapper.cpp:633:3: warning: this 'if' clause does not guard... [-Wmisleading-indentation]
2020-01-21T19:08:21.0516254Z cargo:warning=   if (any_isa<const Function *>(WrappedIr))
2020-01-21T19:08:21.0516318Z cargo:warning=   ^~
2020-01-21T19:08:21.0516650Z cargo:warning=../rustllvm/PassWrapper.cpp:634:21: note: ...this statement, but the latter is misleadingly indented as if it were guarded by the 'if'
2020-01-21T19:08:21.0516931Z cargo:warning=     return any_cast<const Function *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0517001Z cargo:warning=                     ^~~~~
2020-01-21T19:08:21.0517281Z cargo:warning=../rustllvm/PassWrapper.cpp:634:37: error: expected unqualified-id before '>' token
2020-01-21T19:08:21.0517547Z cargo:warning=     return any_cast<const Function *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0517628Z cargo:warning=                                     ^
2020-01-21T19:08:21.0517910Z cargo:warning=../rustllvm/PassWrapper.cpp:634:37: error: expected initializer before '>' token
2020-01-21T19:08:21.0518191Z cargo:warning=../rustllvm/PassWrapper.cpp:635:7: error: 'any_isa' was not declared in this scope
2020-01-21T19:08:21.0518263Z cargo:warning=   if (any_isa<const Loop *>(WrappedIr))
2020-01-21T19:08:21.0518314Z cargo:warning=       ^~~~~~~
2020-01-21T19:08:21.0518594Z cargo:warning=../rustllvm/PassWrapper.cpp:635:15: error: expected primary-expression before 'const'
2020-01-21T19:08:21.0518674Z cargo:warning=   if (any_isa<const Loop *>(WrappedIr))
2020-01-21T19:08:21.0518725Z cargo:warning=               ^~~~~
2020-01-21T19:08:21.0518995Z cargo:warning=../rustllvm/PassWrapper.cpp:635:15: error: expected ')' before 'const'
2020-01-21T19:08:21.0519294Z cargo:warning=../rustllvm/PassWrapper.cpp:636:12: error: 'any_cast' was not declared in this scope
2020-01-21T19:08:21.0519568Z cargo:warning=     return any_cast<const Loop *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0519621Z cargo:warning=            ^~~~~~~~
2020-01-21T19:08:21.0519919Z cargo:warning=../rustllvm/PassWrapper.cpp:636:21: error: expected primary-expression before 'const'
2020-01-21T19:08:21.0520289Z cargo:warning=     return any_cast<const Loop *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0520506Z cargo:warning=                     ^~~~~
2020-01-21T19:08:21.0520790Z cargo:warning=../rustllvm/PassWrapper.cpp:636:21: error: expected ';' before 'const'
2020-01-21T19:08:21.0521097Z cargo:warning=../rustllvm/PassWrapper.cpp:635:3: warning: this 'if' clause does not guard... [-Wmisleading-indentation]
2020-01-21T19:08:21.0521165Z cargo:warning=   if (any_isa<const Loop *>(WrappedIr))
2020-01-21T19:08:21.0521229Z cargo:warning=   ^~
2020-01-21T19:08:21.0521562Z cargo:warning=../rustllvm/PassWrapper.cpp:636:21: note: ...this statement, but the latter is misleadingly indented as if it were guarded by the 'if'
2020-01-21T19:08:21.0521991Z cargo:warning=     return any_cast<const Loop *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0522061Z cargo:warning=                     ^~~~~
2020-01-21T19:08:21.0522494Z cargo:warning=../rustllvm/PassWrapper.cpp:636:33: error: expected unqualified-id before '>' token
2020-01-21T19:08:21.0522860Z cargo:warning=     return any_cast<const Loop *>(WrappedIr)->getName().str();
2020-01-21T19:08:21.0522926Z cargo:warning=                                 ^
2020-01-21T19:08:21.0523184Z cargo:warning=../rustllvm/PassWrapper.cpp:636:33: error: expected initializer before '>' token
2020-01-21T19:08:21.0523683Z cargo:warning=../rustllvm/PassWrapper.cpp:637:7: error: 'any_isa' was not declared in this scope
2020-01-21T19:08:21.0523757Z cargo:warning=   if (any_isa<const LazyCallGraph::SCC *>(WrappedIr))
2020-01-21T19:08:21.0523809Z cargo:warning=       ^~~~~~~
2020-01-21T19:08:21.0524090Z cargo:warning=../rustllvm/PassWrapper.cpp:637:15: error: expected primary-expression before 'const'
2020-01-21T19:08:21.0524174Z cargo:warning=   if (any_isa<const LazyCallGraph::SCC *>(WrappedIr))
2020-01-21T19:08:21.0524225Z cargo:warning=               ^~~~~
2020-01-21T19:08:21.0524496Z cargo:warning=../rustllvm/PassWrapper.cpp:637:15: error: expected ')' before 'const'
2020-01-21T19:08:21.0524796Z cargo:warning=../rustllvm/PassWrapper.cpp:638:12: error: 'any_cast' was not declared in this scope
2020-01-21T19:08:21.0525068Z cargo:warning=     return any_cast<const LazyCallGraph::SCC *>(WrappedIr)->getName();
2020-01-21T19:08:21.0525121Z cargo:warning=            ^~~~~~~~
2020-01-21T19:08:21.0525415Z cargo:warning=../rustllvm/PassWrapper.cpp:638:21: error: expected primary-expression before 'const'
2020-01-21T19:08:21.0525698Z cargo:warning=     return any_cast<const LazyCallGraph::SCC *>(WrappedIr)->getName();
2020-01-21T19:08:21.0525753Z cargo:warning=                     ^~~~~
2020-01-21T19:08:21.0526034Z cargo:warning=../rustllvm/PassWrapper.cpp:638:21: error: expected ';' before 'const'
2020-01-21T19:08:21.0526352Z cargo:warning=../rustllvm/PassWrapper.cpp:637:3: warning: this 'if' clause does not guard... [-Wmisleading-indentation]
2020-01-21T19:08:21.0526411Z cargo:warning=   if (any_isa<const LazyCallGraph::SCC *>(WrappedIr))
2020-01-21T19:08:21.0526475Z cargo:warning=   ^~
2020-01-21T19:08:21.0526809Z cargo:warning=../rustllvm/PassWrapper.cpp:638:21: note: ...this statement, but the latter is misleadingly indented as if it were guarded by the 'if'
2020-01-21T19:08:21.0527083Z cargo:warning=     return any_cast<const LazyCallGraph::SCC *>(WrappedIr)->getName();
2020-01-21T19:08:21.0527152Z cargo:warning=                     ^~~~~
2020-01-21T19:08:21.0527445Z cargo:warning=../rustllvm/PassWrapper.cpp:638:47: error: expected unqualified-id before '>' token
2020-01-21T19:08:21.0527718Z cargo:warning=     return any_cast<const LazyCallGraph::SCC *>(WrappedIr)->getName();
2020-01-21T19:08:21.0527792Z cargo:warning=                                               ^
2020-01-21T19:08:21.0528084Z cargo:warning=../rustllvm/PassWrapper.cpp:638:47: error: expected initializer before '>' token
2020-01-21T19:08:21.0528142Z cargo:warning=../rustllvm/PassWrapper.cpp: At global scope:
2020-01-21T19:08:21.0528468Z cargo:warning=../rustllvm/PassWrapper.cpp:644:5: error: variable or field 'LLVMSelfProfileInitializeCallbacks' declared void
2020-01-21T19:08:21.0528530Z cargo:warning=     PassInstrumentationCallbacks& PIC, void* LlvmSelfProfiler,
2020-01-21T19:08:21.0528583Z cargo:warning=     ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
2020-01-21T19:08:21.0528901Z cargo:warning=../rustllvm/PassWrapper.cpp:644:5: error: 'PassInstrumentationCallbacks' was not declared in this scope
2020-01-21T19:08:21.0529196Z cargo:warning=../rustllvm/PassWrapper.cpp:644:35: error: 'PIC' was not declared in this scope
2020-01-21T19:08:21.0530694Z cargo:warning=     PassInstrumentationCallbacks& PIC, void* LlvmSelfProfiler,
2020-01-21T19:08:21.0530766Z cargo:warning=                                   ^~~
2020-01-21T19:08:21.0531142Z cargo:warning=../rustllvm/PassWrapper.cpp:644:40: error: expected primary-expression before 'void'
2020-01-21T19:08:21.0531205Z cargo:warning=     PassInstrumentationCallbacks& PIC, void* LlvmSelfProfiler,
2020-01-21T19:08:21.0531275Z cargo:warning=                                        ^~~~
2020-01-21T19:08:21.0531585Z cargo:warning=../rustllvm/PassWrapper.cpp:645:43: error: expected primary-expression before 'BeforePassCallback'
2020-01-21T19:08:21.0531646Z cargo:warning=     LLVMRustSelfProfileBeforePassCallback BeforePassCallback,
2020-01-21T19:08:21.0531721Z cargo:warning=                                           ^~~~~~~~~~~~~~~~~~
2020-01-21T19:08:21.0532089Z cargo:warning=../rustllvm/PassWrapper.cpp:646:42: error: expected primary-expression before 'AfterPassCallback'
2020-01-21T19:08:21.0532149Z cargo:warning=     LLVMRustSelfProfileAfterPassCallback AfterPassCallback) {
2020-01-21T19:08:21.0532224Z cargo:warning=                                          ^~~~~~~~~~~~~~~~~
2020-01-21T19:08:21.0532597Z cargo:warning=../rustllvm/PassWrapper.cpp:315:39: warning: 'llvm::PassBuilder::OptimizationLevel fromRust(LLVMRustPassBuilderOptLevel)' defined but not used [-Wunused-function]
2020-01-21T19:08:21.0532668Z cargo:warning= static PassBuilder::OptimizationLevel fromRust(LLVMRustPassBuilderOptLevel Level) {
2020-01-21T19:08:21.0532742Z cargo:warning=                                       ^~~~~~~~
2020-01-21T19:08:21.0532822Z 
2020-01-21T19:08:21.0533033Z --- stderr
2020-01-21T19:08:21.0533064Z 
2020-01-21T19:08:21.0533092Z 
2020-01-21T19:08:21.0533092Z 
2020-01-21T19:08:21.0534874Z error occurred: Command "sccache" "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-7/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-Wextra" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-30250cab0b252f3a/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp" with args "c++" did not execute successfully (status code exit code: 1).
2020-01-21T19:08:21.0535128Z 
2020-01-21T19:08:21.0535156Z 
2020-01-21T19:08:21.0535445Z warning: build failed, waiting for other jobs to finish...
2020-01-21T19:08:21.5218526Z error: build failed
---
2020-01-21T19:08:21.5316120Z   local time: Tue Jan 21 19:08:21 UTC 2020
2020-01-21T19:08:21.8254126Z   network time: Tue, 21 Jan 2020 19:08:21 GMT
2020-01-21T19:08:21.8255057Z == end clock drift check ==
2020-01-21T19:08:23.3975703Z 
2020-01-21T19:08:23.4075458Z ##[error]Bash exited with code '1'.
2020-01-21T19:08:23.4086992Z ##[section]Finishing: Run build
2020-01-21T19:08:23.4101835Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68406/merge to s
2020-01-21T19:08:23.4103796Z Task         : Get sources
2020-01-21T19:08:23.4103847Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T19:08:23.4103913Z Version      : 1.0.0
2020-01-21T19:08:23.4103960Z Author       : Microsoft
2020-01-21T19:08:23.4103960Z Author       : Microsoft
2020-01-21T19:08:23.4104010Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-21T19:08:23.4104082Z ==============================================================================
2020-01-21T19:08:23.8121673Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-21T19:08:23.8165521Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68406/merge to s
2020-01-21T19:08:23.8270794Z Cleaning up task key
2020-01-21T19:08:23.8271550Z Start cleaning up orphan processes.
2020-01-21T19:08:23.8373492Z Terminate orphan process: pid (3522) (python)
2020-01-21T19:08:23.8550962Z ##[section]Finishing: Finalize Job
