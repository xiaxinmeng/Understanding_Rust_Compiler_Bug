
error[E0599]: no method named `join` found for struct `std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:18:18: 18:41]>` in the current scope
  --> file5.rs:18:43
   |
1  | pub trait JoinWithString {
   | ------------------------ this trait defines an item `join`
...
18 |     a.iter().map(|x| x.to_str().unwrap()).join(":");
   |                                           ^^^^ method not found in `std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:18:18: 18:41]>`
   |
   = note: the method `join` exists but the following trait bounds were not satisfied:
           `&std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:18:18: 18:41]>: std::iter::Iterator`
           `<_ as std::iter::Iterator>::Item = std::string::String`
   = help: items from traits can only be used if the trait is implemented and in scope
