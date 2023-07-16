 diff
diff --git a/src/libcore/rand.rs b/src/libcore/rand.rs
index cdf6a5b..5294439 100644
--- a/src/libcore/rand.rs
+++ b/src/libcore/rand.rs
@@ -690,7 +690,7 @@ pub fn task_rng() -> @IsaacRng {

 // Allow direct chaining with `task_rng`
 impl<R: Rng> Rng for @R {
-    fn next(&self) -> u32 { (*self).next() }
+    fn next(&self) -> u32 { (**self).next() }
 }

 /**
