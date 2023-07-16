
error: unsatisfied lifetime constraints
  --> $DIR/issue-52742.rs:25:9
   |
LL |  impl Foo<'tcx, '_> {
  |            ----`'tcx` declared here
LL |     fn take_bar(&mut self, b: Bar<'_>) {
   |                                   -- let's call this `'1`
LL |         self.y = b.z
   |         ^^^^^^^^^^^^ requires that `'1` must outlive `'tcx`
