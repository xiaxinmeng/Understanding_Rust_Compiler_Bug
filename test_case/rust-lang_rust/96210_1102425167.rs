plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/parser/mod.rs at line 263:
     fn inlined_next(&mut self, desugar_doc_comments: bool) -> (Token, Spacing) {
         if !self.frame.open_delim {
             self.frame.open_delim = true;
-            (
-                Token::new(token::OpenDelim(self.frame.delim), self.frame.span.open),
-                Spacing::Alone,
-            )
+            (Token::new(token::OpenDelim(self.frame.delim), self.frame.span.open), Spacing::Alone)
         } else if let Some((tree, spacing)) = self.frame.tree_cursor.next_with_spacing() {
             match tree {
-                TokenTree::Token(token) => {
-                    match (desugar_doc_comments, &token) {
-                        (true, &Token { kind: token::DocComment(_, attr_style, data), span }) =>
-                            self.desugar(attr_style, data, span),
-                        _ => (token, spacing),
+                TokenTree::Token(token) => match (desugar_doc_comments, &token) {
+                    (true, &Token { kind: token::DocComment(_, attr_style, data), span }) => {
+                        self.desugar(attr_style, data, span)
-                }
-                }
+                    _ => (token, spacing),
+                },
                 TokenTree::Delimited(sp, delim, tts) => {
                     // Set `open_delim` to true here because we deal with it immediately.
                     let frame = TokenCursorFrame::new(sp, delim, true, tts, false);
Diff in /checkout/compiler/rustc_parse/src/parser/mod.rs at line 285:
             }
         } else if !self.frame.close_delim {
             self.frame.close_delim = true;
-            (
-                Token::new(token::CloseDelim(self.frame.delim), self.frame.span.close),
-                Spacing::Alone,
-            )
+            (Token::new(token::CloseDelim(self.frame.delim), self.frame.span.close), Spacing::Alone)
         } else if let Some(frame) = self.stack.pop() {
             self.frame = frame;
             self.next(desugar_doc_comments)
Diff in /checkout/compiler/rustc_parse/src/parser/mod.rs at line 318:
             [
                 TokenTree::token(token::Ident(sym::doc, false), span),
                 TokenTree::token(token::Eq, span),
-                TokenTree::token(
-                    TokenKind::lit(token::StrRaw(num_of_hashes), data, None),
-                ),
-                ),
+                TokenTree::token(TokenKind::lit(token::StrRaw(num_of_hashes), data, None), span),
             .iter()
             .cloned()
Diff in /checkout/compiler/rustc_parse/src/parser/mod.rs at line 335:
                 token::NoDelim,
                 token::NoDelim,
                 false,
                 if attr_style == AttrStyle::Inner {
-                    [
-                        TokenTree::token(token::Pound, span),
-                        TokenTree::token(token::Not, span),
-                        body,
-                    .iter()
-                    .cloned()
-                    .cloned()
-                    .collect::<TokenStream>()
+                    [TokenTree::token(token::Pound, span), TokenTree::token(token::Not, span), body]
+                        .iter()
+                        .cloned()
+                        .collect::<TokenStream>()
                 } else {
                     [TokenTree::token(token::Pound, span), body]
                         .iter()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/mod.rs" "/checkout/compiler/rustc_parse/src/parser/nonterminal.rs" "/checkout/compiler/rustc_parse/src/parser/item.rs" "/checkout/compiler/rustc_parse/src/parser/path.rs" "/checkout/compiler/rustc_parse/src/parser/attr.rs" "/checkout/compiler/rustc_traits/src/implied_outlives_bounds.rs" "/checkout/compiler/rustc_parse/src/parser/pat.rs" "/checkout/compiler/rustc_parse/src/parser/attr_wrapper.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
