rust
> cargo +nightly run --release
    Finished release [unoptimized] target(s) in 0.29s
     Running `target\release\rust-issue-82890.exe`
thread 'main' panicked at 'index out of bounds: the len is 31 but the index is 18446744073709551615', library\core\src\unicode\unicode_data.rs:82:62
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
User { username: StaticString { array: "error: process didn't exit successfully: `target\release\rust-issue-82890.exe` (exit code: 101)
