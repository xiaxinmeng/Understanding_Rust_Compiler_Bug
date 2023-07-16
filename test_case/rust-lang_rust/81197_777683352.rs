
'D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\bin\rustdoc.exe' -L D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib --test -Z unstable-options --test-builder D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\bin\rustc.exe doctest.rs

failures:

---- doctest.rs - Foo (line 1) stdout ----
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=i686-pc-windows-msvc target=i686-pc-windows-msvc
thread 'doctest.rs - Foo (line 1)' panicked at 'Failed to spawn rustc process: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }', src\librustdoc\doctest.rs:317:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
