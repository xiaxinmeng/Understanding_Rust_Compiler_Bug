plain
[TIMING] Rustc { target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None }, compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } } -- 0.761
Assembling stage2 compiler (x86_64-pc-windows-msvc)
[TIMING] Sysroot { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } } -- 0.000
[TIMING] Libdir { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } }, target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 0.000
thread 'main' panicked at 'failed to copy `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\x86_64-pc-windows-msvc\release\rustc-main.exe` to `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe`: The process cannot access the file because it is being used by another process. (os error 32)', src\bootstrap\lib.rs:1335:17
Build completed unsuccessfully in 0:00:03
