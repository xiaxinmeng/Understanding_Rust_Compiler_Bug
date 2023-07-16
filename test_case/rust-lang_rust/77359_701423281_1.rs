diff
--- main.f.005-007.SimplifyArmIdentity.before.mir
+++ main.f.005-007.SimplifyArmIdentity.after.mir
@@ -1,2 +1,2 @@
-// MIR for `f` before SimplifyArmIdentity
+// MIR for `f` after SimplifyArmIdentity
 
@@ -13,6 +13,6 @@
     scope 1 {
-        debug x => _4;                   // in scope 1 at src/main.rs:15:17: 15:18
+        debug x => ((_7 as Some).0: A);  // in scope 1 at src/main.rs:15:17: 15:18
         let _6: A;                       // in scope 1 at src/main.rs:16:13: 16:14
         scope 2 {
-            debug y => _6;               // in scope 2 at src/main.rs:16:13: 16:14
+            debug y => ((_7 as Some).0: A); // in scope 2 at src/main.rs:16:13: 16:14
         }
@@ -47,9 +47,4 @@
         StorageLive(_6);                 // scope 1 at src/main.rs:16:13: 16:14
-        _6 = move _4;                    // scope 1 at src/main.rs:16:17: 16:18
+        _7 = move _1;                    // scope 2 at src/main.rs:17:21: 17:28
         StorageLive(_7);                 // scope 2 at src/main.rs:17:21: 17:28
-        StorageLive(_8);                 // scope 2 at src/main.rs:17:26: 17:27
-        _8 = move _6;                    // scope 2 at src/main.rs:17:26: 17:27
-        ((_7 as Some).0: A) = move _8;   // scope 2 at src/main.rs:17:21: 17:28
-        discriminant(_7) = 1;            // scope 2 at src/main.rs:17:21: 17:28
-        StorageDead(_8);                 // scope 2 at src/main.rs:17:27: 17:28
         ((_0 as B).0: std::option::Option<A>) = move _7; // scope 2 at src/main.rs:17:16: 17:29
