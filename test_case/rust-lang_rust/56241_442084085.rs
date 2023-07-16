diff
diff --git a/src/librustc_resolve/Cargo.toml b/src/librustc_resolve/Cargo.toml
index 3a8e84a328..70dcd226fc 100644
--- a/src/librustc_resolve/Cargo.toml
+++ b/src/librustc_resolve/Cargo.toml
@@ -19,3 +19,4 @@ rustc_errors = { path = "../librustc_errors" }
 syntax_pos = { path = "../libsyntax_pos" }
 rustc_data_structures = { path = "../librustc_data_structures" }
 rustc_metadata = { path = "../librustc_metadata" }
+hashbrown = { git = "https://github.com/Amanieu/hashbrown.git", branch = "sta>
diff --git a/src/librustc_resolve/lib.rs b/src/librustc_resolve/lib.rs
index 443b1ccdef..bb31c17521 100644
--- a/src/librustc_resolve/lib.rs
+++ b/src/librustc_resolve/lib.rs
@@ -30,6 +30,7 @@ extern crate arena;
 extern crate rustc;
 extern crate rustc_data_structures;
 extern crate rustc_metadata;
+extern crate hashbrown;
 
 pub use rustc::hir::def::{Namespace, PerNS};
 
@@ -1074,7 +1075,7 @@ pub struct ModuleData<'a> {
     builtin_attrs: RefCell<Vec<(Ident, ParentScope<'a>)>>,
 
     // Macro invocations that can expand into items in this module.
-    unresolved_invocations: RefCell<FxHashSet<Mark>>,
+    unresolved_invocations: RefCell<hashbrown::HashSet<Mark>>,
 
     no_implicit_prelude: bool,
 
