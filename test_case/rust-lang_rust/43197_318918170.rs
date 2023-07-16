patch
diff --git a/src/librustc_trans/mir/constant.rs b/src/librustc_trans/mir/constan
t.rs
index 98e774a298..e80c13cc9b 100644
--- a/src/librustc_trans/mir/constant.rs
+++ b/src/librustc_trans/mir/constant.rs
@@ -725,6 +725,7 @@ impl<'a, 'tcx> MirConstContext<'a, 'tcx> {
             }
 
             mir::Rvalue::Ref(_, bk, ref lvalue) => {
+                eprintln!("!!!!! lvalue={:?} bk={:?} span={:?}",lvalue, bk, spa
n);
                 let tr_lvalue = self.const_lvalue(lvalue, span)?;
 
                 let ty = tr_lvalue.ty;
