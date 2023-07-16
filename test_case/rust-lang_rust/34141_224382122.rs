
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src/test\codegen\stores.rs:34:11: error: expected string not found in input
// CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* [[Y8]], i8* [[TMP8]], i64 4, i32 1, i1 false)
          ^
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\codegen\stores.ll:19:2: note: scanning from here
 call void @llvm.memcpy.p0i8.p0i8.i32(i8* %3, i8* %4, i32 4, i32 1, i1 false)
 ^
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\codegen\stores.ll:19:2: note: with variable "Y8" equal to "%3"
 call void @llvm.memcpy.p0i8.p0i8.i32(i8* %3, i8* %4, i32 4, i32 1, i1 false)
 ^
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\codegen\stores.ll:19:2: note: with variable "TMP8" equal to "%4"
 call void @llvm.memcpy.p0i8.p0i8.i32(i8* %3, i8* %4, i32 4, i32 1, i1 false)
 ^
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src/test\codegen\stores.rs:49:11: error: expected string not found in input
// CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* [[Y8]], i8* [[TMP8]], i64 4, i32 1, i1 false)
          ^
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\codegen\stores.ll:50:2: note: scanning from here
 call void @llvm.memcpy.p0i8.p0i8.i32(i8* %3, i8* %4, i32 4, i32 1, i1 false)
 ^
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\codegen\stores.ll:50:2: note: with variable "Y8" equal to "%3"
 call void @llvm.memcpy.p0i8.p0i8.i32(i8* %3, i8* %4, i32 4, i32 1, i1 false)
 ^
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\codegen\stores.ll:50:2: note: with variable "TMP8" equal to "%4"
 call void @llvm.memcpy.p0i8.p0i8.i32(i8* %3, i8* %4, i32 4, i32 1, i1 false)
