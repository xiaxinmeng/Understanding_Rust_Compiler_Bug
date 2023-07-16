diff
diff --git a/src/link.rs b/src/link.rs
index 543a5a5..03f37b6 100644
--- a/src/link.rs
+++ b/src/link.rs
@@ -23,8 +23,11 @@ macro_rules! link {
     (@LOAD: #[cfg($cfg:meta)] fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*) => (
         #[cfg($cfg)]
         pub fn $name(library: &mut super::SharedLibrary) {
-            let symbol = unsafe { library.library.get(stringify!($name).as_bytes()) }.ok();
-            library.functions.$name = symbol.map(|s| *s);
+            use libloading::Symbol;
+            use super::*;
+            let symbol: Option<Symbol<Option<unsafe extern fn($($pty,)*) -> _>>> =
+                unsafe { library.library.get(stringify!($name).as_bytes()) }.ok();
+            library.functions.$name = symbol.and_then(|s| *s);
         }
 
         #[cfg(not($cfg))]
