
stdout:
------------------------------------------
------------------------------------------
stderr:
------------------------------------------
C:\projects\rust\src/test\codegen\remap_path_prefix\main.rs:19:11: error: expected string not found in input
// CHECK: internal constant [34 x i8] c"/the/src/remap_path_prefix/main.rs"
          ^
C:\projects\rust\build\x86_64-pc-windows-gnu\test\codegen\remap_path_prefix\main.ll:1:1: note: scanning from here
; ModuleID = 'main.cgu-0.rs'
^
C:\projects\rust\build\x86_64-pc-windows-gnu\test\codegen\remap_path_prefix\main.ll:9:10: note: possible intended match here
@str.0 = internal constant [34 x i8] c"/the/src\5Cremap_path_prefix\5Cmain.rs"
         ^
------------------------------------------
thread '[codegen] codegen\remap_path_prefix\main.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2630
note: Run with `RUST_BACKTRACE=1` for a backtrace.
