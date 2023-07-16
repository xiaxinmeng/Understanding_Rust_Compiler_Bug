 diff
diff --git a/src/librand/isaac.rs b/src/librand/isaac.rs
index 0f7cda4..d80999e 100644
--- a/src/librand/isaac.rs
+++ b/src/librand/isaac.rs
@@ -185,7 +185,7 @@ impl Rng for IsaacRng {
             self.isaac();
         }
         self.cnt -= 1;
-        self.rsl[self.cnt as uint]
+        self.rsl[self.cnt as u8 as uint]
     }
 }

@@ -416,7 +416,7 @@ impl Rng for Isaac64Rng {
             self.isaac64();
         }
         self.cnt -= 1;
-        unsafe { *self.rsl.unsafe_get(self.cnt) }
+        self.rsl[self.cnt as u8 as uint]
     }
 }
