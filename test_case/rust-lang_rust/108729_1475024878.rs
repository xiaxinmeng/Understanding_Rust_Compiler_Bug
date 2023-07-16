diff
--- a/compiler/rustc_resolve/src/imports.rs
+++ b/compiler/rustc_resolve/src/imports.rs
@@ -423,13 +423,17 @@ fn import_dummy_binding(&mut self, import: &'a Import<'a>) {
     /// Resolves all imports for the crate. This method performs the fixed-
     /// point iteration.
     pub(crate) fn resolve_imports(&mut self) {
-        let mut prev_num_indeterminates = self.indeterminate_imports.len() + 1;
-        while self.indeterminate_imports.len() < prev_num_indeterminates {
-            prev_num_indeterminates = self.indeterminate_imports.len();
+        let mut prev_indeterminate_count = usize::MAX;
+        let mut indeterminate_count = self.indeterminate_imports.len() * 3;
+        while indeterminate_count < prev_indeterminate_count {
+            prev_indeterminate_count = indeterminate_count;
+            indeterminate_count = 0;
             for import in mem::take(&mut self.indeterminate_imports) {
-                match self.resolve_import(&import) {
-                    true => self.determined_imports.push(import),
-                    false => self.indeterminate_imports.push(import),
+                let import_indeterminate_count = self.resolve_import(&import);
+                indeterminate_count += import_indeterminate_count;
+                match import_indeterminate_count {
+                    0 => self.determined_imports.push(import),
+                    _ => self.indeterminate_imports.push(import),
                 }
             }
         }
@@ -583,7 +587,7 @@ fn throw_unresolved_import_error(&self, errors: Vec<(&Import<'_>, UnresolvedImpo
 
     /// Attempts to resolve the given import, returning true if its resolution is determined.
     /// If successful, the resolved bindings are written into the module.
-    fn resolve_import(&mut self, import: &'a Import<'a>) -> bool {
+    fn resolve_import(&mut self, import: &'a Import<'a>) -> usize {
         debug!(
             "(resolving import for module) resolving import `{}::...` in `{}`",
             Segment::names_to_string(&import.module_path),
@@ -601,8 +605,8 @@ fn resolve_import(&mut self, import: &'a Import<'a>) -> bool {
 
             match path_res {
                 PathResult::Module(module) => module,
-                PathResult::Indeterminate => return false,
-                PathResult::NonModule(..) | PathResult::Failed { .. } => return true,
+                PathResult::Indeterminate => return 3,
+                PathResult::NonModule(..) | PathResult::Failed { .. } => return 0,
             }
         };
 
@@ -618,12 +622,12 @@ fn resolve_import(&mut self, import: &'a Import<'a>) -> bool {
             } => (source, target, source_bindings, target_bindings, type_ns_only),
             ImportKind::Glob { .. } => {
                 self.resolve_glob_import(import);
-                return true;
+                return 0;
             }
             _ => unreachable!(),
         };
 
-        let mut indeterminate = false;
+        let mut indeterminate_count = 0;
         self.per_ns(|this, ns| {
             if !type_ns_only || ns == TypeNS {
                 if let Err(Undetermined) = source_bindings[ns].get() {
@@ -646,7 +650,7 @@ fn resolve_import(&mut self, import: &'a Import<'a>) -> bool {
 
                 let parent = import.parent_scope.module;
                 match source_bindings[ns].get() {
-                    Err(Undetermined) => indeterminate = true,
+                    Err(Undetermined) => indeterminate_count += 1,
                     // Don't update the resolution, because it was never added.
                     Err(Determined) if target.name == kw::Underscore => {}
                     Ok(binding) if binding.is_importable() => {
@@ -670,7 +674,7 @@ fn resolve_import(&mut self, import: &'a Import<'a>) -> bool {
             }
         });
 
-        !indeterminate
+        indeterminate_count
     }
 
     /// Performs final import resolution, consistency checks and error reporting.

