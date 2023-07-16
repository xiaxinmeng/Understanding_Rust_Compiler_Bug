rust

fn fold_graph(entry_block: ControlFlowBlock) {
    let mut visited = HashSet::new();
    let mut to_visit = vec![entry_block.clone()];

    // continue until all no nodes are left to visit
    while let Some(current) = to_visit.pop() {
        // return if we have already visited the current node
        if visited.contains(&current) {
            return;
        }

        // flag for whether we mutated the current node and therefore need to attempt folding again
        let mut did_change = false;

        // check if the current node jumps unconditionally to the child (target) node
        let terminator = current.terminator();
        if let Terminator::Unconditional { target } = terminator {
            let parents = target.parents();

            // check if the child has only 1 parent (the current node)
            if parents.len() == 1 {
                // smoke test that the child's parent is the current node
                assert_eq!(current, parents[0]);
                // smoke test that the child is not the current node
                assert_ne!(current, target);

                did_change = true;

                // move all statements and the terminator from the child to the current node (child will be deallocated automatically)
                let mut statements = current.statements();
                statements.append(&mut target.statements());
                current.set_statements(statements);
                ControlFlowBlock::set_terminator(&current, target.terminator());
            }
        }

        // if we modified the current node, visit it again, otherwise mark it as visited
        if did_change {
            // restart
            to_visit = vec![entry_block.clone()];
            break;
        } else {
            visited.insert(current.clone());
        }

        // push children to visit
        to_visit.extend(current.terminator().targets());
    }
}
