
+declare_clippy_lint! {
+    pub RANDOM_STATE,
+    nursery,
+    "use of RandomState"
+}
+
+pub struct Pass;
+
+impl LintPass for Pass {
+    fn get_lints(&self) -> LintArray {
+        lint_array!(RANDOM_STATE)
+    }
+}
+
+impl<'a, 'tcx> LateLintPass<'a, 'tcx> for Pass {
+    fn check_ty(&mut self, cx: &LateContext<'a, 'tcx>, ty: &Ty) {
+        if let Some(tys) = cx.tables.node_id_to_type_opt(ty.hir_id) {
+            if let TyKind::Adt(_, substs) = tys.sty {
+                for subst in substs {
+                    if let UnpackedKind::Type(build_hasher) = subst.unpack() {
+                        if match_type(cx, build_hasher, &paths::RANDOM_STATE) {
+                            span_lint(cx, RANDOM_STATE, ty.span, "usage of RandomState");
+                        }
+                    }
+                }
+            }
+        }
+    }
+}
+
diff --git a/clippy_lints/src/utils/paths.rs b/clippy_lints/src/utils/paths.rs
index 0779d779..0ec684e3 100644
--- a/clippy_lints/src/utils/paths.rs
+++ b/clippy_lints/src/utils/paths.rs
@@ -73,6 +73,7 @@ pub const PATH_BUF: [&str; 3] = ["std", "path", "PathBuf"];
 pub const PATH_TO_PATH_BUF: [&str; 4] = ["std", "path", "Path", "to_path_buf"];
 pub const PTR_NULL: [&str; 2] = ["ptr", "null"];
 pub const PTR_NULL_MUT: [&str; 2] = ["ptr", "null_mut"];
+pub const RANDOM_STATE: [&str; 5] = ["std", "collections", "hash", "map", "RandomState"];
 pub const RANGE: [&str; 3] = ["core", "ops", "Range"];
 pub const RANGE_ARGUMENT_TRAIT: [&str; 3] = ["core", "ops", "RangeBounds"];
 pub const RANGE_FROM: [&str; 3] = ["core", "ops", "RangeFrom"];
diff --git a/tests/run-pass/used_underscore_binding_macro.rs b/tests/run-pass/used_underscore_binding_macro.rs
index 8b6c6557..dc590cbc 100644
--- a/tests/run-pass/used_underscore_binding_macro.rs
+++ b/tests/run-pass/used_underscore_binding_macro.rs
@@ -8,6 +8,7 @@
 // except according to those terms.
 
 #![allow(clippy::useless_attribute)] //issue #2910
+#![allow(clippy::random_state)]
 
 #[macro_use]
 extern crate serde_derive;
diff --git a/tests/ui/random_state.rs b/tests/ui/random_state.rs
new file mode 100644
index 00000000..9f278ecc
--- /dev/null
+++ b/tests/ui/random_state.rs
@@ -0,0 +1,20 @@
+#![warn(clippy::random_state)]
+
+use std::collections::hash_map::RandomState;
+use std::collections::hash_map::{DefaultHasher, HashMap};
+use std::hash::BuildHasherDefault;
+
+fn main() {
+    // Should warn
+    let mut map = HashMap::new();
+    map.insert(3, 4);
+    let mut map = HashMap::with_hasher(RandomState::new());
+    map.insert(true, false);
+    let _map: HashMap<_, _> = vec![(2, 3)].into_iter().collect();
+    let _vec: Vec<HashMap<i32, i32>>;
+    // Shouldn't warn
+    let _map: HashMap<i32, i32, BuildHasherDefault<DefaultHasher>> = HashMap::default();
+    let mut map = HashMap::with_hasher(BuildHasherDefault::<DefaultHasher>::default());
+    map.insert("a", "b");
+}
+
diff --git a/tests/ui/random_state.stderr b/tests/ui/random_state.stderr
new file mode 100644
index 00000000..df224bf0
--- /dev/null
+++ b/tests/ui/random_state.stderr
@@ -0,0 +1,28 @@
+error: usage of RandomState
+  --> $DIR/random_state.rs:9:19
+   |
+LL |     let mut map = HashMap::new();
+   |                   ^^^^^^^^^^^^
+   |
+   = note: `-D clippy::random-state` implied by `-D warnings`
+
+error: usage of RandomState
+  --> $DIR/random_state.rs:11:19
+   |
+LL |     let mut map = HashMap::with_hasher(RandomState::new());
+   |                   ^^^^^^^^^^^^^^^^^^^^
+
+error: usage of RandomState
+  --> $DIR/random_state.rs:13:15
+   |
+LL |     let _map: HashMap<_, _> = vec![(2, 3)].into_iter().collect();
+   |               ^^^^^^^^^^^^^
+
+error: usage of RandomState
+  --> $DIR/random_state.rs:14:19
+   |
+LL |     let _vec: Vec<HashMap<i32, i32>>;
+   |                   ^^^^^^^^^^^^^^^^^
+
+error: aborting due to 4 previous errors
+
