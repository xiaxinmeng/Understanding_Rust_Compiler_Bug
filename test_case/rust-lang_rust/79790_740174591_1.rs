
error[E0599]: no method named `bar` found for unit type `()` in the current scope
  --> main.rs:10:7
   |
10 |     x.bar()
   |       ^^^ method not found in `()`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Bar` defines an item `bar`, perhaps you need to implement it
  --> test.rs:3:1
   |
3  | trait Bar {
