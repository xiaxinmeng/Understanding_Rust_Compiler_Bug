console
error[E0499]: cannot borrow `*stuff` as mutable more than once at a time
  --> src/lib.rs:44:57
   |
39 |     let mut one = One(stuff);
   |                       ----- first mutable borrow occurs here
...
42 |             one = match one.step() {
   |             --- first borrow might be used here, when `one` is dropped and runs the `Drop` code for type `One`
43 |                 Either::Left(x) => break x,
44 |                 Either::Right(save) => save.resume(grow(stuff)),
   |                                                         ^^^^^ second mutable borrow occurs here
