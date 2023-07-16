diff
         let parser_snapshot_before_type = self.clone();
         match self.parse_ty_no_plus() {
             Ok(rhs) => {
-                Ok(mk_expr(self, rhs))
+                let expr = mk_expr(self, rhs);
+                if let ExprKind::Type(..) = expr.kind  {
+                    panic!("PARSED TYPE ASCRIPTION!");
+                }
+                Ok(expr)
             }
