diff
diff --git a/src/tools/clippy/tests/ui/crashes/used_underscore_binding_macro.rs b/src/tools/clippy/tests/ui/crashes/used_underscore_binding_macro.rs
index 6d2124c12fe..c57a45dc7aa 100644
--- a/src/tools/clippy/tests/ui/crashes/used_underscore_binding_macro.rs
+++ b/src/tools/clippy/tests/ui/crashes/used_underscore_binding_macro.rs
@@ -1,10 +1,9 @@
-#![allow(clippy::useless_attribute)] //issue #2910
+// edition:2018

-#[macro_use]
-extern crate serde_derive;
+use serde::Deserialize;

 /// Tests that we do not lint for unused underscores in a `MacroAttribute`
 /// expansion
 #[deny(clippy::used_underscore_binding)]
 #[derive(Deserialize)]
 struct MacroAttributesTest {
