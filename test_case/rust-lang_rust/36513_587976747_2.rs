
error[E0599]: no method named `do_b` found for struct `Example` in the current scope
  --> file5.rs:28:7
   |
1  | pub struct Example;
   | -------------------
   | |
   | method `do_b` not found for this
   | doesn't satisfy `Example: Empty`
...
7  | trait B {
   | ------- this trait defines an item `do_b`
...
28 |     e.do_b();
   |       ^^^^ method not found in `Example`
   |
   = note: the method `do_b` exists but the following trait bounds were not satisfied:
           `Example: Empty`
   = help: items from traits can only be used if the trait is implemented and in scope
