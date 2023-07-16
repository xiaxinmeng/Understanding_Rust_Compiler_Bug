diff
diff --git a/src/libsyntax/parse/parser.rs b/src/libsyntax/parse/parser.rs
index fb5ff7e8f9..665308eda2 100644
--- a/src/libsyntax/parse/parser.rs
+++ b/src/libsyntax/parse/parser.rs
@@ -3620,7 +3620,15 @@ impl<'a> Parser<'a> {
         let mut etc_span = None;
 
         while self.token != token::CloseDelim(token::Brace) {
-            let attrs = self.parse_outer_attributes()?;
+            let attrs = match self.parse_outer_attributes() {
+                Ok(attrs) => attrs,
+                Err(err) => {
+                    if let Some(mut delayed) = delayed_err {
+                        delayed.emit();
+                    }
+                    return Err(err);
+                },
+            };
             let lo = self.token.span;
 
             // check that a comma comes after every field
