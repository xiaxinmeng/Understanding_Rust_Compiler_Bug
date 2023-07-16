
[01:50:32] test test::test_handle_utf8_directory has been running for over 60 seconds
[01:54:52] test test::test_handle_utf8_directory ... FAILED
[01:54:52] 
[01:54:52] failures:
[01:54:52] 
[01:54:52] ---- test::test_handle_utf8_directory stdout ----
[01:54:52] 	thread 'test::test_handle_utf8_directory' panicked at 'Hit timeout', src\tools\rls\src\test\harness.rs:142:12
[01:54:52] 
[01:54:52] 
[01:54:52] failures:
[01:54:52]     test::test_handle_utf8_directory
[01:54:52] 
[01:54:52] test result: FAILED. 23 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:54:52] 
[01:54:52] error: test failed, to rerun pass '--bin rls'
[01:54:52] 
[01:54:52] 
[01:54:52] command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "test" "--target" "x86_64-pc-windows-msvc" "--release" "--locked" "--color" "always" "--manifest-path" "C:\\projects\\rust\\src/tools/rls/Cargo.toml"
[01:54:52] expected success, got: exit code: 101
