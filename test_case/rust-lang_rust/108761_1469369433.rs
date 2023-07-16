plain
.................................................................................FF..... 616/672
.....................................FF.................
failures:

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
---- src/vec/mod.rs - vec::Vec<T,A,CO_ALLOC_PREF>::from_raw_parts_in (line 856) stdout ----
error[E0282]: type annotations needed for `Vec<usize, System, CO_ALLOC_PREF>`
   |
   |
33 |     let rebuilt = Vec::from_raw_parts_in(p, len, cap, alloc.clone());
   |
   |
help: consider giving `rebuilt` an explicit type, where the the value of const parameter `CO_ALLOC_PREF` is specified
   |
33 |     let rebuilt: Vec<usize, System, CO_ALLOC_PREF> = Vec::from_raw_parts_in(p, len, cap, alloc.clone());

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T,A,CO_ALLOC_PREF>::into_raw_parts_with_alloc (line 974) stdout ----
error[E0282]: type annotations needed for `Vec<u32, System, CO_ALLOC_PREF>`
   |
15 | let rebuilt = unsafe {
   |     ^^^^^^^
   |
   |
help: consider giving `rebuilt` an explicit type, where the the value of const parameter `CO_ALLOC_PREF` is specified
   |
15 | let rebuilt: Vec<u32, System, CO_ALLOC_PREF> = unsafe {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T>::with_capacity (line 479) stdout ----
error: unknown start of token: `
  --> src/vec/mod.rs:502:1
   |
26 | 