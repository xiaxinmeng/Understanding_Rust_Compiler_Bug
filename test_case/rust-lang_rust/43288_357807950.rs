
error[E0623]: lifetime mismatch
 --> src/main.rs:6:16
  |
5 | fn foo(iter: &mut Iterator<Item=&str>, mut map: MyMap) {
  |                                 ----            -----
  |                                 |
  |                                 these two types are declared with different lifetimes...
6 |     map.insert(iter.next().unwrap(), "value");
  |                ^^^^^^^^^^^^^^^^^^^^ ...but data from `iter` flows into `map` here
