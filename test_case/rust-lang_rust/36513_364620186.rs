
error[E0599]: no method named `join` found for type `std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@src/main.rs:16:18: 16:41]>` in the current scope
  --> src/main.rs:16:43
   |
16 |     a.iter().map(|x| x.to_str().unwrap()).join(":");
   |                                           ^^^^
   |
   = note: the method `join` exists but the following trait bounds were not satisfied:
           `std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@src/main.rs:16:18: 16:41]> : JoinWithString`
           `&std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@src/main.rs:16:18: 16:41]> : JoinWithString`
           `&mut std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@src/main.rs:16:18: 16:41]> : JoinWithString`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `join`, perhaps you need to implement it:
           candidate #1: `JoinWithString`
