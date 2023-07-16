rust
fn has_custom_linkage<'tcx>(tcx: TyCtxt<'tcx>, def_id: LocalDefId) -> bool {
    // Anything which has custom linkage gets thrown on the worklist no
    // matter where it is in the crate, along with "special std symbols"
    // which are currently akin to allocator symbols.
    let codegen_attrs = tcx.codegen_fn_attrs(def_id);
    tcx.def_kind(def_id).has_codegen_attrs()
    && (codegen_attrs.contains_extern_indicator()
        || codegen_attrs.flags.contains(CodegenFnAttrFlags::RUSTC_STD_INTERNAL_SYMBOL)
        // FIXME(nbdd0121): `#[used]` are marked as reachable here so it's picked up by
        // `linked_symbols` in cg_ssa. They won't be exported in binary or cdylib due to their
        // `SymbolExportLevel::Rust` export level but may end up being exported in dylibs.
        || codegen_attrs.flags.contains(CodegenFnAttrFlags::USED)
        || codegen_attrs.flags.contains(CodegenFnAttrFlags::USED_LINKER))
}
 