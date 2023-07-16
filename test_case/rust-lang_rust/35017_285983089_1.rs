
./x.py test src/test/compile-fail --test-args alloc
running 9 tests
test [compile-fail] compile-fail/feature-gate-allocator.rs ... ok
test [compile-fail] compile-fail/feature-gate-needs-allocator.rs ... ok
test [compile-fail] compile-fail/regions-return-stack-allocated-vec.rs ... ok
test [compile-fail] compile-fail/allocator-depends-on-needs-allocators.rs ... ok
test [compile-fail] compile-fail/two-allocators-3.rs ... ok
test [compile-fail] compile-fail/allocator-dylib-is-system.rs ... FAILED
test [compile-fail] compile-fail/two-allocators-2.rs ... ok
test [compile-fail] compile-fail/allocator-rust-dylib-is-jemalloc.rs ... FAILED
test [compile-fail] compile-fail/two-allocators.rs ... ok
