
V:\tmp\myapp>cargo run --release --
   Compiling myapp v0.1.0 (file:///V:/tmp/myapp)
    Finished release [optimized] target(s) in 1.67 secs
     Running `target\release\myapp.exe`
Start
thread 'main' panicked at 'failed to launch browser: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }', libcore\result.rs:945:5 note: Run with `RUST_BACKTRACE=1` for a backtrace. error: process didn't exit successfully: `target\release\myapp.exe` (exit code: 101)
