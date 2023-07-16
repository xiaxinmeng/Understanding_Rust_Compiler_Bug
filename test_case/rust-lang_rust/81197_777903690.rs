
running 1 test
test doctest.rs - Foo (line 1) ... FAILED

failures:

---- doctest.rs - Foo (line 1) stdout ----
Some tests failed in compiletest
suite=run-make-fulldeps mode=run-make host=i686-pc-windows-msvc
target=i686-pc-windows-msvc
thread 'doctest.rs - Foo (line 1)' panicked at
'Failed to spawn rustc process: Os { code: 2, kind: NotFound, message: "The
system cannot find the file specified." }', src\librustdoc\doctest.rs:317:38
note: run with `RUST_BACKTRACE=1` environment
variable to display a backtrace



failures:
    doctest.rs - Foo (line 1)

test result: FAILED. 0 passed; 1 failed; 0 ignored; finished in 0.01s

make[1]: Leaving directory
make-fulldeps/rustdoc-test-builder'

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: unexpected panic

error: Unrecognized option: 'test-builder'

make[1]: *** [Makefile:10: all] Error 101
