
fn query_optimised_mir(did: DefId) { if is_function(did) { query_inlined_mir(did) } else { query_constevaluated_mir(did) }
fn query_inlined_mir(did: DefId) { ... do inlining on mir returned by `query_some_other_mir` ... }
fn query_some_other_mir(did: DefId) { fn get_base_mir(did) }
