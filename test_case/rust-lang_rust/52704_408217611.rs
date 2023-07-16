
commit d01427f53e81a24e1c49d1672f37d4809c04a004
Author: Alex Crichton <alex@alexcrichton.com>
Date:   Fri Oct 20 11:40:04 2017 -0700

    [test] Add some `#[inline]` to `HashMap`

diff --git a/src/libstd/collections/hash/map.rs b/src/libstd/collections/hash/map.rs
index 3c0fa0860d..d96a4f40d3 100644
--- a/src/libstd/collections/hash/map.rs
+++ b/src/libstd/collections/hash/map.rs
@@ -1113,6 +1113,7 @@ impl<K, V, S> HashMap<K, V, S>
     /// assert_eq!(map.get(&2), None);
     /// 