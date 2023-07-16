rust
diff --git a/rustc.main.002-014.InstCombine.before.mir b/rustc.main.002-014.InstCombine.after.mir
index 81357e1..0f5c50c 100644
--- a/rustc.main.002-014.InstCombine.before.mir
+++ b/rustc.main.002-014.InstCombine.after.mir
@@ -1,7 +1,7 @@
 // MIR for `main`
 // source = MirSource { def_id: DefId(0/0:5 ~ x[317d]::main[0]), promoted: None }
 // pass_name = InstCombine
-// disambiguator = before
+// disambiguator = after

 fn main() -> () {
     let mut _0: ();                      // return place
@@ -238,7 +238,14 @@ fn main() -> () {
                                          // <E2><94><94> span: x.rs:16:33: 16:34
                                          // <E2><94><94> ty: usize
                                          // <E2><94><94> literal: const 0usize
-        _54 = Len(_32);                  // bb3[32]: scope 4 at x.rs:16:31: 16:35
+        _54 = const 2usize;              // bb3[32]: scope 4 at x.rs:16:31: 16:35
+                                         // ty::Const
+                                         // <E2><94><94> ty: usize
+                                         // <E2><94><94> val: Integral(Usize(Us64(2)))
+                                         // mir::Constant
+                                         // <E2><94><94> span: x.rs:16:31: 16:35
+                                         // <E2><94><94> ty: usize
+                                         // <E2><94><94> literal: const 2usize
         _55 = Lt(_53, _54);              // bb3[33]: scope 4 at x.rs:16:31: 16:35
         assert(move _55, "index out of bounds: the len is {} but the index is {}", move _54, _53) -> bb4; // bb3[34]: scope 4 at x.rs:16:31: 16:35
     }
