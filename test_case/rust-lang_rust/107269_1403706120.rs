plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error: unexpected `cfg` condition value
  --> src/cranelift_native.rs:18:13
   |
18 | #![cfg_attr(feature = "clippy", plugin(clippy(conf_file = "../../clippy.toml")))]
   |
   |
   = note: expected values for `feature` are: __check_build_system_using_ra, cranelift-jit, inline_asm, jit, libloading, unstable-features
   = note: `-D unexpected-cfgs` implied by `-D warnings`
error: unexpected `cfg` condition value
  --> src/cranelift_native.rs:19:13
   |
   |
19 | #![cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
   |
   |
   = note: expected values for `feature` are: __check_build_system_using_ra, cranelift-jit, inline_asm, jit, libloading, unstable-features
error: unexpected `cfg` condition value
  --> src/cranelift_native.rs:21:5
   |
21 |     feature = "cargo-clippy",
21 |     feature = "cargo-clippy",
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `feature` are: __check_build_system_using_ra, cranelift-jit, inline_asm, jit, libloading, unstable-features
error: could not compile `rustc_codegen_cranelift` due to 3 previous errors
Build completed unsuccessfully in 0:01:56
