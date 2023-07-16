diff
diff --git a/src/librustc/middle/const_val.rs b/src/librustc/middle/const_val.rs
index 3bbaf5c929..7e98f25894 100644
--- a/src/librustc/middle/const_val.rs
+++ b/src/librustc/middle/const_val.rs
@@ -205,6 +205,7 @@ impl<'a, 'gcx, 'tcx> ConstEvalErr<'tcx> {
             diag.span_note(primary_span,
                         &format!("for {} here", primary_kind));
         }
+        panic!("{:?}", self.description());
     }
 
     pub fn report(&self,
