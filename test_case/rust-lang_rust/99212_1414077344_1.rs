
error: `BTreeMap::<K, V, A>::len` is not yet stable as a const fn
 --> src/lib.rs:7:13
  |
7 |     let l = x.len();
  |             ^^^^^^^
  |
  = help: add `#![feature(const_btree_len)]` to the crate attributes to enable
