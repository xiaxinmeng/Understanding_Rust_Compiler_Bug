
Compiling arena v0.0.0 (file:///rust/src/libarena)
    Finished release [optimized] target(s) in 1.66 secs
     Running /rust/src/target/release/deps/arena-3b56f105f34c9b1f

running 13 tests
test tests::bench_copy_nonarena ... ok
test tests::bench_noncopy ... ok
test tests::bench_copy ... ok
test tests::bench_noncopy_nonarena ... ok
test tests::test_arena_alloc_nested ... ok
test tests::test_typed_arena_drop_on_clear ... ok
test tests::test_typed_arena_drop_count ... ok
test tests::test_typed_arena_drop_small_count ... FAILED
test tests::test_typed_arena_clear ... ok
test tests::test_copy ... ok
test tests::test_unused ... ok
test tests::test_typed_arena_zero_sized ... ok
test tests::test_noncopy ... ok

failures:

---- tests::test_typed_arena_drop_small_count stdout ----
	thread 'tests::test_typed_arena_drop_small_count' panicked at 'assertion failed: `(left == right)` (left: `0`, right: `100`)', lib.rs:591
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    tests::test_typed_arena_drop_small_count

test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured

