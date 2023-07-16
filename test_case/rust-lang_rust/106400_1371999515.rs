plain
[RUSTC-TIMING] rustc_borrowck test:false 4.283
error[E0599]: no method named `get_parent_node` found for struct `rustc_middle::hir::map::Map` in the current scope
   --> compiler/rustc_hir_typeck/src/demand.rs:229:26
    |
229 |         let parent = map.get_parent_node(pat.hir_id);

error[E0599]: no method named `get_parent_node` found for struct `rustc_middle::hir::map::Map` in the current scope
   --> compiler/rustc_hir_typeck/src/demand.rs:288:30
    |
    |
288 |             let parent = map.get_parent_node(binding.hir_id);

[RUSTC-TIMING] rustc_borrowck test:true 4.495
[RUSTC-TIMING] rustc_codegen_llvm test:true 2.571
[RUSTC-TIMING] rustc_codegen_llvm test:false 2.653
