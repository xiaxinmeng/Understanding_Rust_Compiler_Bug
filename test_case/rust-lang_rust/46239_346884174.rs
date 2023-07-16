diff
diff --git a/src/link.rs b/src/link.rs
index 543a5a5..bc81d96 100644
--- a/src/link.rs
+++ b/src/link.rs
@@ -23,7 +23,10 @@ macro_rules! link {
     (@LOAD: #[cfg($cfg:meta)] fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*) => (
         #[cfg($cfg)]
         pub fn $name(library: &mut super::SharedLibrary) {
-            let symbol = unsafe { library.library.get(stringify!($name).as_bytes()) }.ok();
+            use libloading::Symbol;
+            use super::*;
+            let symbol: Option<Symbol<unsafe extern fn($($pty,)*) -> _>> =
+                unsafe { library.library.get(stringify!($name).as_bytes()) }.ok();
             library.functions.$name = symbol.map(|s| *s);
         }
