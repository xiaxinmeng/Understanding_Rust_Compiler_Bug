
error[E0275]: overflow evaluating the requirement `_: std::cmp::Ord`
  --> test.rs:20:5
   |
20 |     f1(&a);
   |     ^^
   |
   = note: consider adding a `#![recursion_limit="128"]` attribute to your crate
   = note: required because of the requirements on the impl of `std::ops::Sub` for `&std::collections::BTreeSet<_>`
   = note: required because ...
   ...
