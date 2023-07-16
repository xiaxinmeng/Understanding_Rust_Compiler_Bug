plain
Building stage0 tool lld-wrapper (x86_64-unknown-linux-gnu)
   Compiling lld-wrapper v0.1.0 (/checkout/src/tools/lld-wrapper)
[RUSTC-TIMING] lld_wrapper test:false 0.337
    Finished release [optimized] target(s) in 0.70s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, tool: "lld-wrapper", path: "src/tools/lld-wrapper", mode: ToolStd, is_optional_tool: false, source_type: InTree, extra_features: ["ld"] } -- 0.718
[TIMING] LldWrapper { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, flavor_feature: "ld" } -- 0.000
Building stage0 tool lld-wrapper (x86_64-unknown-linux-gnu)
   Compiling lld-wrapper v0.1.0 (/checkout/src/tools/lld-wrapper)
[RUSTC-TIMING] lld_wrapper test:false 0.347
    Finished release [optimized] target(s) in 0.52s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, tool: "lld-wrapper", path: "src/tools/lld-wrapper", mode: ToolStd, is_optional_tool: false, source_type: InTree, extra_features: ["ld64"] } -- 0.535
[TIMING] LldWrapper { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, flavor_feature: "ld64" } -- 0.000
[TIMING] StartupObjects { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.000
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.69
   Compiling core v0.0.0 (/checkout/library/core)
---
Building stage1 tool lld-wrapper (x86_64-unknown-linux-gnu)
   Compiling lld-wrapper v0.1.0 (/checkout/src/tools/lld-wrapper)
[RUSTC-TIMING] lld_wrapper test:false 0.420
    Finished release [optimized] target(s) in 0.75s
[TIMING] ToolBuild { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, tool: "lld-wrapper", path: "src/tools/lld-wrapper", mode: ToolStd, is_optional_tool: false, source_type: InTree, extra_features: ["ld"] } -- 0.764
[TIMING] LldWrapper { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, flavor_feature: "ld" } -- 0.000
Building stage1 tool lld-wrapper (x86_64-unknown-linux-gnu)
   Compiling lld-wrapper v0.1.0 (/checkout/src/tools/lld-wrapper)
[RUSTC-TIMING] lld_wrapper test:false 0.407
    Finished release [optimized] target(s) in 0.57s
[TIMING] ToolBuild { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, tool: "lld-wrapper", path: "src/tools/lld-wrapper", mode: ToolStd, is_optional_tool: false, source_type: InTree, extra_features: ["ld64"] } -- 0.587
[TIMING] LldWrapper { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, flavor_feature: "ld64" } -- 0.000
[TIMING] StartupObjects { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "wasm32-unknown-unknown", file: None } } -- 0.000
[TIMING] Libdir { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "wasm32-unknown-unknown", file: None } } -- 0.000
Building stage2 std artifacts (x86_64-unknown-linux-gnu -> wasm32-unknown-unknown)
---
[RUSTC-TIMING] corebenches test:true 0.085
error: function is never used: `catch_unwind_silent`
   --> library/core/tests/array.rs:428:4
    |
428 | fn catch_unwind_silent<F, R>(f: F) -> std::thread::Result<R>
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] coretests test:true 14.903
error: could not compile `core` due to previous error



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "wasm32-unknown-unknown" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--"


Build completed unsuccessfully in 0:20:22
