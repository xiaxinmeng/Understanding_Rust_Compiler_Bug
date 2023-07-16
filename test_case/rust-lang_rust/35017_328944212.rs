text
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 4 tests
test [run-pass] run-pass/allocator-alloc-one.rs ... ok
test [run-pass] run-pass/allocator/custom.rs ... ok
test [run-pass] run-pass/allocator/xcrate-use.rs ... ok
test [run-pass] run-pass/allocator/xcrate-use2.rs ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 2752 filtered out

        finished in 0.614
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 8 tests
test [compile-fail] compile-fail/feature-gate-needs-allocator.rs ... ok
test [compile-fail] compile-fail/feature-gate-global_allocator.rs ... ok
test [compile-fail] compile-fail/allocator/function-allocator.rs ... ok
test [compile-fail] compile-fail/feature-gate-allocator_internals.rs ... ok
test [compile-fail] compile-fail/allocator/two-allocators.rs ... ok
test [compile-fail] compile-fail/allocator/not-an-allocator.rs ... ok
test [compile-fail] compile-fail/allocator/two-allocators2.rs ... ok
test [compile-fail] compile-fail/allocator/two-allocators3.rs ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 2741 filtered out
