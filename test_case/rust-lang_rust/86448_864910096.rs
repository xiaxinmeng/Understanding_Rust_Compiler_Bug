rust
    if let Some(did) = for_.def_id() {
        if cx.tcx.get_attrs(did).lists(sym::doc).has_word(sym::hidden) {
            return
        }
    }
