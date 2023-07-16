 diff
diff --git a/src/librustc_mir/build/expr/as_rvalue.rs b/src/librustc_mir/build/expr/as_rvalue.rs
index 23ca221..e351215 100644
--- a/src/librustc_mir/build/expr/as_rvalue.rs
+++ b/src/librustc_mir/build/expr/as_rvalue.rs
@@ -44,8 +44,8 @@ impl<'a,'tcx> Builder<'a,'tcx> {
             }
             ExprKind::Repeat { value, count } => {
                 let value_operand = unpack!(block = this.as_operand(block, value));
-                let count_operand = unpack!(block = this.as_operand(block, count));
-                block.and(Rvalue::Repeat(value_operand, count_operand))
+                let count = this.as_constant(count);
+                block.and(Rvalue::Repeat(value_operand, count))
             }
             ExprKind::Borrow { region, borrow_kind, arg } => {
                 let arg_lvalue = unpack!(block = this.as_lvalue(block, arg));
diff --git a/src/librustc_mir/repr.rs b/src/librustc_mir/repr.rs
index 89b1afa..4ab08f9 100644
--- a/src/librustc_mir/repr.rs
+++ b/src/librustc_mir/repr.rs
@@ -546,7 +546,7 @@ pub enum Rvalue<'tcx> {
     Use(Operand<'tcx>),

     // [x; 32]
-    Repeat(Operand<'tcx>, Operand<'tcx>),
+    Repeat(Operand<'tcx>, Constant<'tcx>),

     // &x or &mut x
     Ref(Region, BorrowKind, Lvalue<'tcx>),
diff --git a/src/librustc_mir/visit.rs b/src/librustc_mir/visit.rs
index b4d6075..9f214af 100644
--- a/src/librustc_mir/visit.rs
+++ b/src/librustc_mir/visit.rs
@@ -134,7 +134,7 @@ pub trait Visitor<'tcx> {

             Rvalue::Repeat(ref value, ref len) => {
                 self.visit_operand(value);
-                self.visit_operand(len);
+                self.visit_constant(len);
             }

             Rvalue::Ref(r, bk, ref path) => {
diff --git a/src/librustc_trans/trans/mir/rvalue.rs b/src/librustc_trans/trans/mir/rvalue.rs
index f3515c0..218edc2 100644
--- a/src/librustc_trans/trans/mir/rvalue.rs
+++ b/src/librustc_trans/trans/mir/rvalue.rs
@@ -51,9 +51,9 @@ impl<'bcx, 'tcx> MirContext<'bcx, 'tcx> {

             mir::Rvalue::Repeat(ref elem, ref count) => {
                 let elem = self.trans_operand(bcx, elem);
-                let size = self.trans_operand(bcx, count);
+                let size = self.trans_constant(bcx, count);
                 let base = expr::get_dataptr(bcx, lldest);
-                tvec::iter_vec_raw(bcx, base, elem.ty, size.llval, |b, vref, _| {
+                tvec::iter_vec_raw(bcx, base, elem.ty, size, |b, vref, _| {
                     build::Store(b, elem.llval, vref);
                     b
                 })
