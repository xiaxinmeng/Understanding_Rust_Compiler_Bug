
PS> rustup run nightly rustc --version
rustc 1.43.0-nightly (6d0e58bff 2020-02-23)
PS> rustup run nightly cargo run --release
    Finished release [optimized] target(s) in 0.01s
     Running `target\release\rust-segfault.exe`
error: process didn't exit successfully: `target\release\rust-segfault.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
