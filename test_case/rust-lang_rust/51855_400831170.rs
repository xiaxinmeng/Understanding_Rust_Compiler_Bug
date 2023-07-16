rust
impl ConstraintSet {
  fn new() -> Self { .. }

  fn push(&mut self, constraint: Constraint) { .. }

  fn link(&mut self) { /* updates the `next` fields */ }

  fn each_affected_by_dirty(&mut self, r: RegionVid, op: impl FnMut(ConstraintIndex)) {
    /* given that the region `a` has changed, invoke `op` with each constraint `b: a` */
  }
}
