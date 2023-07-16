
diff --git a/compiler/rustc_ast/src/mut_visit.rs b/compiler/rustc_ast/src/mut_visit.rs
index 87950b44083..45acd5b4295 100644
--- a/compiler/rustc_ast/src/mut_visit.rs
+++ b/compiler/rustc_ast/src/mut_visit.rs
@@ -20,7 +20,6 @@
 
 use smallvec::{smallvec, Array, SmallVec};
 use std::ops::DerefMut;
-use std::{panic, process, ptr};
 
 pub trait ExpectOne<A: Array> {
     fn expect_one(self, err: &'static str) -> A::Item;
@@ -288,18 +287,13 @@ fn visit_span(&mut self, _sp: &mut Span) {
 /// method. Abort the program if the closure panics.
 //
 // No `noop_` prefix because there isn't a corresponding method in `MutVisitor`.
-pub fn visit_clobber<T, F>(t: &mut T, f: F)
+pub fn visit_clobber<T : Clone, F>(t: &mut T, f: F)
 where
     F: FnOnce(T) -> T,
 {
-    unsafe {
-        // Safe because `t` is used in a read-only fashion by `read()` before
-        // being overwritten by `write()`.
-        let old_t = ptr::read(t);
-        let new_t = panic::catch_unwind(panic::AssertUnwindSafe(|| f(old_t)))
-            .unwrap_or_else(|_| process::abort());
-        ptr::write(t, new_t);
-    }
+    let old_t = t.clone();
+    let new_t = f(old_t);
+    *t = new_t;
 }
