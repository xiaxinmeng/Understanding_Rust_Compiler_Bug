plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMUMHGzT/1lC

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
[TIMING] Assemble { target_compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } } } -- 0.000
[TIMING] Sysroot { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } } } -- 0.000
[TIMING] Libdir { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None } } -- 0.000
Building stage0 tool unstable-book-gen (x86_64-apple-darwin)
error: failed to run `rustc` to learn about target-specific information
failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap dist --stage 2
Caused by:
Build completed unsuccessfully in 0:00:01
Build completed unsuccessfully in 0:00:01
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names --cfg=bootstrap -Zmacro-backtrace -Zosx-rpath-install-name '-Clink-args=-Wl,-rpath,@loader_path/../lib' -Zrun-dsymutil=no -Ztls-model=initial-exec --target x86_64-apple-darwin --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit code: 1)
