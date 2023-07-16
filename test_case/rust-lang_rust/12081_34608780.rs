 diff
@@ -1025,9 +1025,12 @@ impl<K: Hash + Eq, V> HashMap<K, V> {

             if table::hash_of(&idx) == hash {
                 let (bucket_k, bucket_v) = self.table.read_mut(&idx);
+                // FIXME #12147 the conditional return confuses
+                // borrowck if we return bucket_v directly
+                let bv: *mut V = bucket_v;
                 if k == *bucket_k {
                     // Key already exists. Get its reference.
-                    return bucket_v;
+                    return unsafe {&mut *bv};
                 }
             }
