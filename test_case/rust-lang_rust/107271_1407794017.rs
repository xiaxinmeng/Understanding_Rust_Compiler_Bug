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
 Documenting rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: Rust code block is empty
  --> compiler/rustc_mir_transform/src/elaborate_drops.rs:38:5
   |
38 |   /// 