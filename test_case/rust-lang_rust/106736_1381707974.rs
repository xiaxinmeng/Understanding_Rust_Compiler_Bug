rust
pub fn build_dep_graph(...) {
    ....
    Some(DepGraph::new(
        &sess.prof,
        prev_graph,
        prev_work_products,
        encoder,
        sess.opts.unstable_opts.query_dep_graph,
        sess.opts.unstable_opts.incremental_info,
    ))
