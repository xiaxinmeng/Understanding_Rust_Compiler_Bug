patch
--- a/src/librustc_resolve/resolve_imports.rs
+++ b/src/librustc_resolve/resolve_imports.rs
@@ -764,6 +764,12 @@ impl<'a, 'b> ImportResolver<'a, 'b> {
             match path_res {
                 PathResult::Module(module) => module,
                 PathResult::Indeterminate => return false,
+                PathResult::Failed { .. } if directive.is_glob() => {
+                    // Do not complain about unused import when encountering a bad glob import.
+                    // (#63724)
+                    self.r.used_imports.insert((directive.id, TypeNS));
+                    return true;
+                }
                 PathResult::NonModule(..) | PathResult::Failed { .. } => return true,
             }
         };
