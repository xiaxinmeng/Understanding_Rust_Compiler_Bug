diff
--- before
+++ after
@@ -10,15 +10,10 @@
     if let Some(x) = root {
         queue.push(x);
     }
-    while !queue.is_empty() {
-        let x = queue.pop().unwrap();
+    while let Some(x) = queue.pop() {
         if let Some(next) = x.borrow().next.clone() {
             queue.push(next);
         }
-        // uncommenting any of the following will compile
-        // ()
-        // 0;
-        // let completely_random_var = 0;
     }
 }
