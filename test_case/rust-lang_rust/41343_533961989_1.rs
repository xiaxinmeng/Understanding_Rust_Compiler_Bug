
error[E0053]: method `index` has an incompatible type for trait
  --> src/lib.rs:15:5
   |
15 |     fn index(self, x: usize) -> &'static T {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected &List<T>, found struct `List`
   |
   = note: expected type `fn(&List<T>, usize) -> &T`
              found type `fn(List<T>, usize) -> &'static T`
