plain
2019-10-23T13:13:48.2384409Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-23T13:13:48.2580527Z ##[command]git config gc.auto 0
2019-10-23T13:13:48.2649719Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-23T13:13:48.2713918Z ##[command]git config --get-all http.proxy
2019-10-23T13:13:48.2848909Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65720/merge:refs/remotes/pull/65720/merge
---
2019-10-23T13:43:08.7645068Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2019-10-23T13:43:14.7373326Z error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
2019-10-23T13:43:14.7377900Z 
2019-10-23T13:43:14.7378266Z Caused by:
2019-10-23T13:43:14.7379049Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/release/build/rustc_llvm-ed5e716ea7a9979f/build-script-build` (exit code: 101)
2019-10-23T13:43:14.7379617Z --- stdout
2019-10-23T13:43:14.7380174Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
2019-10-23T13:43:14.7380748Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
2019-10-23T13:43:14.7382817Z cargo:rerun-if-changed=/usr/lib/llvm-6.0/bin/llvm-config
2019-10-23T13:43:14.7383517Z cargo:rerun-if-env-changed=LLVM_CONFIG
2019-10-23T13:43:14.7384078Z cargo:rustc-cfg=llvm_component="aarch64"
2019-10-23T13:43:14.7384608Z cargo:rustc-cfg=llvm_component="amdgpu"
2019-10-23T13:43:14.7385159Z cargo:rustc-cfg=llvm_component="arm"
2019-10-23T13:43:14.7385693Z cargo:rustc-cfg=llvm_component="asmparser"
2019-10-23T13:43:14.7386246Z cargo:rustc-cfg=llvm_component="bitreader"
2019-10-23T13:43:14.7386818Z cargo:rustc-cfg=llvm_component="bitwriter"
2019-10-23T13:43:14.7387354Z cargo:rustc-cfg=llvm_component="hexagon"
2019-10-23T13:43:14.7387889Z cargo:rustc-cfg=llvm_component="instrumentation"
2019-10-23T13:43:14.7388440Z cargo:rustc-cfg=llvm_component="ipo"
2019-10-23T13:43:14.7388972Z cargo:rustc-cfg=llvm_component="linker"
2019-10-23T13:43:14.7389511Z cargo:rustc-cfg=llvm_component="lto"
2019-10-23T13:43:14.7390042Z cargo:rustc-cfg=llvm_component="mips"
2019-10-23T13:43:14.7390575Z cargo:rustc-cfg=llvm_component="msp430"
2019-10-23T13:43:14.7391453Z cargo:rustc-cfg=llvm_component="nvptx"
2019-10-23T13:43:14.7392522Z cargo:rustc-cfg=llvm_component="powerpc"
2019-10-23T13:43:14.7393117Z cargo:rustc-cfg=llvm_component="sparc"
2019-10-23T13:43:14.7393685Z cargo:rustc-cfg=llvm_component="systemz"
2019-10-23T13:43:14.7394210Z cargo:rustc-cfg=llvm_component="x86"
2019-10-23T13:43:14.7394740Z cargo:rerun-if-changed-env=LLVM_RUSTLLVM
2019-10-23T13:43:14.7395550Z cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
2019-10-23T13:43:14.7396163Z cargo:rerun-if-changed=../rustllvm/Linker.cpp
2019-10-23T13:43:14.7397494Z cargo:rerun-if-changed=../rustllvm/.editorconfig
2019-10-23T13:43:14.7398091Z cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
2019-10-23T13:43:14.7398594Z cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
2019-10-23T13:43:14.7399085Z cargo:rerun-if-changed=../rustllvm/README
2019-10-23T13:43:14.7399526Z cargo:rerun-if-changed=../rustllvm/rustllvm.h
2019-10-23T13:43:14.7399957Z TARGET = Some("x86_64-unknown-linux-gnu")
2019-10-23T13:43:14.7400183Z OPT_LEVEL = Some("2")
2019-10-23T13:43:14.7400576Z HOST = Some("x86_64-unknown-linux-gnu")
2019-10-23T13:43:14.7401013Z CXX_x86_64-unknown-linux-gnu = Some("sccache c++")
2019-10-23T13:43:14.7402119Z CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
2019-10-23T13:43:14.7402350Z CRATE_CC_NO_DEFAULTS = None
2019-10-23T13:43:14.7402760Z DEBUG = Some("false")
2019-10-23T13:43:14.7402930Z CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
2019-10-23T13:43:14.7404695Z running: "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-6.0/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-3a1e09189d25ee92/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp"
2019-10-23T13:43:14.7405083Z exit code: 0
2019-10-23T13:43:14.7406920Z running: "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-6.0/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-3a1e09189d25ee92/out/../rustllvm/RustWrapper.o" "-c" "../rustllvm/RustWrapper.cpp"
2019-10-23T13:43:14.7407611Z cargo:warning=../rustllvm/RustWrapper.cpp: In function 'unsigned int LLVMRustGetInstructionCount(LLVMModuleRef)':
2019-10-23T13:43:14.7408251Z cargo:warning=../rustllvm/RustWrapper.cpp:91:21: error: 'class llvm::Module' has no member named 'getInstructionCount'
2019-10-23T13:43:14.7408760Z cargo:warning=   return unwrap(M)->getInstructionCount();
2019-10-23T13:43:14.7408982Z cargo:warning=                     ^
2019-10-23T13:43:14.7409258Z 
2019-10-23T13:43:14.7409611Z --- stderr
2019-10-23T13:43:14.7409999Z thread 'main' panicked at '
2019-10-23T13:43:14.7410167Z 
2019-10-23T13:43:14.7410167Z 
2019-10-23T13:43:14.7429630Z Internal error occurred: Command "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-6.0/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-3a1e09189d25ee92/out/../rustllvm/RustWrapper.o" "-c" "../rustllvm/RustWrapper.cpp" with args "c++" did not execute successfully (status code exit code: 1).
2019-10-23T13:43:14.7439048Z ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.35/src/lib.rs:2398:5
2019-10-23T13:43:14.7439413Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-23T13:43:14.7439568Z 
2019-10-23T13:43:14.7440404Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json-render-diagnostics"
---
2019-10-23T13:43:14.7477741Z   local time: Wed Oct 23 13:43:14 UTC 2019
2019-10-23T13:43:14.8903367Z   network time: Wed, 23 Oct 2019 13:43:14 GMT
2019-10-23T13:43:14.8903533Z == end clock drift check ==
2019-10-23T13:43:17.1689774Z 
2019-10-23T13:43:17.1795224Z ##[error]Bash exited with code '1'.
2019-10-23T13:43:17.1831074Z ##[section]Starting: Checkout
2019-10-23T13:43:17.1833677Z ==============================================================================
2019-10-23T13:43:17.1833734Z Task         : Get sources
2019-10-23T13:43:17.1833795Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
