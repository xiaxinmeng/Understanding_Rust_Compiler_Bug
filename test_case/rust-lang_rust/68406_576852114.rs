plain
2020-01-21T19:33:31.5437707Z ========================== Starting Command Output ===========================
2020-01-21T19:33:31.5440459Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/356722ba-da43-42d6-b6fb-563371cb6b9f.sh
2020-01-21T19:33:31.5440492Z 
2020-01-21T19:33:31.5442907Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T19:33:31.5449993Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68406/merge to s
2020-01-21T19:33:31.5451695Z Task         : Get sources
2020-01-21T19:33:31.5451725Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T19:33:31.5451768Z Version      : 1.0.0
2020-01-21T19:33:31.5451796Z Author       : Microsoft
---
2020-01-21T19:33:32.6003861Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T19:33:32.6017296Z ##[command]git config gc.auto 0
2020-01-21T19:33:32.6021166Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T19:33:32.6026048Z ##[command]git config --get-all http.proxy
2020-01-21T19:33:32.6033635Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68406/merge:refs/remotes/pull/68406/merge
---
2020-01-21T19:41:23.3912621Z    Compiling env_logger v0.7.1
2020-01-21T19:41:24.1488293Z error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
2020-01-21T19:41:24.1490010Z 
2020-01-21T19:41:24.1490277Z Caused by:
2020-01-21T19:41:24.1505036Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/rustc_llvm-180f5513e9eeadc0/build-script-build` (exit code: 1)
2020-01-21T19:41:24.1505504Z --- stdout
2020-01-21T19:41:24.1505751Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
2020-01-21T19:41:24.1506110Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
2020-01-21T19:41:24.1506314Z cargo:rerun-if-changed=/usr/lib/llvm-7/bin/llvm-config
2020-01-21T19:41:24.1506520Z cargo:rerun-if-env-changed=LLVM_CONFIG
2020-01-21T19:41:24.1506717Z cargo:rustc-cfg=llvm_component="aarch64"
2020-01-21T19:41:24.1506905Z cargo:rustc-cfg=llvm_component="amdgpu"
2020-01-21T19:41:24.1507106Z cargo:rustc-cfg=llvm_component="arm"
2020-01-21T19:41:24.1507290Z cargo:rustc-cfg=llvm_component="asmparser"
2020-01-21T19:41:24.1507473Z cargo:rustc-cfg=llvm_component="bitreader"
2020-01-21T19:41:24.1507674Z cargo:rustc-cfg=llvm_component="bitwriter"
2020-01-21T19:41:24.1507859Z cargo:rustc-cfg=llvm_component="hexagon"
2020-01-21T19:41:24.1508049Z cargo:rustc-cfg=llvm_component="instrumentation"
2020-01-21T19:41:24.1508260Z cargo:rustc-cfg=llvm_component="ipo"
2020-01-21T19:41:24.1508444Z cargo:rustc-cfg=llvm_component="linker"
2020-01-21T19:41:24.1508623Z cargo:rustc-cfg=llvm_component="lto"
2020-01-21T19:41:24.1508803Z cargo:rustc-cfg=llvm_component="mips"
2020-01-21T19:41:24.1509071Z cargo:rustc-cfg=llvm_component="msp430"
2020-01-21T19:41:24.1509250Z cargo:rustc-cfg=llvm_component="nvptx"
2020-01-21T19:41:24.1509432Z cargo:rustc-cfg=llvm_component="powerpc"
2020-01-21T19:41:24.1509640Z cargo:rustc-cfg=llvm_component="sparc"
2020-01-21T19:41:24.1509829Z cargo:rustc-cfg=llvm_component="systemz"
2020-01-21T19:41:24.1510015Z cargo:rustc-cfg=llvm_component="webassembly"
2020-01-21T19:41:24.1510215Z cargo:rustc-cfg=llvm_component="x86"
2020-01-21T19:41:24.1510397Z cargo:rerun-if-changed-env=LLVM_RUSTLLVM
2020-01-21T19:41:24.1510584Z cargo:rerun-if-changed=../rustllvm/rustllvm.h
2020-01-21T19:41:24.1510784Z cargo:rerun-if-changed=../rustllvm/README
2020-01-21T19:41:24.1510975Z cargo:rerun-if-changed=../rustllvm/Linker.cpp
2020-01-21T19:41:24.1511180Z cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
2020-01-21T19:41:24.1511387Z cargo:rerun-if-changed=../rustllvm/.editorconfig
2020-01-21T19:41:24.1511583Z cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
2020-01-21T19:41:24.1511775Z cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
2020-01-21T19:41:24.1511966Z TARGET = Some("x86_64-unknown-linux-gnu")
2020-01-21T19:41:24.1512027Z OPT_LEVEL = Some("2")
2020-01-21T19:41:24.1512220Z HOST = Some("x86_64-unknown-linux-gnu")
2020-01-21T19:41:24.1512417Z CXX_x86_64-unknown-linux-gnu = Some("sccache c++")
2020-01-21T19:41:24.1512666Z CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
2020-01-21T19:41:24.1512713Z CRATE_CC_NO_DEFAULTS = None
2020-01-21T19:41:24.1512755Z DEBUG = Some("false")
2020-01-21T19:41:24.1512811Z CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
2020-01-21T19:41:24.1514750Z running: "sccache" "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-7/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-Wextra" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-30250cab0b252f3a/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp"
2020-01-21T19:41:24.1515492Z cargo:warning=../rustllvm/PassWrapper.cpp:708:5: error: 'LLVMRustSelfProfileBeforePassCallback' has not been declared
2020-01-21T19:41:24.1515561Z cargo:warning=     LLVMRustSelfProfileBeforePassCallback BeforePassCallback,
2020-01-21T19:41:24.1515613Z cargo:warning=     ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2020-01-21T19:41:24.1517459Z cargo:warning=../rustllvm/PassWrapper.cpp:709:5: error: 'LLVMRustSelfProfileAfterPassCallback' has not been declared
2020-01-21T19:41:24.1517522Z cargo:warning=     LLVMRustSelfProfileAfterPassCallback AfterPassCallback) {
2020-01-21T19:41:24.1517571Z cargo:warning=     ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2020-01-21T19:41:24.1517956Z cargo:warning=../rustllvm/PassWrapper.cpp:315:39: warning: 'llvm::PassBuilder::OptimizationLevel fromRust(LLVMRustPassBuilderOptLevel)' defined but not used [-Wunused-function]
2020-01-21T19:41:24.1518023Z cargo:warning= static PassBuilder::OptimizationLevel fromRust(LLVMRustPassBuilderOptLevel Level) {
2020-01-21T19:41:24.1518097Z cargo:warning=                                       ^~~~~~~~
2020-01-21T19:41:24.1518172Z 
2020-01-21T19:41:24.1518364Z --- stderr
2020-01-21T19:41:24.1518414Z 
2020-01-21T19:41:24.1518440Z 
2020-01-21T19:41:24.1518440Z 
2020-01-21T19:41:24.1520677Z error occurred: Command "sccache" "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-7/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-Wextra" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-30250cab0b252f3a/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp" with args "c++" did not execute successfully (status code exit code: 1).
2020-01-21T19:41:24.1520935Z 
2020-01-21T19:41:24.1520958Z 
2020-01-21T19:41:24.1521197Z warning: build failed, waiting for other jobs to finish...
2020-01-21T19:41:24.8447432Z error: build failed
---
2020-01-21T19:41:24.8552354Z   local time: Tue Jan 21 19:41:24 UTC 2020
2020-01-21T19:41:25.1479495Z   network time: Tue, 21 Jan 2020 19:41:25 GMT
2020-01-21T19:41:25.1482734Z == end clock drift check ==
2020-01-21T19:41:26.4956726Z 
2020-01-21T19:41:26.5048791Z ##[error]Bash exited with code '1'.
2020-01-21T19:41:26.5060211Z ##[section]Finishing: Run build
2020-01-21T19:41:26.5073999Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68406/merge to s
2020-01-21T19:41:26.5076616Z Task         : Get sources
2020-01-21T19:41:26.5076661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T19:41:26.5076723Z Version      : 1.0.0
2020-01-21T19:41:26.5076763Z Author       : Microsoft
2020-01-21T19:41:26.5076763Z Author       : Microsoft
2020-01-21T19:41:26.5076807Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-21T19:41:26.5076885Z ==============================================================================
2020-01-21T19:41:26.9254329Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-21T19:41:26.9301168Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68406/merge to s
2020-01-21T19:41:26.9407932Z Cleaning up task key
2020-01-21T19:41:26.9408752Z Start cleaning up orphan processes.
2020-01-21T19:41:26.9517403Z Terminate orphan process: pid (3375) (python)
2020-01-21T19:41:26.9727045Z ##[section]Finishing: Finalize Job
