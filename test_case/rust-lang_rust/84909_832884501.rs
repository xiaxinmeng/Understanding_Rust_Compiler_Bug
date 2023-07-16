plain
[RUSTC-TIMING] cargo test:false 87.874
error: build failed
command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "build" "--target" "aarch64-pc-windows-msvc" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\rust\\rust\\src/tools/rls\\Cargo.toml" "--features" "clippy, rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'Unable to build RLS', src\bootstrap\dist.rs:44:9
[TIMING] ToolBuild { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } }, target: TargetSelection { triple: "aarch64-pc-windows-msvc", file: None }, tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: ["clippy"] } -- 118.526
[TIMING] Rls { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } }, target: TargetSelection { triple: "aarch64-pc-windows-msvc", file: None }, extra_features: [] } -- 0.000
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 0:58:29
