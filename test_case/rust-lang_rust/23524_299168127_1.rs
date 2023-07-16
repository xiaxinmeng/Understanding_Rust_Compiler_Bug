
rustc 1.19.0-nightly (2d4ed8e0c 2017-05-03)
warning: field is never used: `population`
 --> <anon>:4:5
  |
4 |     population: u16,
  |     ^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: field is never used: `children`
 --> <anon>:5:5
  |
5 |     children: T,
  |     ^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: unused import: `std::sync::atomic`
  --> <anon>:12:9
   |
12 |     use std::sync::atomic;
   |         ^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_imports)] on by default

16
16
