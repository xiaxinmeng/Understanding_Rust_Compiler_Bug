 rust
struct RawPtrDerivingVisitor<'a> {
     cx: &'a mut Context
}
impl<'a> Visitor<()> for RawPtrDerivingVisitor<'a> {
     fn visit_ty(&mut self, ty: &ast::Ty, _: ()) {
           /* check `ty` is a TyPtr, if so, span_lint on this type */

           // recurse, to walk the interiors of other types
           visit::walk_ty(self, ty, ());
     }
     // explicit override to a no-op these to reduce code bloat
     fn visit_expr(&mut self, e: &ast::Expr, _: ()) {}
     fn visit_block(&mut self, e: &ast::Block, _: ()) {}
}

fn check_raw_ptr_deriving(cx: &mut Context, item: &Item) {
     // ... do the attribute check ...
     match item {
          ItemStruct(..) | ItemEnum(..) => {
               let mut visitor = RawPtrDerivingVisitor { cx: cx };
               visit::walk_item(&mut visitor, item, ());
          }
     }
}
