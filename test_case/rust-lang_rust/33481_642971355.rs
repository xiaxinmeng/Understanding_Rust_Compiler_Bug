
error[E0271]: type mismatch resolving `<(i32, f32) as Sameness>::Same == False`
  --> src/main.rs:49:5
   |
40 | fn not_same<A, B>() where (A, B): NotSame {}
   |                                   ------- required by this bound in `not_same`
...
49 |     not_same::<i32, f32>();
   |     ^^^^^^^^^^^^^^^^^^^^ expected associated type, found struct `False`
   |
   = note: expected associated type `<(i32, f32) as Sameness>::Same`
                       found struct `False`
   = help: consider constraining the associated type `<(i32, f32) as Sameness>::Same` to `False` or calling a method that returns `<(i32, f32) as Sameness>::Same`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
   = note: required because of the requirements on the impl of `NotSame` for `(i32, f32)`
