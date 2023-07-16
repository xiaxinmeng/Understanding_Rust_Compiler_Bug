diff
diff --git a/benches/bench.rs b/benches/bench.rs
index 8bad8d0..904788f 100644
--- a/benches/bench.rs
+++ b/benches/bench.rs
@@ -32,7 +32,10 @@ impl Case {
     fn pos(&self, size: usize) -> usize {
         match self {
             Case::Best => size / 2,
-            Case::Average => 2_f32.powf((size as f32).log2() / 2.) as usize,
+            Case::Average => {
+                let half_steps = (((size as f32).log2() + 1.) / 2.).ceil();
+                size / 2_f32.powf(half_steps) as usize
+            }
             Case::Worst => 0,
         }
     }
@@ -56,9 +59,8 @@ where
 
 fn std_bench_binary_search_case(b: &mut Bencher, cache: Cache, case: Case) {
     let size = cache.size();
-    let mut v = vec![0; size];
-    let i = 1;
-    v[case.pos(size)] = i;
+    let v = (0..size).collect::<Vec<_>>();
+    let i = case.pos(size);
     b.iter(move || {
         black_box(std_binary_search(&v, &i).is_ok());
     })
@@ -157,9 +159,8 @@ where
 
 fn stdnew_bench_binary_search_case(b: &mut Bencher, cache: Cache, case: Case) {
     let size = cache.size();
-    let mut v = vec![0; size];
-    let i = 1;
-    v[case.pos(size)] = i;
+    let v = (0..size).collect::<Vec<_>>();
+    let i = case.pos(size);
     b.iter(move || {
         black_box(stdnew_binary_search(&v, &i).is_ok());
     })
