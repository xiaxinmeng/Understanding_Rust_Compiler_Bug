
error: unsatisfied lifetime constraints
  --> src/main.rs:16:9
   |
15 |     fn take_bar(&mut self, b: Bar<'_>) {
   |                 ---------         -- let's call this `'1`
   |                 |
   |                 has type `&mut Foo<'_, '2>`
16 |         self.y = b.z
   |         ^^^^^^^^^^^^ requires that `'1` must outlive `'2`
