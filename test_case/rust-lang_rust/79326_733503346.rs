diff
diff --git src/tools/cargotest/main.rs src/tools/cargotest/main.rs
index 8aabe077cf1..dc09e7ffc1d 100644
--- src/tools/cargotest/main.rs
+++ src/tools/cargotest/main.rs
@@ -43,7 +43,7 @@ struct Test {
     Test {
         name: "servo",
         repo: "https://github.com/servo/servo",
-        sha: "caac107ae8145ef2fd20365e2b8fadaf09c2eb3b",
+        sha: "90e8e19f5e53fe35ab15e372c359ba681c8bc843",
         lock: None,
         // Only test Stylo a.k.a. Quantum CSS, the parts of Servo going into Firefox.
         // This takes much less time to build than all of Servo and supports stable Rust.