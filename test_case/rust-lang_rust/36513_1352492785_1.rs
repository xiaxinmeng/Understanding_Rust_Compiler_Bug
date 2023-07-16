
error[E0599]: the method `join` exists for struct `Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:21]>`, but its trait bounds were not satisfied
  --> src/main.rs:16:43
   |
16 |     a.iter().map(|x| x.to_str().unwrap()).join(":");
   |                                           ^^^^ method cannot be called on `Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:21]>` due to unsatisfied trait bounds
  --> /rustc/0f529f0f49f4dd404b78e605398531c96f220fc5/library/core/src/iter/adapters/map.rs:61:1
   |
   |   = note: doesn't satisfy `<_ as Iterator>::Item = String`
  
 = note: doesn't satisfy `_: JoinWithString`
   |
note: trait bound `&Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:21]>: Iterator` was not satisfied
  --> src/main.rs:6:12
   |
5  | impl<X> JoinWithString for X 
   |         --------------     -
6  |   where X: Iterator<Item = String> 
   |            ^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound introduced here
note: the following trait bounds were not satisfied:
      `<&Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:21]> as Iterator>::Item = String`
      `<&mut Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:21]> as Iterator>::Item = String`
      `<Map<std::slice::Iter<'_, PathBuf>, [closure@src/main.rs:16:18: 16:21]> as Iterator>::Item = String`
  --> src/main.rs:6:21
   |
5  | impl<X> JoinWithString for X 
   |         --------------     -
6  |   where X: Iterator<Item = String> 
   |                     ^^^^^^^^^^^^^ unsatisfied trait bound introduced here
