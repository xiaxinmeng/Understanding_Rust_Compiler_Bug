plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMyFXmVnVEue

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None }, tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 86.074
[TIMING] UnstableBookGen { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None } } -- 0.000


command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/unstable-book-gen" "/Users/runner/work/rust/rust/library" "/Users/runner/work/rust/rust/compiler" "/Users/runner/work/rust/rust/src" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/md-doc/unstable-book"
expected success, got: signal: 9

failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap dist
Build completed unsuccessfully in 0:01:27
