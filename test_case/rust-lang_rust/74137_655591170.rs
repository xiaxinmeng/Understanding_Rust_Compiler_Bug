
 failures:

---- package::long_file_names stdout ----
thread 'package::long_file_names' panicked at 'could not create file D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t1188\foo\012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789: The system cannot find the path specified. (os error 3)', src\tools\cargo\crates\cargo-test-support\src\lib.rs:168:33
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    package::long_file_names

test result: FAILED. 1890 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--test testsuite'
