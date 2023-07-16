plain
test [codegen] tests\codegen\issues\issue-56267-2.rs ... ok
test [codegen] tests\codegen\issues\issue-59352.rs ... ok
test [codegen] tests\codegen\issues\issue-56267.rs ... ok
test [codegen] tests\codegen\issues\issue-56927.rs ... ok
2023-03-16T07:34:16.004521Z ERROR compiletest::runtest: fatal error, panic: "aux-build `D:\\a\\rust\\rust\\tests\\codegen\\issues\\auxiliary\\static_dllimport_aux.rs` source not found"
test [codegen] tests\codegen\issues\issue-73827-bounds-check-index-in-subexpr.rs ... ok
test [codegen] tests\codegen\issues\issue-73338-effecient-cmp.rs ... ok
test [codegen] tests\codegen\issues\issue-73031.rs ... ok
test [codegen] tests\codegen\issues\issue-75525-bounds-checks.rs ... ok
---
failures:

---- [codegen] tests\codegen\issues\issue-81408-dllimport-thinlto-windows.rs stdout ----

error: aux-build `D:\a\rust\rust\tests\codegen\issues\auxiliary\static_dllimport_aux.rs` source not found
thread '[codegen] tests\codegen\issues\issue-81408-dllimport-thinlto-windows.rs' panicked at 'fatal error', src\tools\compiletest\src\runtest.rs:2252:9


failures:
    [codegen] tests\codegen\issues\issue-81408-dllimport-thinlto-windows.rs
    [codegen] tests\codegen\issues\issue-81408-dllimport-thinlto-windows.rs

test result: FAILED. 319 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 4.40s

Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Build completed unsuccessfully in 0:26:18
make: *** [Makefile:68: ci-subset-1] Error 1
