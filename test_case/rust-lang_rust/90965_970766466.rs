plain
test workspaces::ws_warn_unused ... ok

failures:

---- credential_process::libexec_path stdout ----
running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe login -Z credential-process abcdefg`
thread 'credential_process::libexec_path' panicked at '
test failed running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe login -Z credential-process abcdefg`
error: stderr did not match:
1   1         Updating [..]
2   2     error: failed to execute `[..]libexec/cargo-credential-doesnotexist.exe` to store authentication token for registry `crates-io`
4   4     Caused by:
4   4     Caused by:
5        -  The system cannot find the file specified. (os error 2)
    5    +  The system cannot find the path specified. (os error 3)

other output:

', src\tools\cargo\tests\testsuite\credential_process.rs:414:10
', src\tools\cargo\tests\testsuite\credential_process.rs:414:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- tool_paths::custom_runner_env stdout ----
running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe run`
thread 'tool_paths::custom_runner_env' panicked at '
test failed running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe run`
error: stderr did not match:
1   1        Compiling foo [..]
2   2         Finished dev [..]
3   3          Running `nonexistent-runner --foo target/debug/foo.exe`
4   4     error: could not execute process `nonexistent-runner --foo target/debug/foo.exe` (never executed)
6   6     Caused by:
6   6     Caused by:
7        -  The system cannot find the file specified. (os error 2)
    7    +  program not found

other output:

', src\tools\cargo\tests\testsuite\tool_paths.rs:286:10
