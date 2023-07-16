 rust
fn foo(tcx: DepGraph, def_id: DefId, ty: Ty<'tcx>) -> Ty<'tcx> {
    // here, `ty` is the type of `def_id`
    let _task = tcx.dep_graph.in_task(Foo);
    use(ty); // this read would otherwise go unnoticed
    tcx.some_map[&def_id]
}
