rust
impl Deref for ConstraintSet {
    type Target = IndexVec<ConstraintIndex, OutlivesConstraint>;

    fn deref(&self) -> &Self::Target { &self.constriants }
}
