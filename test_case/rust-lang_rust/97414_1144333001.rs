plain
test [rustdoc] src/test\rustdoc\where.rs ... ok

failures:

---- [rustdoc] src/test\rustdoc\primitive\no_std.rs stdout ----
thread '[rustdoc] src/test\rustdoc\primitive\no_std.rs' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 5, kind: PermissionDenied, message: "Access is denied." }', src\tools\compiletest\src\runtest.rs:2262:34


failures:
    [rustdoc] src/test\rustdoc\primitive\no_std.rs
    [rustdoc] src/test\rustdoc\primitive\no_std.rs

test result: FAILED. 516 passed; 1 failed; 8 ignored; 0 measured; 0 filtered out; finished in 79.78s

Some tests failed in compiletest suite=rustdoc mode=rustdoc host=i686-pc-windows-msvc target=i686-pc-windows-msvc
Build completed unsuccessfully in 0:41:24
make: *** [Makefile:70: ci-subset-1] Error 1
