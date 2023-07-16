rust
fn derp<'a> (cfg: &'a ControlFlowGraph) -> impl Iterator<Item = &'a ControlFlowTarget> {
    cfg.vertices().filter_map(move |vx| cfg.vertex_label(vx))
}
