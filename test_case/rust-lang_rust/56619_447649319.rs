diff
diff --git a/src/librustc/ty/mod.rs b/src/librustc/ty/mod.rs
index a1fc949137..227b75a772 100644
--- a/src/librustc/ty/mod.rs
+++ b/src/librustc/ty/mod.rs
@@ -2061,7 +2061,8 @@ impl ReprOptions {
     /// Returns `true` if this `#[repr()]` should inhibit struct field reordering
     /// optimizations, such as with repr(C) or repr(packed(1)).
     pub fn inhibit_struct_field_reordering_opt(&self) -> bool {
-        !(self.flags & ReprFlags::IS_UNOPTIMISABLE).is_empty() || (self.pack == 1)
+        self.flags.intersects(ReprFlags::IS_UNOPTIMISABLE) || self.pack == 1 ||
+            self.int.is_some()
     }
 
     /// Returns true if this `#[repr()]` should inhibit union abi optimisations
