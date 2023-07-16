rust 
if let Some(def_id) = dep_dep_node.extract_def_id(tcx) {
    if is_something_that_corresponds_to_a_dep_node(def_id) {
        // If the node does exist, it should have
        // been marked as either red or green
        bug!(
            "DepNode {:?} should have been \
              pre-marked as red or green but wasn't.",
            dep_dep_node
        )
    } else {
        // This is something that has a valid DefPath 
        // but does not have a corresponding `DepNode`,
        // e.g. a struct field. This branch is hit if
        // a proper item with the given DefPath existed
        // in the previous compilation session.
        return None;
    }
} else {
    // If the node does not exist anymore, we
    // just fail to mark green.
    return None;
}
