plain
   |
57 |         llvm_component = "x86",
   |         ^^^^^^^^^^^^^^
   |
   = note: `-D unexpected-cfgs` implied by `-D warnings`
error: unexpected `cfg` condition name
  --> compiler/rustc_llvm/src/lib.rs:65:9
   |
65 |         llvm_component = "arm",
---

error: unexpected `cfg` condition name
  --> compiler/rustc_llvm/src/lib.rs:81:9
   |
81 |         llvm_component = "amdgpu",

error: unexpected `cfg` condition name
  --> compiler/rustc_llvm/src/lib.rs:89:9
   |
   |
89 |         llvm_component = "avr",

error: unexpected `cfg` condition name
  --> compiler/rustc_llvm/src/lib.rs:97:9
   |
---

error: unexpected `cfg` condition name
   --> compiler/rustc_llvm/src/lib.rs:129:9
    |
129 |         llvm_component = "jsbackend",

error: unexpected `cfg` condition name
   --> compiler/rustc_llvm/src/lib.rs:135:9
    |
---

error: unexpected `cfg` condition name
   --> compiler/rustc_llvm/src/lib.rs:182:9
    |
182 |         llvm_component = "bpf",

error: could not compile `rustc_llvm` due to 17 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_llvm` due to 17 previous errors
