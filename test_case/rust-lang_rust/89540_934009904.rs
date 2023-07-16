plain
Building stage0 tool lld-wrapper (x86_64-unknown-linux-gnu)
   Compiling lld-wrapper v0.1.0 (/checkout/src/tools/lld-wrapper)
[RUSTC-TIMING] lld_wrapper test:false 0.319
    Finished release [optimized] target(s) in 0.61s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, tool: "lld-wrapper", path: "src/tools/lld-wrapper", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: ["ld"] } -- 0.622
[TIMING] LldWrapper { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, flavor_feature: "ld" } -- 0.000
Building stage0 tool lld-wrapper (x86_64-unknown-linux-gnu)
   Compiling lld-wrapper v0.1.0 (/checkout/src/tools/lld-wrapper)
[RUSTC-TIMING] lld_wrapper test:false 0.321
    Finished release [optimized] target(s) in 0.46s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, tool: "lld-wrapper", path: "src/tools/lld-wrapper", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: ["ld64"] } -- 0.477
[TIMING] LldWrapper { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, flavor_feature: "ld64" } -- 0.000
[TIMING] StartupObjects { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-freebsd", file: None } } -- 0.000
Building sanitizers for x86_64-unknown-freebsd
running: "cmake" "/checkout/src/llvm-project/compiler-rt" "-G" "Ninja" "-DCMAKE_C_COMPILER_TARGET=x86_64-unknown-freebsd" "-DCOMPILER_RT_BUILD_BUILTINS=OFF" "-DCOMPILER_RT_BUILD_CRT=OFF" "-DCOMPILER_RT_BUILD_LIBFUZZER=OFF" "-DCOMPILER_RT_BUILD_PROFILE=OFF" "-DCOMPILER_RT_BUILD_SANITIZERS=ON" "-DCOMPILER_RT_BUILD_XRAY=OFF" "-DCOMPILER_RT_DEFAULT_TARGET_ONLY=ON" "-DCOMPILER_RT_USE_LIBCXX=OFF" "-DLLVM_CONFIG_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_SYSTEM_NAME=FreeBSD" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=x86_64-unknown-freebsd11-clang" "-DCMAKE_CXX_COMPILER=x86_64-unknown-freebsd11-clang++" "-DCMAKE_ASM_COMPILER=x86_64-unknown-freebsd11-clang" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-freebsd" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-freebsd -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-freebsd/native/sanitizers" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-freebsd" "-DCMAKE_BUILD_TYPE=Release"
-- The C compiler identification is Clang 6.0.0
---
Building stage0 tool lld-wrapper (x86_64-unknown-freebsd)
   Compiling lld-wrapper v0.1.0 (/checkout/src/tools/lld-wrapper)
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-unknown-freebsd` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-freebsd`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
For more information about this error, try `rustc --explain E0463`.
[RUSTC-TIMING] lld_wrapper test:false 0.037
[RUSTC-TIMING] lld_wrapper test:false 0.037
error: could not compile `lld-wrapper` due to previous error
