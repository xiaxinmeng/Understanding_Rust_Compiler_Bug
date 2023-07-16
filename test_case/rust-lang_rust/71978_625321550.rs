
stderr:
------------------------------------------
D:\a\rust\rust\src/test\codegen\ffi-out-of-bounds-loads.rs:18:12: error: CHECK: expected string not found in input
 // CHECK: load { i64, i32 }, { i64, i32 }* {{.*}}, align 4
           ^
D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\ffi-out-of-bounds-loads\ffi-out-of-bounds-loads.ll:1:1: note: scanning from here
; ModuleID = 'ffi_out_of_bounds_loads.7rcbfp3g-cgu.0'
^
D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\ffi-out-of-bounds-loads\ffi-out-of-bounds-loads.ll:52:8: note: possible intended match here
 %_2 = load i32, i32* %self, align 4
       ^

------------------------------------------



failures:
    [codegen] codegen\ffi-out-of-bounds-loads.rs
