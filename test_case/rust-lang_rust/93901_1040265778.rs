plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0046]: not all trait items implemented, missing: `exported_symbol_means_used_symbol`
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1577:1
     |
189  |     fn exported_symbol_means_used_symbol(&self) -> bool;
     |     ---------------------------------------------------- `exported_symbol_means_used_symbol` from trait
...
1577 | impl<'a> Linker for PtxLinker<'a> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `exported_symbol_means_used_symbol` in implementation
error[E0046]: not all trait items implemented, missing: `exported_symbol_means_used_symbol`
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1676:1
     |
189  |     fn exported_symbol_means_used_symbol(&self) -> bool;
189  |     fn exported_symbol_means_used_symbol(&self) -> bool;
     |     ---------------------------------------------------- `exported_symbol_means_used_symbol` from trait
...
1676 | impl<'a> Linker for BpfLinker<'a> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `exported_symbol_means_used_symbol` in implementation
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
For more information about this error, try `rustc --explain E0046`.
error: could not compile `rustc_codegen_ssa` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
