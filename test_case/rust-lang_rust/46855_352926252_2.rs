rust
diff --git a/working.mir b/broken.mir
index b6ba84c..5d8b34a 100644
--- a/working.mir
+++ b/broken.mir
@@ -213,7 +213,14 @@ fn main() -> () {
                                          // <E2><94><94> span: x.rs:16:33: 16:34
                                          // <E2><94><94> ty: usize
                                          // <E2><94><94> literal: const 0usize
-        _36 = Len(_19);                  // bb3[32]: scope 4 at x.rs:16:31: 16:35
+        _36 = const 2usize;              // bb3[32]: scope 4 at x.rs:16:31: 16:35
         _37 = Lt(_35, _36);              // bb3[33]: scope 4 at x.rs:16:31: 16:35
         assert(move _37, "index out of bounds: the len is {} but the index is {}", move _36, _35) -> bb4; // bb3[34]: scope 4 at x.rs:16:31: 16:35
     }
