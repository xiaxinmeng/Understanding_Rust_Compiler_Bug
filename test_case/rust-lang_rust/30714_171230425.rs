 diff
-            let iv = match eval_const_expr_partial(cx.tcx(), &index, ExprTypeChecked, None) {
-                Ok(ConstVal::Int(i)) => i as u64,
-                Ok(ConstVal::Uint(u)) => u,
-                _ => cx.sess().span_bug(index.span,
-                                        "index is not an integer-constant expression")
+            let iv = try!(const_expr(cx, &**index, param_substs, fn_args, trueconst)).0;
+            let iv = if let Some(iv) = const_to_opt_uint(iv) {
+                iv
+            } else {
+                cx.sess().span_bug(index.span, "index is not an integer-constant expression");
