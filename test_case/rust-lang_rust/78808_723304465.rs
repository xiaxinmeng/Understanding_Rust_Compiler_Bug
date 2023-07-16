diff
diff --git a/compiler/rustc_builtin_macros/src/deriving/cmp/partial_eq.rs b/compiler/rustc_builtin_macros/src/deriving/cmp/partial_eq.rs
index 8e9f15743cc..faaba4bf4ef 100644
--- a/compiler/rustc_builtin_macros/src/deriving/cmp/partial_eq.rs
+++ b/compiler/rustc_builtin_macros/src/deriving/cmp/partial_eq.rs
@@ -5,7 +5,7 @@ use crate::deriving::{path_local, path_std};
 use rustc_ast::ptr::P;
 use rustc_ast::{BinOpKind, Expr, MetaItem};
 use rustc_expand::base::{Annotatable, ExtCtxt};
-use rustc_span::symbol::sym;
+use rustc_span::symbol::{sym, Symbol};
 use rustc_span::Span;
 
 pub fn expand_deriving_partial_eq(
@@ -21,7 +21,7 @@ pub fn expand_deriving_partial_eq(
         cx: &mut ExtCtxt<'_>,
         span: Span,
         substr: &Substructure<'_>,
-        op: BinOpKind,
+        op: Symbol,
         combiner: BinOpKind,
         base: bool,
     ) -> P<Expr> {
@@ -31,7 +31,15 @@ pub fn expand_deriving_partial_eq(
                 _ => cx.span_bug(span, "not exactly 2 arguments in `derive(PartialEq)`"),
             };
 
-            cx.expr_binary(span, op, self_f, other_f.clone())
+            // Instead of `*at_field OP *at_other_field`, do:
+            // `::core::cmp::PartialEq::op(&*at_field, &*at_other_field)`
+            // so as to avoid a bug when `*at_field` is of type
+            // `impl !Copy + Deref<Target = impl !Sized + PartialEq>` (_e.g._,
+            // `Box<dyn Trait>` where `dyn Trait : PartialEq`. See #78808).
+            let self_f_ref = cx.expr_addr_of(span, self_f);
+            let other_f_ref = cx.expr_addr_of(span, other_f.clone());
+            let partial_eq_op = cx.std_path(&[sym::cmp, sym::PartialEq, op]);
+            cx.expr_call_global(span, partial_eq_op, vec![self_f_ref, other_f_ref])
         };
 
         cs_fold1(
@@ -57,10 +65,10 @@ pub fn expand_deriving_partial_eq(
     }
 
     fn cs_eq(cx: &mut ExtCtxt<'_>, span: Span, substr: &Substructure<'_>) -> P<Expr> {
-        cs_op(cx, span, substr, BinOpKind::Eq, BinOpKind::And, true)
+        cs_op(cx, span, substr, sym::eq, BinOpKind::And, true)
     }
     fn cs_ne(cx: &mut ExtCtxt<'_>, span: Span, substr: &Substructure<'_>) -> P<Expr> {
-        cs_op(cx, span, substr, BinOpKind::Ne, BinOpKind::Or, false)
+        cs_op(cx, span, substr, sym::ne, BinOpKind::Or, false)
     }
 
     macro_rules! md {
