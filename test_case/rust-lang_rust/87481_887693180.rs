
$ RUST_BACKTRACE=1 cargo r
   Compiling panic v0.1.0 (D:\tmp\panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s
     Running `target\debug\panic.exe`
thread 'main' panicked at 'Hello, world!', src\main.rs:2:5
stack backtrace:
   0: std::panicking::begin_panic
             at /rustc/08095fc1f875c89e507f17cf6c6a780c8ffa4c01\library\std\src/panicking.rs:541:12
   1: panic::main
             at .\src/main.rs:2:5
   2: core::ops::function::FnOnce::call_once
             at /rustc/08095fc1f875c89e507f17cf6c6a780c8ffa4c01\library\core\src\ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: process didn't exit successfully: `target\debug\panic.exe` (exit code: 101)

$ RUST_BACKTRACE=1 cargo r --release
   Compiling panic v0.1.0 (D:\tmp\panic)
    Finished release [optimized] target(s) in 0.44s
     Running `target\release\panic.exe`
thread 'main' panicked at 'Hello, world!', src\main.rs:2:5
stack backtrace:
   0: std::panicking::begin_panic
   1: panic::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: process didn't exit successfully: `target\release\panic.exe` (exit code: 101)

$ rustc -vV
rustc 1.56.0-nightly (08095fc1f 2021-07-26)
binary: rustc
commit-hash: 08095fc1f875c89e507f17cf6c6a780c8ffa4c01
commit-date: 2021-07-26
host: x86_64-pc-windows-gnu
release: 1.56.0-nightly
LLVM version: 12.0.1
