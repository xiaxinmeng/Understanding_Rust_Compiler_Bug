diff
Mismatch at src/parse/parser.rs:113:
         let result = catch_unwind(AssertUnwindSafe(|| {
             let mut parser = new_parser_from_file(sess.inner(), path, Some(span));
             match parser.parse_mod(&TokenKind::Eof) {
-                Ok((a,
+                    a,

                     ast::ModSpans {
                         inner_span,
                         inner_span,

