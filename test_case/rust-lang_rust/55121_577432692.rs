
error[E0507]: cannot move out of a shared reference
  --> src/main.rs:10:39
   |
10 |     let _tuples = x.iter().map(|test| test.clone().into_tuple()).collect::<Vec<_>>();
   |                                       ^^^^^^^^^^^^ move occurs because value has type `Test`, which does not implement the `Copy` trait
