diff
diff --git a/src/librustc_resolve/build_reduced_graph.rs b/src/librustc_resolve/build_reduced_graph.rs
index 11c7793b3ad..afce46395a9 100644
--- a/src/librustc_resolve/build_reduced_graph.rs
+++ b/src/librustc_resolve/build_reduced_graph.rs
@@ -164,7 +164,10 @@ impl<'a> Resolver<'a> {
         }
 
         let ext = Lrc::new(match self.cstore().load_macro_untracked(def_id, &self.session) {
-            LoadedMacro::MacroDef(item, edition) => self.compile_macro(&item, edition),
+            LoadedMacro::MacroDef(item, edition) => {
+                debug!("loading macro {} with DefId {:?}", item.ident.name, def_id);
+                self.compile_macro(&item, edition)
+            }
             LoadedMacro::ProcMacro(ext) => ext,
         });
