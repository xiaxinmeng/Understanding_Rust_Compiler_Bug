diff
diff --git a/tests/run-pass/function_calls/exported_symbol_good_unwind.rs b/tests/run-pass/function_calls/exported_symbol_good_unwind.rs
index 3dd3b8f2..71b799a1 100644
--- a/tests/run-pass/function_calls/exported_symbol_good_unwind.rs
+++ b/tests/run-pass/function_calls/exported_symbol_good_unwind.rs
@@ -2,16 +2,10 @@
 // found in this form" errors works without `-C prefer-dynamic` (`panic!` calls foreign function
 // `__rust_start_panic`).
 // no-prefer-dynamic
-#![feature(c_unwind, unboxed_closures, unwind_attributes)]
+#![feature(c_unwind, unboxed_closures)]
 
 use std::panic;
 
-#[no_mangle]
-#[unwind(allowed)]
-extern "C" fn good_unwind_allowed() {
-    panic!();
-}
-
 #[no_mangle]
 extern "C-unwind" fn good_unwind_c() {
     panic!();
@@ -29,11 +23,6 @@ extern "rust-call" fn good_unwind_rust_call(_: ()) -> ! {
 }
 
 fn main() -> ! {
-    extern "C" {
-        #[unwind(allowed)]
-        fn good_unwind_allowed();
-    }
-    panic::catch_unwind(|| unsafe { good_unwind_allowed() }).unwrap_err();
     extern "C-unwind" {
         fn good_unwind_c();
     }
diff --git a/tests/run-pass/function_calls/exported_symbol_unwind_allowed.rs b/tests/run-pass/function_calls/exported_symbol_unwind_allowed.rs
index 0e4ec573..fd5ec494 100644
--- a/tests/run-pass/function_calls/exported_symbol_unwind_allowed.rs
+++ b/tests/run-pass/function_calls/exported_symbol_unwind_allowed.rs
@@ -1,5 +1,4 @@
-// compile-flags: -Zmiri-disable-abi-check
-#![feature(unwind_attributes, c_unwind)]
+#![feature(c_unwind)]
 
 #[no_mangle]
 extern "C-unwind" fn unwind() {
@@ -7,8 +6,7 @@ extern "C-unwind" fn unwind() {
 }
 
 fn main() {
-    extern "C" {
-        #[unwind(allowed)]
+    extern "C-unwind" {
         fn unwind();
     }
     unsafe { unwind() }
