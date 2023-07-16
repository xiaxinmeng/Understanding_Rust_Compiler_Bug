diff
-            Constant(ref constant) => self.eval_const_to_op(constant.literal, layout)?,
+            Constant(ref constant) => {
+                self.eval_const_to_op(self.monomorphize(constant.literal)?, layout)?
+            }
