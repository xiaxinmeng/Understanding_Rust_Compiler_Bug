`diff
diff --git a/src/vector/avx2.rs b/src/vector/avx2.rs
index 5228c60..b7d10d5 100644
--- a/src/vector/avx2.rs
+++ b/src/vector/avx2.rs
@@ -12,6 +12,7 @@ impl AVX2VectorBuilder {
         if is_x86_feature_detected!("avx2") {
             Some(AVX2VectorBuilder(()))
         } else {
+
             None
         }
     }
