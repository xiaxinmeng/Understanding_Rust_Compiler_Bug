rust
witnesses.retain(|p| {
    if let rustc_middle::thir::PatKind::Variant { adt_def, variant_index, .. } = &*p.kind {
        let did = adt_def.variants[*variant_index].def_id;

        let x = cx.tcx.lookup_stability(did);
        if let Some(Stability { feature, level: StabilityLevel::Unstable { .. }, .. }) = x {
            return cx.tcx.stability().active_features.contains(&feature);
        }
    }
    true
});
