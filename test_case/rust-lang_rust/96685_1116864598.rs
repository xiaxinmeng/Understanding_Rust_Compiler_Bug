plain
test [rustdoc] src/test\rustdoc\wrapping.rs ... ok

failures:

---- [rustdoc] src/test\rustdoc\primitive\no_std.rs stdout ----
thread '[rustdoc] src/test\rustdoc\primitive\no_std.rs' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 5, kind: PermissionDenied, message: "Access is denied." }', src\tools\compiletest\src\runtest.rs:2252:34


failures:
    [rustdoc] src/test\rustdoc\primitive\no_std.rs
    [rustdoc] src/test\rustdoc\primitive\no_std.rs

test result: FAILED. 515 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out; finished in 68.25s

Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Build completed unsuccessfully in 0:35:55
make: *** [Makefile:72: ci-subset-1] Error 1
