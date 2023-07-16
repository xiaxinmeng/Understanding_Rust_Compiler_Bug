
diff --git a/library/core/src/result.rs b/library/core/src/result.rs
index babd0a0b552..8207ec512c9 100644
--- a/library/core/src/result.rs
+++ b/library/core/src/result.rs
@@ -994,7 +994,7 @@ impl<T, E: fmt::Debug> Result<T, E> {
     pub fn expect(self, msg: &str) -> T {
         match self {
             Ok(t) => t,
-            Err(e) => unwrap_failed(msg, &e),
+            Err(e) => unwrap_failed(msg, e),
         }
     }
 
@@ -1034,7 +1034,7 @@ pub fn expect(self, msg: &str) -> T {
     pub fn unwrap(self) -> T {
         match self {
             Ok(t) => t,
-            Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e),
+            Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", e),
         }
     }
 }
@@ -1061,7 +1061,7 @@ impl<T: fmt::Debug, E> Result<T, E> {
     #[stable(feature = "result_expect_err", since = "1.17.0")]
     pub fn expect_err(self, msg: &str) -> E {
         match self {
-            Ok(t) => unwrap_failed(msg, &t),
+            Ok(t) => unwrap_failed(msg, t),
             Err(e) => e,
         }
     }
@@ -1089,7 +1089,7 @@ pub fn expect_err(self, msg: &str) -> E {
     #[stable(feature = "rust1", since = "1.0.0")]
     pub fn unwrap_err(self) -> E {
         match self {
-            Ok(t) => unwrap_failed("called `Result::unwrap_err()` on an `Ok` value", &t),
+            Ok(t) => unwrap_failed("called `Result::unwrap_err()` on an `Ok` value", t),
             Err(e) => e,
         }
     }
@@ -1351,7 +1351,7 @@ pub const fn into_ok_or_err(self) -> T {
 #[inline(never)]
 #[cold]
 #[track_caller]
-fn unwrap_failed(msg: &str, error: &dyn fmt::Debug) -> ! {
+fn unwrap_failed<E: fmt::Debug>(msg: &str, error: E) -> ! {
     panic!("{}: {:?}", msg, error)
 }
 