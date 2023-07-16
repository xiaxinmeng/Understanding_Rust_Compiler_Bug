
error: Undefined Behavior: type validation failed at .value.1.desc.name.<enum-tag>: encountered uninitialized bytes, but expected a valid enum tag
   --> /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:654:9
    |
654 |         tmp.assume_init()
    |         ^^^^^^^^^^^^^^^^^ type validation failed at .value.1.desc.name.<enum-tag>: encountered uninitialized bytes, but expected a valid enum tag
    |
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
            
    = note: inside `std::ptr::read::<(test::test::TestId, test::test::TestDescAndFn)>` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:654:9
    = note: inside `std::vec::Vec::<(test::test::TestId, test::test::TestDescAndFn)>::pop` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1761:22
    = note: inside `test::test::run_tests::<[closure@test::test::run_tests_console::{closure#2}]>` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/test/src/lib.rs:301:30
    = note: inside `test::test::run_tests_console` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/test/src/console.rs:286:5
    = note: inside `test::test::test_main` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/test/src/lib.rs:116:15
    = note: inside `test::test::test_main_static` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/test/src/lib.rs:135:5
    = note: inside `main`
