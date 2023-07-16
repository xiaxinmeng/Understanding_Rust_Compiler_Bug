rust        
let is_write_macro = sugg_span
    .ctxt()
    .outer_expn_data()
    .macro_def_id
    .map_or(false, |did| tcx.is_diagnostic_item(sym::write_macro, did));

