plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9de747483029ec3b0c050f7af0dc56765035ff98 and 6b2ca5cc98c3b345efaa5cbc059355892aee61a3
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/macros.rs:737:
 
     fn add_meta_variable(&mut self, iter: &mut Cursor) -> Option<()> {
         match iter.next() {
-            Some(TokenTree::Token(Token {
-                kind: TokenKind::Ident(name, _),
-                ..
-            }, _)) => {
+            Some(TokenTree::Token(
+                Token {
+                    kind: TokenKind::Ident(name, _),
+                },
+                _,
+            )) => {
+            )) => {
                 self.result.push(ParsedMacroArg {
                     kind: MacroArgKind::MetaVariable(name, self.buf.clone()),

Mismatch at src/macros.rs:777:
             }
 
 
             match tok {
-                TokenTree::Token(Token {
-                    kind: TokenKind::BinOp(BinOpToken::Plus),
-                }, _)
-                }, _)
-                | TokenTree::Token(Token {
-                    kind: TokenKind::Question,
-                }, _)
-                }, _)
-                | TokenTree::Token(Token {
-                    kind: TokenKind::BinOp(BinOpToken::Star),
-                }, _) => {
+                TokenTree::Token(
+                    Token {
+                    Token {
+                        kind: TokenKind::BinOp(BinOpToken::Plus),
+                    },
+                    _,
+                )
+                | TokenTree::Token(
---
+                    _,
+                )
+                | TokenTree::Token(
+                    Token {
+                        kind: TokenKind::BinOp(BinOpToken::Star),
+                    },
+                    _,
+                ) => {
                     break;
                     break;
                 }
                 TokenTree::Token(ref t, _) => {
Mismatch at src/macros.rs:859:
 
 
         while let Some(tok) = iter.next() {
             match tok {
-                TokenTree::Token(Token {
-                    kind: TokenKind::Dollar,
-                }, _) => {
+                TokenTree::Token(
+                    Token {
+                        kind: TokenKind::Dollar,
+                        kind: TokenKind::Dollar,
+                        span,
+                    },
+                    _,
+                ) => {
                     // We always want to add a separator before meta variables.
                     if !self.buf.is_empty() {
                         self.add_separator();
Mismatch at src/macros.rs:875:
                         span,
                     };
                 }
                 }
-                TokenTree::Token(Token {
-                    kind: TokenKind::Colon,
-                    ..
-                }, _) if self.is_meta_var => {
+                    Token {
+                        kind: TokenKind::Colon,
+                        ..
+                    },
+                    },
+                    _,
+                ) if self.is_meta_var => {
                     self.add_meta_variable(&mut iter)?;
                 }
                 TokenTree::Token(ref t, _) => self.update_buffer(t),
Mismatch at src/macros.rs:1125:
         };
         };
         let args = TokenStream::new(vec![tok]);
         match self.toks.next()? {
-            TokenTree::Token(Token {
-                kind: TokenKind::FatArrow,
-            }, _) => {}
+            TokenTree::Token(
+                Token {
+                    kind: TokenKind::FatArrow,
+                    kind: TokenKind::FatArrow,
+                    ..
+                },
+                _,
+            ) => {}
             _ => return None,
         }
         let (mut hi, body, whole_body) = match self.toks.next()? {
Mismatch at src/macros.rs:1147:
                 )
             }
         };
         };
-        if let Some(TokenTree::Token(Token {
-            kind: TokenKind::Semi,
-            span,
-        }, _)) = self.toks.look_ahead(0)
+        if let Some(TokenTree::Token(
+                kind: TokenKind::Semi,
+                span,
+            },
+            _,
+            _,
+        )) = self.toks.look_ahead(0)
         {
             hi = span.hi();
             self.toks.next();
test test::system_tests ... ok
test test::idempotence_tests ... ok

failures:
failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
