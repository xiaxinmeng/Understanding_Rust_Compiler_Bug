
error[E0599]: the method `join` exists for struct `Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:41]>`, but its trait bounds were not satisfied
  --> src/main.rs:16:43
   |
16 |       a.iter().map(|x| x.to_str().unwrap()).join(":");
   |                                             ^^^^ method cannot be called on `Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:41]>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `<Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:41]> as Iterator>::Item = String`
           which is required by `Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:41]>: JoinWithString`
           `<&Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:41]> as Iterator>::Item = String`
           which is required by `&Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:41]>: JoinWithString`
           `&Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:41]>: Iterator`
           which is required by `&Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:41]>: JoinWithString`
           `<&mut Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:41]> as Iterator>::Item = String`
           which is required by `&mut Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:41]>: JoinWithString`


