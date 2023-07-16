diff
diff --git a/src/main.rs b/src/main.rs
index c66bd3d..58d322f 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,7 +1,9 @@
+#![feature(c_unwind)]
+
 mod ffi {
     use std::os::raw::{c_int, c_void};

-    type LuaCFn = unsafe extern "C" fn(L: *mut c_void) -> c_int;
+    type LuaCFn = unsafe extern "C-unwind" fn(L: *mut c_void) -> c_int;

     extern "C" {
         pub fn luaL_newstate() -> *mut c_void;
@@ -21,7 +23,7 @@ pub unsafe fn test_me() {
     let state = ffi::luaL_newstate();
     assert!(!state.is_null());

-    unsafe extern "C" fn run_me(state: *mut std::os::raw::c_void) -> std::os::raw::c_int {
+    unsafe extern "C-unwind" fn run_me(state: *mut std::os::raw::c_void) -> std::os::raw::c_int {
         ffi::lua_pushnil(state);
         ffi::lua_pushnil(state);
         ffi::lua_gettable2(state, -2); // Changing this to `ffi::lua_gettable` solves the problem
