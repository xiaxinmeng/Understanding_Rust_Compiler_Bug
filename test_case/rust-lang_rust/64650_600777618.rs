
error[E0597]: `x` does not live long enough
  --> src/main.rs:10:55
   |
10 |         .map(|x| async { vec![()].into_iter().map(|_| x) })
   |                        -------------------------------^---
   |                        |                              |  |
   |                        |                              |  `x` dropped here while still borrowed
   |                        |                              borrowed value does not live long enough
   |                        value captured here by generator

error[E0308]: mismatched types
 --> src/main.rs:4:5
  |
4 |     enter(run())
  |     ^^^^^ one type is more general than the other
  |
  = note: expected type `std::ops::FnOnce<(std::iter::Map<std::vec::IntoIter<()>, [closure@src/main.rs:10:51: 10:56 x:&()]>,)>`
             found type `std::ops::FnOnce<(std::iter::Map<std::vec::IntoIter<()>, [closure@src/main.rs:10:51: 10:56 x:&()]>,)>`

error: aborting due to 2 previous errors
