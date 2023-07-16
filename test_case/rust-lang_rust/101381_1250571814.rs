plain
test [ui] src/test/ui/allocator/two-allocators3.rs ... ok
test [ui] src/test/ui/anon-params/anon-params-denied-2018.rs ... ok
test [ui] src/test/ui/allocator/no_std-alloc-error-handler-default.rs ... ok
test [ui] src/test/ui/allocator/custom.rs ... ok
test [ui] src/test/ui/abi/homogenous-floats-target-feature-mixup.rs ... FAILED
test [ui] src/test/ui/allocator/custom-in-block.rs ... ok
test [ui] src/test/ui/anon-params/anon-params-deprecated.rs ... ok
test [ui] src/test/ui/allocator/custom-in-submodule.rs ... ok
test [ui] src/test/ui/anon-params/anon-params-edition-hygiene.rs ... ok
---
test [ui] src/test/ui/transmutability/primitives/numbers.rs ... ok

failures:

---- [ui] src/test/ui/abi/homogenous-floats-target-feature-mixup.rs stdout ----
error: test run failed!
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/homogenous-floats-target-feature-mixup/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `f32x2(-1.7014118e38, 9.3168e-39)`,
 right: `f32x2(1.0, 2.0)`', /checkout/src/test/ui/abi/homogenous-floats-target-feature-mixup.rs:126:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'invalid status at sse: exit status: 101', /checkout/src/test/ui/abi/homogenous-floats-target-feature-mixup.rs:39:9
------------------------------------------




failures:
    [ui] src/test/ui/abi/homogenous-floats-target-feature-mixup.rs
test result: FAILED. 13363 passed; 1 failed; 180 ignored; 0 measured; 0 filtered out; finished in 85.84s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
Build completed unsuccessfully in 0:12:37
