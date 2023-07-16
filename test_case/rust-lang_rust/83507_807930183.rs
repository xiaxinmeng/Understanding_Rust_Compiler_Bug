plain
Successfully built 00a4a6d67f2b
Successfully tagged rust-ci:latest
Built container sha256:00a4a6d67f2b375d29a8d04283affc1a52806be697f93d302bb24d649b72e566
Uploading finished image to https://ci-caches.rust-lang.org/docker/0eee8635c627ca79234c8671635d4c0d7df08a8d5fc20ba0d4a88482f9c5dddee647242a6f516f33086d21c088c4d12f823f40f1161e27d9b9e1fc9749916dc2
upload failed: - to s3://rust-lang-ci-sccache2/docker/0eee8635c627ca79234c8671635d4c0d7df08a8d5fc20ba0d4a88482f9c5dddee647242a6f516f33086d21c088c4d12f823f40f1161e27d9b9e1fc9749916dc2 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking cranelift-frontend v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking cranelift-jit v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking cranelift-object v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0050]: method `add_native_library` has 2 parameters but the declaration in trait `add_native_library` has 3
   |
   |
88 |     fn add_native_library(&mut self, name: rustc_span::symbol::Symbol) {
   |
   |
   = note: `add_native_library` from trait: `fn(&mut Self, rustc_span::Symbol, bool)`
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
  --> src/archive.rs:89:24
   |
   |
89 |         let location = find_library(name, &self.lib_search_paths, self.sess);
   |                        |
   |                        expected 4 arguments
   |
note: function defined here
