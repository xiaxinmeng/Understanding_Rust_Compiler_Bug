
diff --git a/src/libcore/hashmap.rs b/src/libcore/hashmap.rs
index d2be041..c31905b 100644
--- a/src/libcore/hashmap.rs
+++ b/src/libcore/hashmap.rs
@@ -56,7 +56,7 @@ fn resize_at(capacity: uint) -> uint {
 pub fn linear_map_with_capacity<K:Eq + Hash,V>(
     initial_capacity: uint) -> HashMap<K, V> {
     let r = rand::task_rng();
-    linear_map_with_capacity_and_keys(r.gen(), r.gen(),
+    linear_map_with_capacity_and_keys((*r).gen(), (*r).gen(),
                                       initial_capacity)
 }
