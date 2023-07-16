
error[E0599]: the method `do_b` exists for struct `Example`, but its trait bounds were not satisfied
  --> src/main.rs:29:7
   |
2  | pub struct Example;
   | ------------------
   | |
   | method `do_b` not found for this struct
   | doesn't satisfy `Example: B`
   | doesn't satisfy `Example: Empty`
...
29 |     e.do_b();
   |       ^^^^ method cannot be called on `Example` due to unsatisfied trait bounds
   |
note: trait bound `Example: Empty` was not satisfied
  --> src/main.rs:14:31
   |
14 | impl <T> B for T where T: A + Empty
   |          -     -              ^^^^^ unsatisfied trait bound introduced here
note: the following trait must be implemented
  --> src/main.rs:12:1
   |
12 | trait Empty {}
   | ^^^^^^^^^^^
