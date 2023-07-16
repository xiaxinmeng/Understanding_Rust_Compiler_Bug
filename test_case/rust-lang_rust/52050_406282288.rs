diff
diff --git a/src/lib.rs b/src/lib.rs
index 729185a..640234b 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -197,23 +197,23 @@ macro_rules! wrap_function (
     };
 );
 
-pub mod python;
-mod err;
+#[doc(hidden)]
+pub mod argparse;
+pub mod buffer;
+#[doc(hidden)]
+pub mod callback;
 mod conversion;
+mod err;
+pub mod freelist;
 mod instance;
+mod noargs;
 mod object;
-mod objects;
 mod objectprotocol;
-mod noargs;
+mod objects;
+pub mod prelude;
+pub mod python;
 mod pythonrun;
-#[doc(hidden)]
-pub mod callback;
 pub mod typeob;
-#[doc(hidden)]
-pub mod argparse;
-pub mod buffer;
-pub mod freelist;
-pub mod prelude;
 
 // re-export for simplicity
 #[doc(hidden)]
