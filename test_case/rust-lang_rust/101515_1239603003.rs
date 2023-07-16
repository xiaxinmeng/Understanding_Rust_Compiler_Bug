diff
-            let mut err = self.struct_span_err(self.token.span, "unexpected `==`");
-            err.span_suggestion_short(self.token.span, "try using `=` instead `==`", "=", appl)
-                .emit();
-            return Err(err);
+            self.sess.emit_err(UseEqInstead { span: self.token.span });
+            return Ok(true);
