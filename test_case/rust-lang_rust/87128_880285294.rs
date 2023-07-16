diff
diff --git a/library/core/src/slice/mod.rs b/library/core/src/slice/mod.rs
index de25c984abf..e9966a418ef 100644
--- a/library/core/src/slice/mod.rs
+++ b/library/core/src/slice/mod.rs
@@ -701,6 +701,7 @@ pub fn reverse(&mut self) {
     /// assert_eq!(iterator.next(), Some(&4));
     /// assert_eq!(iterator.next(), None);
     /// 