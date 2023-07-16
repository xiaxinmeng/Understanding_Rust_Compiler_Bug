 diff
diff --git a/src/libsyntax/parse/parser.rs b/src/libsyntax/parse/parser.rs
index bef2068..6f75536 100644
--- a/src/libsyntax/parse/parser.rs
+++ b/src/libsyntax/parse/parser.rs
@@ -896,7 +925,9 @@ impl<'a> Parser<'a> {
     pub fn bump(&mut self) -> PResult<()> {
         self.last_span = self.span;
         // Stash token for error recovery (sometimes; clone is not necessarily cheap).
-        self.last_token = if self.token.is_ident() || self.token.is_path() {
+        self.last_token = if self.token.is_ident() ||
+                             self.token.is_path() ||
+                             self.token == token::Comma {
             Some(box self.token.clone())
         } else {
             None
@@ -3796,8 +3914,24 @@ impl<'a> Parser<'a> {
     fn parse_generic_values_after_lt(&mut self) -> PResult<(Vec<ast::Lifetime>,
                                                             Vec<P<Ty>>,
                                                             Vec<P<TypeBinding>>)> {
+
+        debug!("###{} {} token: {:?}, lo: {:?}, hi: {:?}", line!(), "generic_values_after_lt", self.token, self.span.lo, self.span.hi);
         let lifetimes = try!(self.parse_lifetimes(token::Comma));

+        if !lifetimes.is_empty() && self.last_token != Some(box token::Comma) {
+            match self.token {
+                token::Gt | token::Ge |
+                token::BinOpEq(token::Shr) | token::BinOp(token::Shr) => {}
+                _ => {
+                    let this_token_str = self.this_token_to_string();
+                    let msg = format!("expected `,` or `>` after lifetime \
+                                      name, found `{}`",
+                                      this_token_str);
+                    return Err(self.fatal(&msg));
+                }
+            }
+        }
+
         // First parse types.
         let (types, returned) = try!(self.parse_seq_to_gt_or_return(
             Some(token::Comma),
