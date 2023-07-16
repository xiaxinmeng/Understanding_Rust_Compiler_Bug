patch
diff --git a/src/lib.rs b/src/lib.rs
index 19b10697..c51f498e 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -5,6 +5,4 @@ pub struct ConstGeneric<const CHUNK_SIZE: usize> {
     _p: [(); CHUNK_SIZE],
 }

-/////// MOVE START
-impl<const CHUNK_SIZE: usize> ConstGeneric<CHUNK_SIZE> {}
 /////// MOVE END
