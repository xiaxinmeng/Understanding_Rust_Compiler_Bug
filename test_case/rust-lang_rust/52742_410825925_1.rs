
error: unsatisfied lifetime constraints
  --> $DIR/issue-52742.rs:25:9
   |
LL |  impl Foo<'_, '_> {
  |            -- let's call this `'2`
LL |     fn take_bar(&mut self, b: Bar<'_>) {
   |                                   -- let's call this `'1`
LL |         self.y = b.z
   |         ^^^^^^^^^^^^ requires that `'1` must outlive `'2`
