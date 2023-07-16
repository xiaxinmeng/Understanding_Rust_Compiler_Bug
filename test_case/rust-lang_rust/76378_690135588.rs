plain
      Memory: 12 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMWVz+YuxLFe

hw.ncpu: 4
hw.byteorder: 1234
hw.memsize: 12884901888
---
[TIMING] Compiletest { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None }, mode: "debuginfo", suite: "debuginfo", path: "src/test/debuginfo", compare_mode: None } -- 0.435
[TIMING] Compiletest { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None }, mode: "ui", suite: "ui-fulldeps", path: "src/test/ui-fulldeps", compare_mode: None } -- 0.489
[TIMING] Compiletest { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None }, mode: "rustdoc", suite: "rustdoc", path: "src/test/rustdoc", compare_mode: None } -- 0.371
[TIMING] Compiletest { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None }, mode: "pretty", suite: "pretty", path: "src/test/pretty", compare_mode: None } -- 0.390
thread 'main' panicked at 'fs::read_dir(builder.src.join("src/doc/book/redirects")) failed with No such file or directory (os error 2)', src/bootstrap/doc.rs:225:21
failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:00:07
== clock drift check ==
  local time: Thu Sep 10 10:13:41 UTC 2020
  local time: Thu Sep 10 10:13:41 UTC 2020
  network time: Thu, 10 Sep 2020 10:13:41 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (2811) (sccache)
Terminate orphan process: pid (1817) (Python)
