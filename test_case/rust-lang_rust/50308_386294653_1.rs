
error[E0277]: the trait bound `L: AsIndex` is not satisfied
  --> src/main.rs:29:27
   |
29 |     left: [Option<usize>; <L as AsIndex>::MAX + 1],
   |                           ^^^^^^^^^^^^^^^^^^^ the trait `AsIndex` is not implemented for `L`
   |
   = help: consider adding a `where L: AsIndex` bound
note: required by `AsIndex::MAX`
  --> src/main.rs:5:5
   |
5  |     const MAX: usize;
   |     ^^^^^^^^^^^^^^^^^

error[E0277]: the trait bound `R: AsIndex` is not satisfied
  --> src/main.rs:30:28
   |
30 |     right: [Option<usize>; <R as AsIndex>::MAX + 1],
   |                            ^^^^^^^^^^^^^^^^^^^ the trait `AsIndex` is not implemented for `R`
   |
   = help: consider adding a `where R: AsIndex` bound
note: required by `AsIndex::MAX`
  --> src/main.rs:5:5
   |
5  |     const MAX: usize;
   |     ^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors
