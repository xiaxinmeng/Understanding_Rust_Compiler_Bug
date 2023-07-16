plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMNCqBXpwoxV

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
    Finished release [optimized] target(s) in 1.10s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None }, tool: "html-checker", path: "src/tools/html-checker", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 1.124
[TIMING] HtmlChecker { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None } } -- 0.000
Running HTML checker...
=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/trait.Concat.html` (error code: 2) <=
line 11 column 11 - Warning: <t:> attribute "clone," lacks value (MISSING_ATTR_VALUE)
line 11 column 11 - Warning: <t:> missing '>' for end of tag (UNEXPECTED_GT)
line 11 column 11 - Error: <t:> is not recognized! (UNKNOWN_ELEMENT)
This document has errors that must be fixed before
using HTML Tidy to generate a tidied up version.


=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/slice/trait.Concat.html` (error code: 2) <=
line 11 column 11 - Warning: <t:> attribute "clone," lacks value (MISSING_ATTR_VALUE)
line 11 column 11 - Warning: <t:> missing '>' for end of tag (UNEXPECTED_GT)
line 11 column 11 - Error: <t:> is not recognized! (UNKNOWN_ELEMENT)
This document has errors that must be fixed before
using HTML Tidy to generate a tidied up version.


=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/proc_macro/struct.Group.html` (error code: 1) <=
line 13 column 66 - Warning: unescaped & or unknown entity "&self" (UNKNOWN_ENTITY)
line 15 column 71 - Warning: unescaped & or unknown entity "&self" (UNKNOWN_ENTITY)
line 17 column 72 - Warning: unescaped & or unknown entity "&self" (UNKNOWN_ENTITY)
Error: "HTML check failed: 15 errors"
Done! Read 26507 files...


