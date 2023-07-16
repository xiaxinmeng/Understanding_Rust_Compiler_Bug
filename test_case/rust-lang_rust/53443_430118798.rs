patch
diff --git a/src/librustc_resolve/build_reduced_graph.rs b/src/librustc_resolve/build_reduced_graph.rs
index 25a7ff9cd3..02a28a1e2f 100644
--- a/src/librustc_resolve/build_reduced_graph.rs
+++ b/src/librustc_resolve/build_reduced_graph.rs
@@ -736,6 +736,9 @@ impl<'a, 'cl> Resolver<'a, 'cl> {
             Def::Macro(..) => {
                 self.define(parent, ident, MacroNS, (def, vis, DUMMY_SP, expansion));
             }
+            Def::Existential(..) => {
+                self.define(parent, ident, TypeNS, (def, vis, DUMMY_SP, expansion));
+            }
             _ => bug!("unexpected definition: {:?}", def)
         }
     }
