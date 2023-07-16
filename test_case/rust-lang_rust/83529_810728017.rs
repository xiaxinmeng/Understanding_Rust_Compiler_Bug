plain
 finished in 1.741 seconds
[TIMING] Miri { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } }, target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 1.751
Dist rust-nightly-x86_64-pc-windows-msvc
 finished in 104.276 seconds
thread 'main' panicked at 'could not read dir "D:\\a\\rust\\rust\\build\\tmp\\tarball\\rust\\x86_64-pc-windows-msvc\\rust-demangler-nightly-x86_64-pc-windows-msvc\\rust-demangler": Os { code: 3, kind: NotFound, message: "The system cannot find the path specified." }', src\bootstrap\lib.rs:1343:25
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 0:46:04
