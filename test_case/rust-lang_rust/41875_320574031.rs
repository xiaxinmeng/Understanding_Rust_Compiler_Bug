
diff --git a/src/test/run-pass/typeid-intrinsic.rs b/src/test/run-pass/typeid-intrinsic.rs
index 3becb8c..3fc2737 100644
--- a/src/test/run-pass/typeid-intrinsic.rs
+++ b/src/test/run-pass/typeid-intrinsic.rs
@@ -18,6 +18,7 @@ extern crate typeid_intrinsic_aux2 as other2;
 
 use std::hash::{SipHasher, Hasher, Hash};
 use std::any::TypeId;
+use std::intrinsics::type_id;
 
 struct A;
 struct Test;
@@ -98,10 +99,10 @@ pub fn main() {
     assert!(TypeId::of::<fn(fn(A) -> A) -> A>() !=
             TypeId::of::<fn(fn() -> A, A) -> A>());
 
-    // non-static lifetimes are OK but ignored
+    // non-static lifetimes are OK but ignored for the raw type_id intrinsic
     fn non_static<'a>(arg: &'a str) {
-        assert_eq!(TypeId::of::<ContainsRef<'a>>(),
-                   TypeId::of::<ContainsRef<'static>>());
+        assert_eq!(type_id::<ContainsRef<'a>>(),
+                   type_id::<ContainsRef<'static>>());
     }
