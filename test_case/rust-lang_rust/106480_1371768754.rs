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
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0599]: no method named `get_parent_node` found for struct `rustc_middle::hir::map::Map` in the current scope
   --> compiler/rustc_hir_typeck/src/demand.rs:229:26
    |
229 |         let parent = map.get_parent_node(pat.hir_id);

error[E0599]: no method named `get_parent_node` found for struct `rustc_middle::hir::map::Map` in the current scope
   --> compiler/rustc_hir_typeck/src/demand.rs:288:30
    |
    |
288 |             let parent = map.get_parent_node(binding.hir_id);

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_hir_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
