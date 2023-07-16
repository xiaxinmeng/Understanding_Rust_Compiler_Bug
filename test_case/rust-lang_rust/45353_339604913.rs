rust
profq_msg!(tcx, ProfileQueriesMsg::ProviderBegin); 
let res = tcx.cycle_check(span, Query::$name(key), || { 
    tcx.sess.diagnostic().track_diagnostics(|| { 
        if dep_node.is_eval_always() {
            tcx.dep_graph.with_eval_always_task(dep_node, 
                                                tcx, 
                                                key, 
                                                Self::compute_result) 
        } else {
            tcx.dep_graph.with_task(dep_node, 
                                    tcx, 
                                    key, 
                                    Self::compute_result) 
        }
    }) 
})?; 
profq_msg!(tcx, ProfileQueriesMsg::ProviderEnd);
