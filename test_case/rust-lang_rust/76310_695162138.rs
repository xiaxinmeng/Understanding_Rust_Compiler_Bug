console
    Finished release [optimized] target(s) in 1m 05s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None }, tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 65.034
[TIMING] Tidy { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None } } -- 0.000
tidy check


command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/tidy" "/Users/runner/work/rust/rust" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo"
expected success, got: signal: 9


failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:01:13
