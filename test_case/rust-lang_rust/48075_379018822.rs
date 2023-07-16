patch
diff --git a/src/libsyntax/ext/tt/quoted.rs b/src/libsyntax/ext/tt/quoted.rs
index f324ede..a6fb782 100644
--- a/src/libsyntax/ext/tt/quoted.rs
+++ b/src/libsyntax/ext/tt/quoted.rs
@@ -470,8 +470,11 @@ where
                         GateIssue::Language,
                         explain,
                     );
+                } else {
+                    sess.span_diagnostic
+                        .span_err(span, "`?` macro repetition does not allow a separator");
                 }
-                return (Some(tok), op);
+                return (None, op);
             }
             Ok(Ok(op)) => return (Some(tok), op),
