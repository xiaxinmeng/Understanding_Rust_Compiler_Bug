rust
        visibility: |tcx, id| {
            let cdata = CStore::from_tcx(tcx).get_crate_data(id.krate);
            cdata.get_visibility(id.index)
        },
