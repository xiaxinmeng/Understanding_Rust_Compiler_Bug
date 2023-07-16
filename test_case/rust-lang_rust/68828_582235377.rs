rust
struct Cycle {
    root: LocalDefId,
    members: FxHashSet<LocalDefId>,
}
fn mir_inliner_cycle(tcx: TyCtxt<'tcx>, root: LocalDefId) -> Option<&'tcx Cycle> {
    let root_hash = tcx.def_path_hash(root);

    let mut cycle = None;

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(caller) = queue.pop_front() {
        for callee in tcx.mir_inliner_callees(caller) {
            if callee == def_id {
                cycle = Some(Cycle {
                    root,
                    // TODO: accumulate cycle members.
                    // Can probably use a stack instead of a queue, emulating
                    // the runtime callstack, but this example is large as is.
                    members: FxHashSet::new(),
                });
                continue;
            }

            if tcx.def_path_hash(callee) < root_hash {
                // We may share a cycle with `callee`, in which case
                // it's "closer" to the cycle's real root (lower hash).
                if let Some(callee_cycle) = tcx.mir_inliner_cycle(callee) {
                    if callee_cycle.members.contains(&root) {
                        return Some(callee_cycle);
                    }
                }
            } else {
                // Keep exploring areas of the callgraph that could
                // contain a cycle that we're the root of
                // (i.e. all other nodes have higher hash).
                queue.push_back(callee);
            }
        }
    }

    cycle
}
