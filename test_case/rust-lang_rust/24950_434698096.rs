
error[E0038]: the trait `Flow` cannot be made into an object
  --> src/main.rs:11:10
   |
11 | impl<'a> LayoutDamageComputation for &'a mut (Flow + 'a) {
   |          ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Flow` cannot be made into an object
   |
   = note: method `foo` has no receiver

error[E0038]: the trait `Flow` cannot be made into an object
  --> src/main.rs:12:5
   |
12 |     fn compute_layout_damage(self) {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Flow` cannot be made into an object
   |
   = note: method `foo` has no receiver

