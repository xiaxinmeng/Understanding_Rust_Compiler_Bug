rust
let mut results = BitSet::empty(body.basic_blocks());
let mut queue: VecDeque<_> = self_calls.iter().collect();
while let Some(bb) = queue.pop_front() {
    if results.contains(bb) {
        // Already propagated.
        continue;
    }

    let always_reaches_self_call = match body[bb].terminator.kind {
        TerminatorKind::Call(..) if self_calls.contains(bb) => true,

        TerminatorKind::SwitchInt(branches)
            => branches.iter().all(|&target| results.contains(target)),

        TerminatorKind::Goto { target }
        // ... and all other single target variants, ignoring any cleanup edges
            => result.contains(target),

        // We only ever go backwards through the CFG, these should be impossible to hit.
        TerminatorKind::Return | TerminatorKind::Resume => unreachable!(),
    };

    if always_reaches_self_call {
        // This is a newly confirmed-to-always-reach-self-call block.
        assert!(results.insert(bb));

        // Propagate backwards through the CFG.
        queue.extend(body.predecessors(bb));
    }
}
let fn_always_reaches_self_call = results[START_BLOCK];
