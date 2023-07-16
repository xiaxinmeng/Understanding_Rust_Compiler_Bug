
diff --git a/src/doc/intro.md b/src/doc/intro.md
index 90a018c..d9bfe71 100644
--- a/src/doc/intro.md
+++ b/src/doc/intro.md
@@ -480,7 +480,7 @@ use std::sync::{Arc,Mutex};
 fn main() {
     let numbers = Arc::new(Mutex::new(vec![1, 2, 3]));

-    for i in 0us..3 {
+    for i in 0..3 {
         let number = numbers.clone();
         Thread::spawn(move || {
             let mut array = number.lock().unwrap();
@@ -541,7 +541,7 @@ use std::thread::Thread;
 fn main() {
     let vec = vec![1, 2, 3];

-    for i in 0us..3 {
+    for i in 0..3 {
         Thread::spawn(move || {
             println!("{}", vec[i]);
         });
