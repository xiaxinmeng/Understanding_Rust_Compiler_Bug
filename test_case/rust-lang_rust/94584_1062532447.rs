
             match parser.parse_mod(&TokenKind::Eof) {
-                Ok((a, i, ast::ModSpans { inner_span, inject_use_span: _ })) => Some((a, i, inner_span)),
+                Ok((
+                    a,
+                    i,
+                    ast::ModSpans {
+                        inner_span,
+                        inject_use_span: _,
+                    },
+                )) => Some((a, i, inner_span)),

