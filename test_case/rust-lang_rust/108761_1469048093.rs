plain
.........................................F.............................................. 264/672
.....i...i.........i..................i................................................. 352/672
........................................................................................ 440/672
........................................................................................ 528/672
.................................................................................F.F.... 616/672
...........................F.........F.F................


---- src/collections/vec_deque/mod.rs - collections::vec_deque::Vec<T,A,CO_ALLOC_PREF>::from (line 3061) stdout ----
error[E0282]: type annotations needed for `Vec<i32, std::alloc::Global, CO_ALLOC_PREF>`
  |
  |
9 | let vec = Vec::from(deque);
  |
  |
help: consider giving `vec` an explicit type, where the the value of const parameter `CO_ALLOC_PREF` is specified
  |
9 | let vec: Vec<i32, std::alloc::Global, CO_ALLOC_PREF> = Vec::from(deque);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T,A,CO_ALLOC_PREF>::from_raw_parts_in (line 842) stdout ----
error[E0282]: type annotations needed for `Vec<usize, System, CO_ALLOC_PREF>`
   |
   |
11 | let mut v = Vec::with_capacity_in(3, System);
   |
   |
help: consider giving `v` an explicit type, where the the value of const parameter `CO_ALLOC_PREF` is specified
   |
11 | let mut v: Vec<usize, System, CO_ALLOC_PREF> = Vec::with_capacity_in(3, System);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T,A,CO_ALLOC_PREF>::into_raw_parts_with_alloc (line 960) stdout ----
error[E0282]: type annotations needed for `Vec<u32, System, CO_ALLOC_PREF>`
   |
15 | let rebuilt = unsafe {
   |     ^^^^^^^
   |
   |
help: consider giving `rebuilt` an explicit type, where the the value of const parameter `CO_ALLOC_PREF` is specified
   |
15 | let rebuilt: Vec<u32, System, CO_ALLOC_PREF> = unsafe {
   |            +++++++++++++++++++++++++++++++++
error: doctest failed, to rerun pass `-p alloc --doc`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T,A,CO_ALLOC_PREF>::with_capacity_in (line 760) stdout ----
error[E0282]: type annotations needed for `Vec<i32, System, CO_ALLOC_PREF>`
  |
  |
8 | let mut vec = Vec::with_capacity_in(10, System);
  |
  |
help: consider giving `vec` an explicit type, where the the value of const parameter `CO_ALLOC_PREF` is specified
  |
8 | let mut vec: Vec<i32, System, CO_ALLOC_PREF> = Vec::with_capacity_in(10, System);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T>::from_raw_parts (line 677) stdout ----
error[E0282]: type annotations needed for `Vec<u32, std::alloc::Global, CO_ALLOC_PREF>`
   |
10 |     let vec = unsafe {
   |         ^^^
   |
   |
help: consider giving `vec` an explicit type, where the the value of const parameter `CO_ALLOC_PREF` is specified
   |
10 |     let vec: Vec<u32, std::alloc::Global, CO_ALLOC_PREF> = unsafe {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T>::with_capacity (line 479) stdout ----
error: unknown start of token: `
  --> src/vec/mod.rs:502:1
   |
26 | 