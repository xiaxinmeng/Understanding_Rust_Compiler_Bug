
--- src/test/debuginfo/type-names.rs.orig       2016-09-10 19:14:05.466433219 +0000
+++ src/test/debuginfo/type-names.rs    2016-09-10 19:12:35.384131562 +0000
@@ -224,7 +224,7 @@
 fn rust_fn_with_return_value(_: f64) -> usize { 4 }
 extern "C" fn extern_c_fn_with_return_value() -> Struct1 { Struct1 }
 unsafe fn unsafe_fn_with_return_value(_: GenericStruct<u16, u8>) -> mod1::Struct2 { mod1::Struct2 }
-extern "stdcall" fn extern_stdcall_fn_with_return_value(_: Box<isize>) -> usize { 0 }
+//extern "stdcall" fn extern_stdcall_fn_with_return_value(_: Box<isize>) -> usize { 0 }

 fn generic_function<T>(x: T) -> T { x }

@@ -315,7 +315,7 @@
     let rust_fn_with_return_value = (rust_fn_with_return_value, 0_usize);
     let extern_c_fn_with_return_value = (extern_c_fn_with_return_value, 0_usize);
     let unsafe_fn_with_return_value = (unsafe_fn_with_return_value, 0_usize);
-    let extern_stdcall_fn_with_return_value = (extern_stdcall_fn_with_return_value, 0_usize);
+//    let extern_stdcall_fn_with_return_value = (extern_stdcall_fn_with_return_value, 0_usize);

     let generic_function_int = (generic_function::<isize>, 0_usize);
     let generic_function_struct3 = (generic_function::<mod1::mod2::Struct3>, 0_usize);
