plain

failures:

---- path::tests::test_compare stdout ----
thread 'path::tests::test_compare' panicked at '"c:\\foo" == "C:\\foo", expected true, got 16285471172997689862 and 1937321254420185683', library\std\src\path\tests.rs:1538:9

failures:
    path::tests::test_compare


test result: FAILED. 864 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 14.84s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "--target" "i686-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "D:\\a\\rust\\rust\\library/test/Cargo.toml" "-p" "std" "--"


Build completed unsuccessfully in 0:50:40
Build completed unsuccessfully in 0:50:40
make: *** [Makefile:72: ci-subset-1] Error 1
