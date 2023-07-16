plain
 finished in 38.627 seconds
[TIMING] Docs { host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 43.758
[TIMING] RustcDocs { host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 0.000
[TIMING] Mingw { host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 0.000
[TIMING] DebuggerScripts { sysroot: "D:\\a\\rust\\rust\\build\\tmp\\tarball\\x86_64-pc-windows-msvc\\image", host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 0.002
 finished in 41.981 seconds
[TIMING] Rustc { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } } -- 41.999
Dist rust-std-nightly-x86_64-pc-windows-msvc
 finished in 32.090 seconds
---
 finished in 3.249 seconds
[TIMING] Miri { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } }, target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 3.261
Dist rust-nightly-x86_64-pc-windows-msvc
 finished in 145.862 seconds
thread 'main' panicked at 'could not read dir "D:\\a\\rust\\rust\\build\\tmp/dist\\work\\rustc-nightly-x86_64-pc-windows-msvc\\rustc": Os { code: 3, kind: NotFound, message: "The system cannot find the path specified." }', src\bootstrap\lib.rs:1339:25
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 1:00:09
