

failures:

---- [compile-fail] compile-fail/bad-intrinsic-monomorphization.rs stdout ----

error: unexpected compiler message: '/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:26:1: 29:2 error: invalid monomorphization of `cttz` intrinsic: expected basic integer type, found `Foo` [E0511]'

error: unexpected compiler message: '/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:31:1: 34:2 error: invalid monomorphization of `fadd_fast` intrinsic: expected basic float type, found `Foo` [E0511]'

error: unexpected compiler message: '/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:36:1: 39:2 error: invalid monomorphization of `simd_add` intrinsic: expected SIMD input type, found non-SIMD `Foo` [E0511]'

error: expected error on line 27 not found: `cttz` intrinsic: expected basic integer type, found `Foo`

error: expected error on line 32 not found: `fadd_fast` intrinsic: expected basic float type, found `Foo`

error: expected error on line 37 not found: `simd_add` intrinsic: expected SIMD input type, found non-SIMD `Foo`

error: 3 unexpected errors found, 3 expected errors not found
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs -L x86_64-unknown-linux-gnu/test/compile-fail/ --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/compile-fail/bad-intrinsic-monomorphization.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/compile-fail/bad-intrinsic-monomorphization.stage2-x86_64-unknown-linux-gnu --cfg rtopt -Z orbit -C rpath -O -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:26:1: 29:2 error: invalid monomorphization of `cttz` intrinsic: expected basic integer type, found `Foo` [E0511]
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:26 unsafe fn test_cttz(v: Foo) -> Foo {
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:27     intrinsics::cttz(v)
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:28     //~^ ERROR `cttz` intrinsic: expected basic integer type, found `Foo`
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:29 }
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:26:1: 29:2 help: run `rustc --explain E0511` to see a detailed explanation
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:31:1: 34:2 error: invalid monomorphization of `fadd_fast` intrinsic: expected basic float type, found `Foo` [E0511]
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:31 unsafe fn test_fadd_fast(a: Foo, b: Foo) -> Foo {
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:32     intrinsics::fadd_fast(a, b)
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:33     //~^ ERROR `fadd_fast` intrinsic: expected basic float type, found `Foo`
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:34 }
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:31:1: 34:2 help: run `rustc --explain E0511` to see a detailed explanation
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:36:1: 39:2 error: invalid monomorphization of `simd_add` intrinsic: expected SIMD input type, found non-SIMD `Foo` [E0511]
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:36 unsafe fn test_simd_add(a: Foo, b: Foo) -> Foo {
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:37     simd_add(a, b)
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:38     //~^ ERROR `simd_add` intrinsic: expected SIMD input type, found non-SIMD `Foo`
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:39 }
/buildslave/rust-buildbot/slave/auto-linux-64-opt-mir/build/src/test/compile-fail/bad-intrinsic-monomorphization.rs:36:1: 39:2 help: run `rustc --explain E0511` to see a detailed explanation
error: aborting due to 3 previous errors

------------------------------------------
