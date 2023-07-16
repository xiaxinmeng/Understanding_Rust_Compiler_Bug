plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMt/TeasCfea

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
    Finished release [optimized] target(s) in 1.18s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None }, tool: "html-checker", path: "src/tools/html-checker", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 1.204
[TIMING] HtmlChecker { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None } } -- 0.000
Running HTML checker...
=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/core/index.html` (error code: 1) <=
line 39 column 36 - Warning: <a> attribute "rev" not allowed for HTML5 (MISMATCHED_ATTRIBUTE_WARN)
Error: "HTML check failed: 1 errors"
Done! Read 26513 files...


