    #[cfg(not(no_global_oom_handling))]

error: aborting due to 2 previous errors

Couldn't compile the test.
Couldn't compile the test.

failures:
    src/collections/vec_deque/mod.rs - collections::vec_deque::Vec<T,A,CO_ALLOC_PREF>::from (line 3061)
    src/vec/mod.rs - vec::Vec<T,A,CO_ALLOC_PREF>::from_raw_parts_in (line 842)
    src/vec/mod.rs - vec::Vec<T,A,CO_ALLOC_PREF>::into_raw_parts_with_alloc (line 960)
    src/vec/mod.rs - vec::Vec<T,A,CO_ALLOC_PREF>::with_capacity_in (line 760)
    src/vec/mod.rs - vec::Vec<T>::with_capacity (line 479)

test result: FAILED. 662 passed; 6 failed; 4 ignored; 0 measured; 0 filtered out; finished in 11.38s

