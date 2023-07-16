diff
@@ -2,9 +2,8 @@ use std::io::{self, *};
 use std::sync::atomic::*;
 
 fn main() {
-    //std::thread::spawn(|| panic!()).join().unwrap_err();
-    //println!("main after join");
-    panic_in_write_doesnt_flush_in_drop();
+    std::thread::spawn(|| panic!()).join().unwrap_err();
+    println!("main after join");
 }
 #[test]
 fn panic_in_write_doesnt_flush_in_drop() {
