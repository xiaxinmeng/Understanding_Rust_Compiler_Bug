`
running 10 tests
test builder::__test::build_default ... ok
test builder::__test::build_with_target_flag ... ok
test builder::__test::dist_baseline ... FAILED
test builder::__test::dist_with_hosts ... ok
test builder::__test::dist_with_same_targets_and_hosts ... ok
test builder::__test::dist_with_target_flag ... ok
test builder::__test::dist_with_targets ... ok
test builder::__test::dist_with_targets_and_hosts ... ok
test builder::__test::test_exclude ... ok
test builder::__test::test_with_no_doc_stage0 ... ok
failures:
---- builder::__test::dist_baseline stdout ----
git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
thread 'main' panicked at 'command did not execute successfully: "\\\\?\\C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "install" "--list"
expected success, got: exit code: 0xc0000005', src\build_helper\lib.rs:134:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
failures:
    builder::__test::dist_baseline
test result: FAILED. 9 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
error: test failed, to rerun pass '--lib'
command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "--" "--test-threads=1"
expected success, got: exit code: 101
failed to run: C:\projects\rust\build\bootstrap\debug\bootstrap test
Build completed unsuccessfully in 1:53:27
Command exited with code 1
