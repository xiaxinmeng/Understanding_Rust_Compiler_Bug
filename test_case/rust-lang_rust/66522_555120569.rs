plain
2019-11-18T16:57:16.4584905Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-18T16:57:16.4787044Z ##[command]git config gc.auto 0
2019-11-18T16:57:16.4866999Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-18T16:57:16.4927323Z ##[command]git config --get-all http.proxy
2019-11-18T16:57:16.5075827Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66522/merge:refs/remotes/pull/66522/merge
---
2019-11-18T17:25:17.2381771Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2019-11-18T17:25:19.2776194Z error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
2019-11-18T17:25:19.2777071Z 
2019-11-18T17:25:19.2777185Z Caused by:
2019-11-18T17:25:19.2785502Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/release/build/rustc_llvm-e3ad2447f0c5c3b9/build-script-build` (exit code: 1)
2019-11-18T17:25:19.2785736Z --- stdout
2019-11-18T17:25:19.2785993Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
2019-11-18T17:25:19.2786213Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
2019-11-18T17:25:19.2786904Z cargo:rerun-if-changed=/usr/lib/llvm-6.0/bin/llvm-config
2019-11-18T17:25:19.2787183Z cargo:rerun-if-env-changed=LLVM_CONFIG
2019-11-18T17:25:19.2787419Z cargo:rustc-cfg=llvm_component="aarch64"
2019-11-18T17:25:19.2787640Z cargo:rustc-cfg=llvm_component="amdgpu"
2019-11-18T17:25:19.2787885Z cargo:rustc-cfg=llvm_component="arm"
2019-11-18T17:25:19.2788288Z cargo:rustc-cfg=llvm_component="asmparser"
2019-11-18T17:25:19.2788576Z cargo:rustc-cfg=llvm_component="bitreader"
2019-11-18T17:25:19.2788829Z cargo:rustc-cfg=llvm_component="bitwriter"
2019-11-18T17:25:19.2789069Z cargo:rustc-cfg=llvm_component="hexagon"
2019-11-18T17:25:19.2789302Z cargo:rustc-cfg=llvm_component="instrumentation"
2019-11-18T17:25:19.2789543Z cargo:rustc-cfg=llvm_component="ipo"
2019-11-18T17:25:19.2789767Z cargo:rustc-cfg=llvm_component="linker"
2019-11-18T17:25:19.2790195Z cargo:rustc-cfg=llvm_component="lto"
2019-11-18T17:25:19.2790423Z cargo:rustc-cfg=llvm_component="mips"
2019-11-18T17:25:19.2790634Z cargo:rustc-cfg=llvm_component="msp430"
2019-11-18T17:25:19.2790836Z cargo:rustc-cfg=llvm_component="nvptx"
2019-11-18T17:25:19.2791041Z cargo:rustc-cfg=llvm_component="powerpc"
2019-11-18T17:25:19.2791269Z cargo:rustc-cfg=llvm_component="sparc"
2019-11-18T17:25:19.2791478Z cargo:rustc-cfg=llvm_component="systemz"
2019-11-18T17:25:19.2791693Z cargo:rustc-cfg=llvm_component="x86"
2019-11-18T17:25:19.2791923Z cargo:rerun-if-changed-env=LLVM_RUSTLLVM
2019-11-18T17:25:19.2792144Z cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
2019-11-18T17:25:19.2792367Z cargo:rerun-if-changed=../rustllvm/Linker.cpp
2019-11-18T17:25:19.2792602Z cargo:rerun-if-changed=../rustllvm/.editorconfig
2019-11-18T17:25:19.2792824Z cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
2019-11-18T17:25:19.2793044Z cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
2019-11-18T17:25:19.2793446Z cargo:rerun-if-changed=../rustllvm/README
2019-11-18T17:25:19.2793852Z cargo:rerun-if-changed=../rustllvm/rustllvm.h
2019-11-18T17:25:19.2794079Z TARGET = Some("x86_64-unknown-linux-gnu")
2019-11-18T17:25:19.2794148Z OPT_LEVEL = Some("2")
2019-11-18T17:25:19.2794377Z HOST = Some("x86_64-unknown-linux-gnu")
2019-11-18T17:25:19.2794615Z CXX_x86_64-unknown-linux-gnu = Some("sccache c++")
2019-11-18T17:25:19.2794894Z CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
2019-11-18T17:25:19.2794978Z CRATE_CC_NO_DEFAULTS = None
2019-11-18T17:25:19.2795028Z DEBUG = Some("false")
2019-11-18T17:25:19.2795075Z CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
2019-11-18T17:25:19.2797782Z running: "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-6.0/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-b63a5c43d180ed4c/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp"
2019-11-18T17:25:19.2798170Z cargo:warning=../rustllvm/PassWrapper.cpp:21:62: fatal error: llvm/Transforms/Instrumentation/AddressSanitizer.h: No such file or directory
2019-11-18T17:25:19.2798247Z cargo:warning=compilation terminated.
2019-11-18T17:25:19.2798330Z 
2019-11-18T17:25:19.2798630Z --- stderr
2019-11-18T17:25:19.2798663Z 
2019-11-18T17:25:19.2798689Z 
2019-11-18T17:25:19.2798689Z 
2019-11-18T17:25:19.2800912Z error occurred: Command "sccache" "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-6.0/include" "-std=c++0x" "-fuse-ld=gold" "-Wl,--no-keep-files-mapped" "-Wl,--no-map-whole-files" "-fPIC" "-fvisibility-inlines-hidden" "-Werror=date-time" "-std=c++11" "-Wall" "-W" "-Wno-unused-parameter" "-Wwrite-strings" "-Wcast-qual" "-Wno-missing-field-initializers" "-pedantic" "-Wno-long-long" "-Wno-maybe-uninitialized" "-Wdelete-non-virtual-dtor" "-Wno-comment" "-ffunction-sections" "-fdata-sections" "-O2" "-DNDEBUG" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-b63a5c43d180ed4c/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp" with args "c++" did not execute successfully (status code exit code: 1).
2019-11-18T17:25:19.2801194Z 
2019-11-18T17:25:19.2801218Z 
2019-11-18T17:25:19.2803072Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json-render-diagnostics"
2019-11-18T17:25:19.2803158Z expected success, got: exit code: 101
2019-11-18T17:25:19.2803158Z expected success, got: exit code: 101
2019-11-18T17:25:19.2809692Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-18T17:25:19.2809770Z Build completed unsuccessfully in 0:21:42
2019-11-18T17:25:19.2871919Z == clock drift check ==
2019-11-18T17:25:19.2887418Z   local time: Mon Nov 18 17:25:19 UTC 2019
2019-11-18T17:25:19.5679697Z   network time: Mon, 18 Nov 2019 17:25:19 GMT
2019-11-18T17:25:19.5683488Z == end clock drift check ==
2019-11-18T17:25:22.0469510Z 
2019-11-18T17:25:22.0540261Z ##[error]Bash exited with code '1'.
2019-11-18T17:25:22.0574454Z ##[section]Starting: Checkout
2019-11-18T17:25:22.0577203Z ==============================================================================
2019-11-18T17:25:22.0577264Z Task         : Get sources
2019-11-18T17:25:22.0577333Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
