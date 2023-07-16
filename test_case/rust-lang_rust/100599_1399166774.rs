rust
fn verify_no_symbol_conflicts(&self, root: &CrateRoot) -> Result<(), CrateError> {
    // Check for (potential) conflicts with the local crate
    if self.sess.local_stable_crate_id() == root.stable_crate_id() {
        return Err(CrateError::SymbolConflictsCurrent(root.name()));
    }

    // Check for conflicts with any crate loaded so far
    for (_, other) in self.cstore.iter_crate_data() {
        // Same stable crate id but different SVH
        if other.stable_crate_id() == root.stable_crate_id() && other.hash() != root.hash() {
            return Err(CrateError::SymbolConflictsOthers(root.name()));
        }
    }

    Ok(())
}
