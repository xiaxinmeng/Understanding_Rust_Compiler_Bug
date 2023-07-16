
[01:07:46] stdout:
[01:07:46] ------------------------------------------
[01:07:46] 
[01:07:46] ------------------------------------------
[01:07:46] stderr:
[01:07:46] ------------------------------------------
[01:07:46] C:\projects\rust\src/test\codegen\remap_path_prefix\main.rs:19:11: error: expected string not found in input
[01:07:46] // CHECK: internal constant [34 x i8] c"/the/src/remap_path_prefix/main.rs"
[01:07:46]           ^
[01:07:46] C:\projects\rust\build\x86_64-pc-windows-gnu\test\codegen\remap_path_prefix\main.ll:1:1: note: scanning from here
[01:07:46] ; ModuleID = 'main.cgu-0.rs'
[01:07:46] ^
[01:07:46] C:\projects\rust\build\x86_64-pc-windows-gnu\test\codegen\remap_path_prefix\main.ll:9:10: note: possible intended match here
[01:07:46] @str.0 = internal constant [34 x i8] c"/the/src\5Cremap_path_prefix\5Cmain.rs"
[01:07:46]          ^
[01:07:46] 
[01:07:46] ------------------------------------------
[01:07:46] 
[01:07:46] thread '[codegen] codegen\remap_path_prefix\main.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2630
[01:07:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:46] 
[01:07:46] 
[01:07:46] failures:
[01:07:46]     [codegen] codegen\remap_path_prefix\main.rs
[01:07:46] 
[01:07:46] test result: FAILED. 34 passed; 1 failed; 5 ignored; 0 measured
[01:07:46] 
[01:07:46] thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:329
