diff
diff --git a/src/libstd/sys/unix/fast_thread_local.rs b/src/libstd/sys/unix/fast_thread_local.rs
index d48d701dd5..27d3cae5f6 100644
--- a/src/libstd/sys/unix/fast_thread_local.rs
+++ b/src/libstd/sys/unix/fast_thread_local.rs
@@ -12,23 +12,11 @@
 // Due to rust-lang/rust#18804, make sure this is not generic!
 #[cfg(any(target_os = "linux", target_os = "fuchsia", target_os = "hermit"))]
 pub unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern fn(*mut u8)) {
-    use libc;
-    use mem;
     use sys_common::thread_local::register_dtor_fallback;
 
     extern {
         #[linkage = "extern_weak"]
         static __dso_handle: *mut u8;
-        #[linkage = "extern_weak"]
-        static __cxa_thread_atexit_impl: *const libc::c_void;
-    }
-    if !__cxa_thread_atexit_impl.is_null() {
-        type F = unsafe extern fn(dtor: unsafe extern fn(*mut u8),
-                                  arg: *mut u8,
-                                  dso_handle: *mut u8) -> libc::c_int;
-        mem::transmute::<*const libc::c_void, F>(__cxa_thread_atexit_impl)
-            (dtor, t, &__dso_handle as *const _ as *mut _);
-        return
     }
     register_dtor_fallback(t, dtor);
 }
