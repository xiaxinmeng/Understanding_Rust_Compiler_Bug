
   |
2  | pub struct Example;
   | ------------------- method `do_b` not found for this
...
29 |     e.do_b();
   |       ^^^^ method not found in `Example`
   |
   = note: the method `do_b` exists but the following trait bounds were not satisfied:
           `Example : B`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `do_b`, perhaps you need to implement it:
           candidate #1: `B`
