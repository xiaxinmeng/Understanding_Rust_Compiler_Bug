
% git diff pcw-by-value-upvars
diff --git a/src/doc/guide-container.md b/src/doc/guide-container.md
index 30bfd28..9acb38a 100644
--- a/src/doc/guide-container.md
+++ b/src/doc/guide-container.md
@@ -184,8 +184,9 @@ let xs = [1,2,3,4,5];
 let mut calls = 0;

 {
+    let calls_ref = &mut calls;
     let it = xs.iter().scan((), |_, x| {
-        calls += 1;
+        *calls_ref += 1;
         if *x < 3 { Some(x) } else { None }});

     // the iterator will only yield 1 and 2 before returning None
diff --git a/src/doc/tutorial.md b/src/doc/tutorial.md
index 011d1b2..79b05d3 100644
--- a/src/doc/tutorial.md
+++ b/src/doc/tutorial.md
@@ -1838,7 +1838,8 @@ access local variables in the enclosing scope.

 let mut max = 0;
-let f = |x: int| if x > max { max = x };
+let max_ref = &mut max;
+let f = |x: int| if x > *max_ref { *max_ref = x };
 for x in [1, 2, 3].iter() {
     f(*x);
 }
diff --git a/src/librustc/util/common.rs b/src/librustc/util/common.rs
index 66da8f1..02de185 100644
--- a/src/librustc/util/common.rs
+++ b/src/librustc/util/common.rs
@@ -31,7 +31,7 @@ impl Timer {
         depth.replace(Some(old_depth + 1));

         Timer {
-            start: time::precise_time_s(),
+            start: if enabled { time::precise_time_s() } else { 0f64 },
             what: what,
             enabled: enabled,
             old_depth: old_depth,
@@ -41,12 +41,14 @@ impl Timer {

 impl Drop for Timer {
     fn drop(&mut self) {
-        let end = time::precise_time_s();
         depth.replace(Some(self.old_depth));
-        println!("{}time: {:3.3f} s\t{}",
-                 "  ".repeat(self.old_depth),
-                 end - self.start,
-                 self.what);
+        if self.enabled {
+            let end = time::precise_time_s();
+            println!("{}time: {:3.3f} s\t{}",
+                     "  ".repeat(self.old_depth),
+                     end - self.start,
+                     self.what);
+        }
     }
 }

diff --git a/src/llvm b/src/llvm
index 4b4d053..0a89464 160000
--- a/src/llvm
+++ b/src/llvm
@@ -1 +1 @@
-Subproject commit 4b4d0533b4f76cc3fbba31bd9e7ac02e0c738b1d
+Subproject commit 0a894645cf120539876e9eb4eb0d7b572dfa9d14
