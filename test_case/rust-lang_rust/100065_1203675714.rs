plain
test workspaces::ws_warn_unused ... ok

failures:

---- jobserver::jobserver_and_j stdout ----
running `make -j2`
thread 'jobserver::jobserver_and_j' panicked at '
test failed running `make -j2`
error: stderr did not match:
1        -warning: a `-j` argument was passed to Cargo but Cargo is also configured with an external jobserver in its environment, ignoring the `-j` parameter
2   1        Compiling [..]
3   2         Finished [..]

other output:
D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build -j2


', src\tools\cargo\tests\testsuite\jobserver.rs:186:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- jobserver::makes_jobserver_used stdout ----
running `make -j2`
thread '<unnamed>' panicked at 'assertion failed: l.accept().is_err()', src\tools\cargo\tests\testsuite\jobserver.rs:143:13
thread 'jobserver::makes_jobserver_used' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', src\tools\cargo\tests\testsuite\jobserver.rs:159:18

failures:
    jobserver::jobserver_and_j
    jobserver::makes_jobserver_used
