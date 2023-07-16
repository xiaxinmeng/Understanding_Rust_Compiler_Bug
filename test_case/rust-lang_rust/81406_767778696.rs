plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VME8FrTDtXub

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
[TIMING] StartupObjects { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "aarch64-apple-ios", file: None } } -- 0.000
[TIMING] Sanitizers { target: TargetSelection { triple: "aarch64-apple-ios", file: None } } -- 0.000
[TIMING] Libdir { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "aarch64-apple-ios", file: None } } -- 0.000
Building stage2 std artifacts (x86_64-apple-darwin -> aarch64-apple-ios)
error: failed to run `rustc` to learn about target-specific information
failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap dist
Build completed unsuccessfully in 1:33:49
Caused by:
