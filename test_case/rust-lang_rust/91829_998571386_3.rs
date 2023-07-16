
error[E0308]: mismatched types
  --> src/main.rs:18:20
   |
6  | ) -> impl FnMut(A) -> C {
   |      ------------------
   |      |
   |      the expected opaque type
   |      the found opaque type
...
18 |     takes_composed(&b);
   |                    ^^ one type is more general than the other
   |
   = note: expected associated type `<impl FnMut<(&usize,)> as FnOnce<(&usize,)>>::Output`
              found associated type `<impl FnMut<(&usize,)> as FnOnce<(&usize,)>>::Output`
