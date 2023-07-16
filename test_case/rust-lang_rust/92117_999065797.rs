plain

failures:

---- process::tests::test_interior_nul_in_arg_is_error stdout ----
thread 'process::tests::test_interior_nul_in_arg_is_error' panicked at 'assertion failed: `(left == right)`
  left: `NotFound`,
 right: `InvalidInput`', library\std\src\process\tests.rs:309:19
---- process::tests::test_interior_nul_in_args_is_error stdout ----
thread 'process::tests::test_interior_nul_in_args_is_error' panicked at 'assertion failed: `(left == right)`
  left: `NotFound`,
  left: `NotFound`,
 right: `InvalidInput`', library\std\src\process\tests.rs:317:19
---- process::tests::test_interior_nul_in_current_dir_is_error stdout ----
thread 'process::tests::test_interior_nul_in_current_dir_is_error' panicked at 'assertion failed: `(left == right)`
  left: `NotFound`,
  left: `NotFound`,
 right: `InvalidInput`', library\std\src\process\tests.rs:325:19
---- sys::windows::process::tests::windows_exe_resolver stdout ----
---- sys::windows::process::tests::windows_exe_resolver stdout ----
thread 'sys::windows::process::tests::windows_exe_resolver' panicked at 'assertion failed: resolve_exe(OsStr::new(\"rustc\"), child_paths).is_ok()', library\std\src\sys\windows\process\tests.rs:174:5

failures:
    process::tests::test_interior_nul_in_arg_is_error
    process::tests::test_interior_nul_in_args_is_error
    process::tests::test_interior_nul_in_args_is_error
    process::tests::test_interior_nul_in_current_dir_is_error
    sys::windows::process::tests::windows_exe_resolver

test result: FAILED. 879 passed; 4 failed; 3 ignored; 0 measured; 0 filtered out; finished in 13.57s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "D:\\a\\rust\\rust\\library/test/Cargo.toml" "-p" "std" "--"


Build completed unsuccessfully in 0:48:02
Build completed unsuccessfully in 0:48:02
make: *** [Makefile:72: ci-subset-1] Error 1
