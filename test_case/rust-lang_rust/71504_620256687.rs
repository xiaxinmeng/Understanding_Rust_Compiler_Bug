
$> cargo run --release
    Finished release [optimized] target(s) in 0.03s
     Running `target\release\i71504.exe`
thread 'main' panicked at 'assertion failed: GlobSetBuilder::new().build().unwrap().is_match("src/tests")', src\main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\release\i71504.exe` (exit code: 101)
