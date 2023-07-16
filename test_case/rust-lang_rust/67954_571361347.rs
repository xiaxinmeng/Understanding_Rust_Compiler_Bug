plain
2020-01-06T23:10:18.9476059Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-06T23:10:18.9556224Z ##[command]git config gc.auto 0
2020-01-06T23:10:18.9634533Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-06T23:10:18.9702380Z ##[command]git config --get-all http.proxy
2020-01-06T23:10:18.9845332Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67954/merge:refs/remotes/pull/67954/merge
---
2020-01-06T23:18:30.9925540Z    Compiling rustc_index v0.0.0 (/checkout/src/librustc_index)
2020-01-06T23:18:37.0116064Z error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
2020-01-06T23:18:37.0120650Z 
2020-01-06T23:18:37.0121043Z Caused by:
2020-01-06T23:18:37.0129349Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/rustc_llvm-180f5513e9eeadc0/build-script-build` (exit code: 1)
2020-01-06T23:18:37.0130088Z --- stdout
2020-01-06T23:18:37.0132322Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
2020-01-06T23:18:37.0132880Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
2020-01-06T23:18:37.0133145Z cargo:rerun-if-changed=/usr/lib/llvm-7/bin/llvm-config
2020-01-06T23:18:37.0133400Z cargo:rerun-if-env-changed=LLVM_CONFIG
2020-01-06T23:18:37.0133635Z cargo:rustc-cfg=llvm_component="aarch64"
2020-01-06T23:18:37.0133867Z cargo:rustc-cfg=llvm_component="amdgpu"
2020-01-06T23:18:37.0134112Z cargo:rustc-cfg=llvm_component="arm"
2020-01-06T23:18:37.0134348Z cargo:rustc-cfg=llvm_component="asmparser"
2020-01-06T23:18:37.0134580Z cargo:rustc-cfg=llvm_component="bitreader"
2020-01-06T23:18:37.0134827Z cargo:rustc-cfg=llvm_component="bitwriter"
2020-01-06T23:18:37.0135062Z cargo:rustc-cfg=llvm_component="hexagon"
2020-01-06T23:18:37.0135304Z cargo:rustc-cfg=llvm_component="instrumentation"
2020-01-06T23:18:37.0135532Z cargo:rustc-cfg=llvm_component="ipo"
2020-01-06T23:18:37.0135792Z cargo:rustc-cfg=llvm_component="linker"
2020-01-06T23:18:37.0136022Z cargo:rustc-cfg=llvm_component="lto"
2020-01-06T23:18:37.0136251Z cargo:rustc-cfg=llvm_component="mips"
2020-01-06T23:18:37.0136514Z cargo:rustc-cfg=llvm_component="msp430"
2020-01-06T23:18:37.0136745Z cargo:rustc-cfg=llvm_component="nvptx"
2020-01-06T23:18:37.0136977Z cargo:rustc-cfg=llvm_component="powerpc"
2020-01-06T23:18:37.0137221Z cargo:rustc-cfg=llvm_component="sparc"
2020-01-06T23:18:37.0137458Z cargo:rustc-cfg=llvm_component="systemz"
2020-01-06T23:18:37.0137694Z cargo:rustc-cfg=llvm_component="webassembly"
2020-01-06T23:18:37.0137938Z cargo:rustc-cfg=llvm_component="x86"
2020-01-06T23:18:37.0138171Z cargo:rerun-if-changed-env=LLVM_RUSTLLVM
2020-01-06T23:18:37.0138416Z cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
2020-01-06T23:18:37.0138658Z cargo:rerun-if-changed=../rustllvm/.editorconfig
2020-01-06T23:18:37.0138918Z cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
2020-01-06T23:18:37.0139518Z cargo:rerun-if-changed=../rustllvm/Linker.cpp
2020-01-06T23:18:37.0139793Z cargo:rerun-if-changed=../rustllvm/rustllvm.h
2020-01-06T23:18:37.0140074Z cargo:rerun-if-changed=../rustllvm/README
2020-01-06T23:18:37.0140356Z cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
2020-01-06T23:18:37.0140616Z TARGET = Some("x86_64-unknown-linux-gnu")
2020-01-06T23:18:37.0140684Z OPT_LEVEL = Some("2")
2020-01-06T23:18:37.0140941Z HOST = Some("x86_64-unknown-linux-gnu")
2020-01-06T23:18:37.0141206Z CXX_x86_64-unknown-linux-gnu = Some("sccache c++")
2020-01-06T23:18:37.0141529Z CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
2020-01-06T23:18:37.0141588Z CRATE_CC_NO_DEFAULTS = None
2020-01-06T23:18:37.0141639Z DEBUG = Some("false")
2020-01-06T23:18:37.0141706Z CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
2020-01-06T23:18:37.0143527Z running: "sccache" "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-7/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-Wextra" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-30250cab0b252f3a/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp"
2020-01-06T23:18:37.0143863Z cargo:warning=../rustllvm/PassWrapper.cpp:16:10: fatal error: llvm/Passes/StandardInstrumentations.h: No such file or directory
2020-01-06T23:18:37.0143925Z cargo:warning= #include "llvm/Passes/StandardInstrumentations.h"
2020-01-06T23:18:37.0143981Z cargo:warning=          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2020-01-06T23:18:37.0144047Z cargo:warning=compilation terminated.
2020-01-06T23:18:37.0144125Z 
2020-01-06T23:18:37.0144400Z --- stderr
2020-01-06T23:18:37.0144450Z 
2020-01-06T23:18:37.0144478Z 
2020-01-06T23:18:37.0144478Z 
2020-01-06T23:18:37.0146271Z error occurred: Command "sccache" "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-7/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-Wextra" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-30250cab0b252f3a/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp" with args "c++" did not execute successfully (status code exit code: 1).
2020-01-06T23:18:37.0146560Z 
2020-01-06T23:18:37.0150756Z 
2020-01-06T23:18:37.0151468Z warning: build failed, waiting for other jobs to finish...
2020-01-06T23:18:40.4162744Z error: build failed
---
2020-01-06T23:18:40.4266862Z   local time: Mon Jan  6 23:18:40 UTC 2020
2020-01-06T23:18:40.7118571Z   network time: Mon, 06 Jan 2020 23:18:40 GMT
2020-01-06T23:18:40.7123865Z == end clock drift check ==
2020-01-06T23:18:42.1951435Z 
2020-01-06T23:18:42.2022433Z ##[error]Bash exited with code '1'.
2020-01-06T23:18:42.2048854Z ##[section]Starting: Checkout
2020-01-06T23:18:42.2050471Z ==============================================================================
2020-01-06T23:18:42.2050922Z Task         : Get sources
2020-01-06T23:18:42.2050985Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
