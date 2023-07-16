
error[E0599]: no method named `join` found for struct `std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:16:18: 16:41]>` in the current scope
   --> file5.rs:16:43
    |
16  |     a.iter().map(|x| x.to_str().unwrap()).join(":");
    |                                           ^^^^ method not found in `std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:16:18: 16:41]>`
    |
   ::: /rust/src/libcore/iter/adapters/mod.rs:751:1
    |
751 | pub struct Map<I, F> {
    | --------------------
    | |
    | doesn't satisfy `<_ as std::iter::Iterator>::Item = std::string::String`
    | doesn't satisfy `_: JoinWithString`
    |
    = note: the method `join` exists but the following trait bounds were not satisfied:
            `<std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:16:18: 16:41]> as std::iter::Iterator>::Item = std::string::String`
            which is required by `std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:16:18: 16:41]>: JoinWithString`
            `<&std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:16:18: 16:41]> as std::iter::Iterator>::Item = std::string::String`
            which is required by `&std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:16:18: 16:41]>: JoinWithString`
            `&std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:16:18: 16:41]>: std::iter::Iterator`
            which is required by `&std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:16:18: 16:41]>: JoinWithString`
            `<&mut std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:16:18: 16:41]> as std::iter::Iterator>::Item = std::string::String`
            which is required by `&mut std::iter::Map<std::slice::Iter<'_, std::path::PathBuf>, [closure@file5.rs:16:18: 16:41]>: JoinWithString`
    = help: items from traits can only be used if the trait is implemented and in scope
note: `JoinWithString` defines an item `join`, perhaps you need to implement it
   --> file5.rs:1:1
    |
1   | pub trait JoinWithString {
    | ^^^^^^^^^^^^^^^^^^^^^^^^
