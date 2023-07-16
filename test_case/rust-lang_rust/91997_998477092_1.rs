
error[E0783]: trait objects without an explicit `dyn` are deprecated
 --> src/main.rs:4:13
  |
4 |     let _ = MyIterator::<Item=()>::next;
  |             ^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `<dyn MyIterator::<Item=()>>`
