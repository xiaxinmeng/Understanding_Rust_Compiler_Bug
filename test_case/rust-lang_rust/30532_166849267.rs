 rust
fn foo(tcx: DepGraph, key: Key) -> Ty<'tcx> {
    let _task = tcx.dep_graph.in_task(Foo);
    tcx.some_map[&key]
}
