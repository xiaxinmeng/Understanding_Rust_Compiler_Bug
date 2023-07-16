
--- src/test/debuginfo/type-names.rs.orig       2016-09-10 19:14:05.466433219 +0000
+++ src/test/debuginfo/type-names.rs    2016-09-11 08:47:57.833460343 +0000
@@ -153,9 +153,6 @@
 // gdb-command:whatis unsafe_fn_with_return_value
 // gdb-check:type = struct (unsafe fn(type_names::GenericStruct<u16, u8>) -> type_names::mod1::Struct2, usize)

-// gdb-command:whatis extern_stdcall_fn_with_return_value
-// gdb-check:type = struct (extern "stdcall" fn(Box<isize>) -> usize, usize)
-
 // gdb-command:whatis generic_function_int
 // gdb-check:type = struct (fn(isize) -> isize, usize)

