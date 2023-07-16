rust
    let opt_def_id = cx.tcx.hir.opt_local_def_id(block_id);
        ...
        let opt_dxn_ext = opt_def_id.and_then(|def_id| {
            cx.tcx.region_maps(def_id).opt_destruction_extent(stmt.node.id())
        });
