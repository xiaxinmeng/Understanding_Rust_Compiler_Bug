plain
test [rustdoc] src/test\rustdoc\wrapping.rs ... ok

failures:

---- [rustdoc] src/test\rustdoc\primitive\no_std.rs stdout ----
thread '[rustdoc] src/test\rustdoc\primitive\no_std.rs' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 5, kind: PermissionDenied, message: "Access is denied." }', src\tools\compiletest\src\runtest.rs:138:43


failures:
    [rustdoc] src/test\rustdoc\primitive\no_std.rs
    [rustdoc] src/test\rustdoc\primitive\no_std.rs
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-pc-windows-gnu target=x86_64-pc-windows-gnu

test result: FAILED. 515 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out; finished in 80.43s

Build completed unsuccessfully in 0:41:45
make: *** [Makefile:80: ci-mingw-subset-1] Error 1
