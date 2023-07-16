plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:560:14
     |
560  |         self.expr_call(overall_span, constructor, std::slice::from_ref(expr))
     |              ^^^^^^^^^ ------------ an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1838:8
     |
1838 |     fn expr_call(
1838 |     fn expr_call(
     |        ^^^^^^^^^
1839 |         &mut self,
1840 |         hir_id: hir::HirId,
1841 |         span: Span,
     |         ----------
     |         ----------
1842 |         e: &'hir hir::Expr<'hir>,
     |         ------------------------
1843 |         args: &'hir [hir::Expr<'hir>],
help: provide the argument
     |
     |
560  |         self.expr_call(/* rustc_hir::HirId */, overall_span, constructor, std::slice::from_ref(expr))

error[E0061]: this function takes 5 arguments but 4 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:777:38
     |
     |
777  |             let new_unchecked = self.expr_call_lang_item_fn_mut(
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^
778  |                 span,
     |                 ---- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1848:8
     |
1848 |     fn expr_call_lang_item_fn_mut(
1848 |     fn expr_call_lang_item_fn_mut(
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^
1849 |         &mut self,
1850 |         fn_mut_hir_id: hir::HirId,
1851 |         span: Span,
     |         ----------
1852 |         lang_item: hir::LangItem,
     |         ------------------------
     |         ------------------------
1853 |         args: &'hir [hir::Expr<'hir>],
     |         -----------------------------
1854 |         hir_id: Option<hir::HirId>,
help: provide the argument
     |
     |
777  |             let new_unchecked = self.expr_call_lang_item_fn_mut(/* rustc_hir::HirId */, span, hir::LangItem::PinNewUnchecked, arena_vec![self; ref_mut_awaitee], Some(expr_hir_id));

error[E0061]: this function takes 5 arguments but 4 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:783:36
     |
     |
783  |             let get_context = self.expr_call_lang_item_fn_mut(
     |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^
784  |                 gen_future_span,
     |                 --------------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1848:8
     |
1848 |     fn expr_call_lang_item_fn_mut(
1848 |     fn expr_call_lang_item_fn_mut(
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^
1849 |         &mut self,
1850 |         fn_mut_hir_id: hir::HirId,
1851 |         span: Span,
     |         ----------
1852 |         lang_item: hir::LangItem,
     |         ------------------------
     |         ------------------------
1853 |         args: &'hir [hir::Expr<'hir>],
     |         -----------------------------
1854 |         hir_id: Option<hir::HirId>,
help: provide the argument
     |
     |
783  |             let get_context = self.expr_call_lang_item_fn_mut(/* rustc_hir::HirId */, gen_future_span, hir::LangItem::GetContext, arena_vec![self; task_context], Some(expr_hir_id));

error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:795:35
     |
     |
795  |             self.arena.alloc(self.expr_unsafe(call))
     |                                   ^^^^^^^^^^^ ---- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1927:8
     |
1927 |     fn expr_unsafe(&mut self,
1927 |     fn expr_unsafe(&mut self,
     |        ^^^^^^^^^^^
1928 |         hir_id: hir::HirId,
     |         ------------------
1929 |         expr: &'hir hir::Expr<'hir>) -> hir::Expr<'hir> {
help: provide the argument
     |
     |
795  |             self.arena.alloc(self.expr_unsafe(/* rustc_hir::HirId */, call))

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:805:31
     |
     |
805  |             let x_expr = self.expr_ident(gen_future_span, x_ident, x_pat_hid);
     |                               ^^^^^^^^^^ --------------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1885:19
     |
1885 |     pub(super) fn expr_ident(
1885 |     pub(super) fn expr_ident(
     |                   ^^^^^^^^^^
1886 |         &mut self,
1887 |         hir_id: hir::HirId,
1888 |         sp: Span,
     |         --------
1889 |         ident: Ident,
     |         ------------
     |         ------------
1890 |         binding: hir::HirId,
help: provide the argument
     |
     |
805  |             let x_expr = self.expr_ident(/* rustc_hir::HirId */, gen_future_span, x_ident, x_pat_hid);

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:818:18
     |
     |
818  |             self.arm(ready_pat, break_x)
     |                  ^^^ --------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2010:8
     |
     |
2010 |     fn arm(&mut self, hir_id: hir::HirId, pat: &'hir hir::Pat<'hir>, expr: &'hir hir::Expr<'hir>) -> hir::Arm<'hir> {
help: provide the argument
     |
     |
818  |             self.arm(/* rustc_hir::HirId */, ready_pat, break_x)

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:830:18
     |
     |
830  |             self.arm(pending_pat, empty_block)
     |                  ^^^ ----------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2010:8
     |
     |
2010 |     fn arm(&mut self, hir_id: hir::HirId, pat: &'hir hir::Pat<'hir>, expr: &'hir hir::Expr<'hir>) -> hir::Arm<'hir> {
help: provide the argument
     |
     |
830  |             self.arm(/* rustc_hir::HirId */, pending_pat, empty_block)

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:856:32
     |
     |
856  |                 let lhs = self.expr_ident(span, task_context_ident, task_context_hid);
     |                                ^^^^^^^^^^ ---- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1885:19
     |
1885 |     pub(super) fn expr_ident(
1885 |     pub(super) fn expr_ident(
     |                   ^^^^^^^^^^
1886 |         &mut self,
1887 |         hir_id: hir::HirId,
1888 |         sp: Span,
     |         --------
1889 |         ident: Ident,
     |         ------------
     |         ------------
1890 |         binding: hir::HirId,
help: provide the argument
     |
     |
856  |                 let lhs = self.expr_ident(/* rustc_hir::HirId */, span, task_context_ident, task_context_hid);

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:885:32
     |
     |
885  |         let awaitee_arm = self.arm(awaitee_pat, loop_expr);
     |                                ^^^ ----------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2010:8
     |
     |
2010 |     fn arm(&mut self, hir_id: hir::HirId, pat: &'hir hir::Pat<'hir>, expr: &'hir hir::Expr<'hir>) -> hir::Arm<'hir> {
help: provide the argument
     |
     |
885  |         let awaitee_arm = self.arm(/* rustc_hir::HirId */, awaitee_pat, loop_expr);

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1301:26
     |
     |
1301 |         let ident = self.expr_ident(lhs.span, ident, binding);
     |                          ^^^^^^^^^^ -------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1885:19
     |
1885 |     pub(super) fn expr_ident(
1885 |     pub(super) fn expr_ident(
     |                   ^^^^^^^^^^
1886 |         &mut self,
1887 |         hir_id: hir::HirId,
1888 |         sp: Span,
     |         --------
1889 |         ident: Ident,
     |         ------------
     |         ------------
1890 |         binding: hir::HirId,
help: provide the argument
     |
     |
1301 |         let ident = self.expr_ident(/* rustc_hir::HirId */, lhs.span, ident, binding);

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1380:26
     |
     |
1380 |                     self.expr_field(ident, expr, e.span)
     |                          ^^^^^^^^^^ ----- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1994:8
     |
1994 |     fn expr_field(
1994 |     fn expr_field(
     |        ^^^^^^^^^^
1995 |         &mut self,
1996 |         hir_id: hir::HirId,
1997 |         ident: Ident,
     |         ------------
     |         ------------
1998 |         expr: &'hir hir::Expr<'hir>,
1999 |         span: Span,
     |         ----------
help: provide the argument
     |
     |
1380 |                     self.expr_field(/* rustc_hir::HirId */, ident, expr, e.span)

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1526:18
     |
     |
1526 |             self.arm(pat, break_expr)
     |                  ^^^ --- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2010:8
     |
     |
2010 |     fn arm(&mut self, hir_id: hir::HirId, pat: &'hir hir::Pat<'hir>, expr: &'hir hir::Expr<'hir>) -> hir::Arm<'hir> {
help: provide the argument
     |
     |
1526 |             self.arm(/* rustc_hir::HirId */, pat, break_expr)

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1537:18
     |
     |
1537 |             self.arm(some_pat, body_expr)
     |                  ^^^ -------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2010:8
     |
     |
2010 |     fn arm(&mut self, hir_id: hir::HirId, pat: &'hir hir::Pat<'hir>, expr: &'hir hir::Expr<'hir>) -> hir::Arm<'hir> {
help: provide the argument
     |
     |
1537 |             self.arm(/* rustc_hir::HirId */, some_pat, body_expr)

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1547:29
     |
     |
1547 |             let iter = self.expr_ident(head_span, iter, iter_pat_nid);
     |                             ^^^^^^^^^^ --------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1885:19
     |
1885 |     pub(super) fn expr_ident(
1885 |     pub(super) fn expr_ident(
     |                   ^^^^^^^^^^
1886 |         &mut self,
1887 |         hir_id: hir::HirId,
1888 |         sp: Span,
     |         --------
1889 |         ident: Ident,
     |         ------------
     |         ------------
1890 |         binding: hir::HirId,
help: provide the argument
     |
     |
1547 |             let iter = self.expr_ident(/* rustc_hir::HirId */, head_span, iter, iter_pat_nid);

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1573:29
     |
     |
1573 |         let iter_arm = self.arm(iter_pat, loop_expr);
     |                             ^^^ -------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2010:8
     |
     |
2010 |     fn arm(&mut self, hir_id: hir::HirId, pat: &'hir hir::Pat<'hir>, expr: &'hir hir::Expr<'hir>) -> hir::Arm<'hir> {
help: provide the argument
     |
     |
1573 |         let iter_arm = self.arm(/* rustc_hir::HirId */, iter_pat, loop_expr);

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1665:18
     |
     |
1665 |             self.arm(continue_pat, val_expr)
     |                  ^^^ ------------ an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2010:8
     |
     |
2010 |     fn arm(&mut self, hir_id: hir::HirId, pat: &'hir hir::Pat<'hir>, expr: &'hir hir::Expr<'hir>) -> hir::Arm<'hir> {
help: provide the argument
     |
     |
1665 |             self.arm(/* rustc_hir::HirId */, continue_pat, val_expr)

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1674:38
     |
     |
1674 |             let residual_expr = self.expr_ident_mut(try_span, residual_ident, residual_local_nid);
     |                                      ^^^^^^^^^^^^^^ -------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1895:19
     |
1895 |     pub(super) fn expr_ident_mut(
1895 |     pub(super) fn expr_ident_mut(
     |                   ^^^^^^^^^^^^^^
1896 |         &mut self,
1897 |         hir_id: hir::HirId,
1898 |         sp: Span,
     |         --------
1899 |         ident: Ident,
     |         ------------
     |         ------------
1900 |         binding: hir::HirId,
help: provide the argument
     |
     |
1674 |             let residual_expr = self.expr_ident_mut(/* rustc_hir::HirId */, try_span, residual_ident, residual_local_nid);

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1700:18
     |
     |
1700 |             self.arm(break_pat, ret_expr)
     |                  ^^^ --------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2010:8
     |
     |
2010 |     fn arm(&mut self, hir_id: hir::HirId, pat: &'hir hir::Pat<'hir>, expr: &'hir hir::Expr<'hir>) -> hir::Arm<'hir> {
help: provide the argument
     |
     |
1700 |             self.arm(/* rustc_hir::HirId */, break_pat, ret_expr)

error[E0061]: this function takes 5 arguments but 4 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1868:31
     |
     |
1868 |         self.arena.alloc(self.expr_call_lang_item_fn_mut(span, lang_item, args, hir_id))
     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ ---- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1848:8
     |
1848 |     fn expr_call_lang_item_fn_mut(
1848 |     fn expr_call_lang_item_fn_mut(
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^
1849 |         &mut self,
1850 |         fn_mut_hir_id: hir::HirId,
1851 |         span: Span,
     |         ----------
1852 |         lang_item: hir::LangItem,
     |         ------------------------
     |         ------------------------
1853 |         args: &'hir [hir::Expr<'hir>],
     |         -----------------------------
1854 |         hir_id: Option<hir::HirId>,
help: provide the argument
     |
     |
1868 |         self.arena.alloc(self.expr_call_lang_item_fn_mut(/* rustc_hir::HirId */, span, lang_item, args, hir_id))


error[E0599]: no method named `expr_ident_with_attrs_hirid` found for mutable reference `&mut LoweringContext<'_, 'hir>` in the current scope
     |
     |
1902 |         self.expr_ident_with_attrs_hirid(hir_id, sp, ident, binding, AttrVec::new())
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `expr_ident_with_attrs_with_hirid`
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/item.rs:1192:42
     |
     |
1192 |                     let move_expr = this.expr_ident(desugared_span, ident, new_parameter_id);
     |                                          ^^^^^^^^^^ -------------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1885:19
     |
1885 |     pub(super) fn expr_ident(
1885 |     pub(super) fn expr_ident(
     |                   ^^^^^^^^^^
1886 |         &mut self,
1887 |         hir_id: hir::HirId,
1888 |         sp: Span,
     |         --------
1889 |         ident: Ident,
     |         ------------
     |         ------------
1890 |         binding: hir::HirId,
help: provide the argument
     |
     |
1192 |                     let move_expr = this.expr_ident(/* rustc_hir::HirId */, desugared_span, ident, new_parameter_id);

Some errors have detailed explanations: E0061, E0599.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_ast_lowering` due to 21 previous errors
