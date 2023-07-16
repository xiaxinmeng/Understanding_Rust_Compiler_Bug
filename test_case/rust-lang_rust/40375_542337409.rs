
error[E0599]: no method named `foo` found for type `std::slice::Iter<'_, ({integer}, char)>` in the current scope
  --> src/main.rs:15:23
   |
15 |     for x in v.iter().foo() {
   |                       ^^^ method not found in `std::slice::Iter<'_, ({integer}, char)>`
   |
   = note: the method `foo` exists but the following trait bounds were not satisfied:
           `&mut std::slice::Iter<'_, ({integer}, char)> : Foo`
           `&std::slice::Iter<'_, ({integer}, char)> : Foo`
           `std::slice::Iter<'_, ({integer}, char)> : Foo`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `foo`, perhaps you need to implement it:
           candidate #1: `Foo`
