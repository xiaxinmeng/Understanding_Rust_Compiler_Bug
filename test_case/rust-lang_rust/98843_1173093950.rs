plain
   |
56 |         llvm_component = "x86",
   |         ^^^^^^^^^^^^^^
   |
   = note: `-D unexpected-cfgs` implied by `-D warnings`
error: unexpected `cfg` condition name
  --> compiler/rustc_llvm/src/lib.rs:64:9
   |
64 |         llvm_component = "arm",
---

error: unexpected `cfg` condition name
  --> compiler/rustc_llvm/src/lib.rs:80:9
   |
80 |         llvm_component = "amdgpu",

error: unexpected `cfg` condition name
  --> compiler/rustc_llvm/src/lib.rs:88:9
   |
   |
88 |         llvm_component = "avr",

error: unexpected `cfg` condition name
  --> compiler/rustc_llvm/src/lib.rs:96:9
   |
---

error: unexpected `cfg` condition name
   --> compiler/rustc_llvm/src/lib.rs:128:9
    |
128 |         llvm_component = "jsbackend",

error: unexpected `cfg` condition name
   --> compiler/rustc_llvm/src/lib.rs:134:9
    |
---

error: unexpected `cfg` condition name
   --> compiler/rustc_llvm/src/lib.rs:181:9
    |
181 |         llvm_component = "bpf",

   Compiling cstr v0.2.8
error: could not compile `rustc_llvm` due to 17 previous errors
warning: build failed, waiting for other jobs to finish...
