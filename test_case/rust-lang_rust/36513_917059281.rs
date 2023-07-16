
error[E0599]: the method `do_b` exists for struct `Example`, but its trait bounds were not satisfied
  --> src/main.rs:29:7
   |
2  | pub struct Example;
   | -------------------
   | |
   | method `do_b` not found for this
   | doesn't satisfy `Example: B`
   | doesn't satisfy `Example: Empty`
...
29 |     e.do_b();
   |       ^^^^ method cannot be called on `Example` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `Example: Empty`
           which is required by `Example: B`
