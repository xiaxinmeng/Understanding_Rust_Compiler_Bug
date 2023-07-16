plain
...........F...
failures:

---- tests::test_typed_arena_drop_small_count stdout ----
thread 'tests::test_typed_arena_drop_small_count' panicked at 'range end index 100 out of range for slice of length 0', library/core/src/slice/index.rs:73:5


failures:
    tests::test_typed_arena_drop_small_count
    tests::test_typed_arena_drop_small_count

test result: FAILED. 14 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

error: test failed, to rerun pass '-p rustc_arena --lib'
