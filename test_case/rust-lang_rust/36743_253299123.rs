
failures:

---- wrong_checksum_is_an_error stdout ----
    thread 'wrong_checksum_is_an_error' panicked at 'failed to remove dir C:\bot\slave\auto-win-msvc-64-cargotest\build\obj\build\ct\cargo\target\cit\t0\bar\target\debug\deps: The directory is not empty. (os error 145)', tests\cargotest\support\paths.rs:143
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    wrong_checksum_is_an_error

test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured



command did not execute successfully: "C:\\bot\\slave\\auto-win-msvc-64-cargotest\\build\\obj\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\cargotest.exe" "C:\\bot\\slave\\auto-win-msvc-64-cargotest\\build\\obj\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "C:\\bot\\slave\\auto-win-msvc-64-cargotest\\build\\obj\\build\\ct"
expected success, got: exit code: 101


Makefile:51: recipe for target 'check-cargotest' failed
