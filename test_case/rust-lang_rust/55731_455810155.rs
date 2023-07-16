
error[E0308]: mismatched types
  --> src/main.rs:49:5
   |
49 |     multi(Map {
   |     ^^^^^ one type is more general than the other
   |
   = note: expected type `DistributedIteratorMulti<&'a ()>`
              found type `DistributedIteratorMulti<&()>`
