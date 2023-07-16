diff
 
 async fn foo(x: bool) -> u32 {
     if x {
-        let f = FutureObj::new(Box::new(foo(false)));
+        fn out_of_line() -> FutureObj<'static, u32> {
+            FutureObj::new(Box::new(foo(false)))
+        }
+        let f = out_of_line();
         await!(f) + 1
     }
     else {
