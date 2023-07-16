rust
assert!(
    mir.yield_ty.is_some() && universal_regions.yield_ty.is_some() ||
    mir.yield_ty.is_nome() && universal_regions.yield_ty.is_nome()
);
if let Some(mir_yield_ty) = mir.yield_ty {
    let ur_yield_ty = universal_regions.yield_ty.unwrap();
    self.equate_normalized_input_or_output(start_position, ur_yield_ty, mir_yield_ty);
}
