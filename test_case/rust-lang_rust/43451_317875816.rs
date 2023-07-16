diff
--- a/src/libsyntax/parse/parser.rs
+++ b/src/libsyntax/parse/parser.rs
@@ -3523,8 +3523,18 @@ impl<'a> Parser<'a> {
             }
             // At this point, token != _, &, &&, (, [
             _ => if self.eat_keyword(keywords::Mut) {
-                // Parse mut ident @ pat
-                pat = self.parse_pat_ident(BindingMode::ByValue(Mutability::Mutable))?;
+                // Parse mut ident @ pat / mut ref ident @ pat
+                let mutref_span = self.prev_span.to(self.span);
+                let binding_mode = if self.eat_keyword(keywords::Ref) {
+                    self.diagnostic()
+                        .struct_span_err(mutref_span, "the order of `mut` and `ref` is incorrect")
+                        .span_suggestion(mutref_span, "try switching the order", "ref mut".into())
+                        .emit();
+                    BindingMode::ByRef(Mutability::Mutable)
+                } else {
+                    BindingMode::ByValue(Mutability::Mutable)
+                };
+                pat = self.parse_pat_ident(binding_mode)?;
             } else if self.eat_keyword(keywords::Ref) {
                 // Parse ref ident @ pat / ref mut ident @ pat
                 let mutbl = self.parse_mutability();
