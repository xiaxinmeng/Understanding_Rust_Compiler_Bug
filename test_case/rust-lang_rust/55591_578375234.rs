
error[E0277]: the trait bound `std::vec::Vec<i32>: Producer` is not satisfied
  --> src/main.rs:26:13
   |
17 | fn dostuff<IP, P>(arg: IP)
   |    -------
...
20 |     P: Producer,
   |        -------- required by this bound in `dostuff`
...
26 |     dostuff(Vec::<i32>::new());
   |             ^^^^^^^^^^^^^^^^^ the trait `Producer` is not implemented for `std::vec::Vec<i32>`
