    #[cfg(not(no_global_oom_handling))]

error: aborting due to 2 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- src/vec/mod.rs - vec::Vec<T>::from_raw_parts (line 745) stdout ----
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

failures:
    src/collections/vec_deque/mod.rs - collections::vec_deque::Vec<T,A,CO_ALLOC_PREF>::from (line 3061)
    src/vec/mod.rs - vec::Vec<T,A,CO_ALLOC_PREF>::from_raw_parts_in (line 856)
    src/vec/mod.rs - vec::Vec<T,A,CO_ALLOC_PREF>::into_raw_parts_with_alloc (line 974)
    src/vec/mod.rs - vec::Vec<T>::with_capacity (line 479)

test result: FAILED. 663 passed; 5 failed; 4 ignored; 0 measured; 0 filtered out; finished in 11.33s


error: doctest failed, to rerun pass `-p alloc --doc`
