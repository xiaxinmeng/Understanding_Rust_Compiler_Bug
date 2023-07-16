
diff --git a/compiler/rustc_resolve/src/late/diagnostics.rs b/compiler/rustc_resolve/src/late/diagnostics.rs
index b05ec654997..af487381702 100644
--- a/compiler/rustc_resolve/src/late/diagnostics.rs
+++ b/compiler/rustc_resolve/src/late/diagnostics.rs
@@ -2166,7 +2166,15 @@ impl<'tcx> LifetimeContext<'_, 'tcx> {
                                     }
                             )
                         }) {
-                            (param.span.shrink_to_lo(), "'a, ".to_string())
+                            match param.kind {
+                                hir::GenericParamKind::Const { .. }  => {
+                                    println!("TODO: {:#?}", param);
+                                    (param.span.shrink_to_lo(), "'a, ".to_string())
+                                },
+                                _ => {
+                                    (param.span.shrink_to_lo(), "'a, ".to_string())
+                                }
+                            }
                         } else {
                             (generics.span, "<'a>".to_string())
                         }
