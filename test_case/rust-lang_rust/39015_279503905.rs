
package:  ~/rust/src/tools/compiletest/Cargo.toml
workspace: ~/rust/src/Cargo.toml
   Compiling arena v0.0.0 (file:///~/rust/src/libarena)
    Finished debug [unoptimized + debuginfo] target(s) in 1.40 secs
     Running ~/rust/src/target/debug/deps/arena-3b56f105f34c9b1f

running 13 tests
test tests::bench_copy ... ok
test tests::bench_copy_nonarena ... ok
test tests::bench_noncopy ... ok
test tests::test_arena_alloc_nested ... ok
test tests::bench_noncopy_nonarena ... ok
test tests::test_typed_arena_drop_count ... ok
test tests::test_typed_arena_drop_on_clear ... ok
test tests::test_typed_arena_drop_small_count ... ok
test tests::test_unused ... ok
test tests::test_typed_arena_zero_sized ... ok
test tests::test_typed_arena_clear ... ok
test tests::test_copy ... ok
test tests::test_noncopy ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured
