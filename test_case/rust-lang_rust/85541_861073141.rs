plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMVR+Fv+Dv1G

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None }, tool: "lint-docs", path: "src/tools/lint-docs", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 14.140
[TIMING] LintDocs { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None } } -- 0.000


command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/lint-docs" "--src" "/Users/runner/work/rust/rust/compiler" "--out" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/md-doc/rustc/src/lints" "--rustc" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustc-target" "x86_64-apple-darwin"
expected success, got: signal: 9

failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap dist
Build completed unsuccessfully in 1:06:01
