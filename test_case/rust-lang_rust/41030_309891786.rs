
error[E0599]: no method named `min` found for type `T` in the current scope
  --> test.rs:18:19
   |
18 |                 e.min(n)
   |                   ^^^
   |
   = note: the method `min` exists but the following trait bounds were not satisfied:
           `T : std::cmp::Ord`
           `T : std::cmp::Ord`
           `T : std::iter::Iterator`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following traits define an item `min`, perhaps you need to implement one of them:
           candidate #1: `Minimum`
           candidate #2: `core::num::Float`
           candidate #3: `std::cmp::Ord`
           candidate #4: `std::iter::Iterator`
