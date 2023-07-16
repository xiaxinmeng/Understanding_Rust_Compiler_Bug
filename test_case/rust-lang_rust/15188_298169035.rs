
rustc 1.18.0-nightly (128aa262e 2017-04-28)
warning: field is never used: `data`
  --> <anon>:39:3
   |
39 |   data: T,
   |   ^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

warning: field is never used: `next`
  --> <anon>:40:3
   |
40 |   next: Option<Box<List<T>>>
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

warning: unused variable: `deserialize`
  --> <anon>:55:7
   |
55 |   let deserialize: Result<List<u8>, _> = Serial::deserialize(serialized.iter().map(|x| *x));
   |       ^^^^^^^^^^^
   |
   = note: #[warn(unused_variables)] on by default

error[E0275]: overflow evaluating the requirement `<std::slice::Iter<u8> as std::iter::Iterator>::Item`
  |
  = help: consider adding a `#![recursion_limit="512"]` attribute to your crate
  = note: required because of the requirements on the impl of `std::iter::Iterator` for `std::iter::Map<std::slice::Iter<u8>, [closure@<anon>:55:84: 55:90]>`
  = note: required because of the requirements on the impl of `std::iter::Iterator` for `&mut std::iter::Map<std::slice::Iter<u8>, [closure@<anon>:55:84: 55:90]>`
   .... snip 200+ lines

error: aborting due to previous error

