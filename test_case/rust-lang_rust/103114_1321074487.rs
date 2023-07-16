plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:560:14
     |
560  |         self.expr_call(overall_span, constructor, std::slice::from_ref(expr))
     |              ^^^^^^^^^ ------------ an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1852:8
     |
1852 |     fn expr_call(
1852 |     fn expr_call(
     |        ^^^^^^^^^
1853 |         &mut self,
1854 |         hir_id: hir::HirId,
1855 |         span: Span,
     |         ----------
     |         ----------
1856 |         e: &'hir hir::Expr<'hir>,
     |         ------------------------
1857 |         args: &'hir [hir::Expr<'hir>],
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
    --> compiler/rustc_ast_lowering/src/expr.rs:1862:8
     |
1862 |     fn expr_call_lang_item_fn_mut(
1862 |     fn expr_call_lang_item_fn_mut(
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^
1863 |         &mut self,
1864 |         fn_mut_hir_id: hir::HirId,
1865 |         span: Span,
     |         ----------
1866 |         lang_item: hir::LangItem,
     |         ------------------------
     |         ------------------------
1867 |         args: &'hir [hir::Expr<'hir>],
     |         -----------------------------
1868 |         hir_id: Option<hir::HirId>,
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
    --> compiler/rustc_ast_lowering/src/expr.rs:1862:8
     |
1862 |     fn expr_call_lang_item_fn_mut(
1862 |     fn expr_call_lang_item_fn_mut(
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^
1863 |         &mut self,
1864 |         fn_mut_hir_id: hir::HirId,
1865 |         span: Span,
     |         ----------
1866 |         lang_item: hir::LangItem,
     |         ------------------------
     |         ------------------------
1867 |         args: &'hir [hir::Expr<'hir>],
     |         -----------------------------
1868 |         hir_id: Option<hir::HirId>,
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
    --> compiler/rustc_ast_lowering/src/expr.rs:1941:8
     |
     |
1941 |     fn expr_unsafe(&mut self, hir_id: hir::HirId, expr: &'hir hir::Expr<'hir>) -> hir::Expr<'hir> {
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
    --> compiler/rustc_ast_lowering/src/expr.rs:1899:19
     |
1899 |     pub(super) fn expr_ident(
1899 |     pub(super) fn expr_ident(
     |                   ^^^^^^^^^^
1900 |         &mut self,
1901 |         hir_id: hir::HirId,
1902 |         sp: Span,
     |         --------
1903 |         ident: Ident,
     |         ------------
     |         ------------
1904 |         binding: hir::HirId,
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
    --> compiler/rustc_ast_lowering/src/expr.rs:2016:8
     |
2016 |     fn arm(
2016 |     fn arm(
     |        ^^^
2017 |         &mut self,
2018 |         hir_id: hir::HirId,
     |         ------------------
2019 |         pat: &'hir hir::Pat<'hir>,
     |         -------------------------
2020 |         expr: &'hir hir::Expr<'hir>,
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
    --> compiler/rustc_ast_lowering/src/expr.rs:2016:8
     |
2016 |     fn arm(
2016 |     fn arm(
     |        ^^^
2017 |         &mut self,
2018 |         hir_id: hir::HirId,
     |         ------------------
2019 |         pat: &'hir hir::Pat<'hir>,
     |         -------------------------
2020 |         expr: &'hir hir::Expr<'hir>,
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
    --> compiler/rustc_ast_lowering/src/expr.rs:1899:19
     |
1899 |     pub(super) fn expr_ident(
1899 |     pub(super) fn expr_ident(
     |                   ^^^^^^^^^^
1900 |         &mut self,
1901 |         hir_id: hir::HirId,
1902 |         sp: Span,
     |         --------
1903 |         ident: Ident,
     |         ------------
     |         ------------
1904 |         binding: hir::HirId,
help: provide the argument
     |
     |
856  |                 let lhs = self.expr_ident(/* rustc_hir::HirId */, span, task_context_ident, task_context_hid);

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:890:32
     |
     |
890  |         let awaitee_arm = self.arm(awaitee_pat, loop_expr);
     |                                ^^^ ----------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2016:8
     |
2016 |     fn arm(
2016 |     fn arm(
     |        ^^^
2017 |         &mut self,
2018 |         hir_id: hir::HirId,
     |         ------------------
2019 |         pat: &'hir hir::Pat<'hir>,
     |         -------------------------
2020 |         expr: &'hir hir::Expr<'hir>,
help: provide the argument
     |
     |
890  |         let awaitee_arm = self.arm(/* rustc_hir::HirId */, awaitee_pat, loop_expr);

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1309:26
     |
     |
1309 |         let ident = self.expr_ident(lhs.span, ident, binding);
     |                          ^^^^^^^^^^ -------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1899:19
     |
1899 |     pub(super) fn expr_ident(
1899 |     pub(super) fn expr_ident(
     |                   ^^^^^^^^^^
1900 |         &mut self,
1901 |         hir_id: hir::HirId,
1902 |         sp: Span,
     |         --------
1903 |         ident: Ident,
     |         ------------
     |         ------------
1904 |         binding: hir::HirId,
help: provide the argument
     |
     |
1309 |         let ident = self.expr_ident(/* rustc_hir::HirId */, lhs.span, ident, binding);

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1388:26
     |
     |
1388 |                     self.expr_field(ident, expr, e.span)
     |                          ^^^^^^^^^^ ----- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2006:8
     |
2006 |     fn expr_field(
2006 |     fn expr_field(
     |        ^^^^^^^^^^
2007 |         &mut self,
2008 |         hir_id: hir::HirId,
2009 |         ident: Ident,
     |         ------------
     |         ------------
2010 |         expr: &'hir hir::Expr<'hir>,
2011 |         span: Span,
     |         ----------
help: provide the argument
     |
     |
1388 |                     self.expr_field(/* rustc_hir::HirId */, ident, expr, e.span)

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1534:18
     |
     |
1534 |             self.arm(pat, break_expr)
     |                  ^^^ --- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2016:8
     |
2016 |     fn arm(
2016 |     fn arm(
     |        ^^^
2017 |         &mut self,
2018 |         hir_id: hir::HirId,
     |         ------------------
2019 |         pat: &'hir hir::Pat<'hir>,
     |         -------------------------
2020 |         expr: &'hir hir::Expr<'hir>,
help: provide the argument
     |
     |
1534 |             self.arm(/* rustc_hir::HirId */, pat, break_expr)

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1545:18
     |
     |
1545 |             self.arm(some_pat, body_expr)
     |                  ^^^ -------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2016:8
     |
2016 |     fn arm(
2016 |     fn arm(
     |        ^^^
2017 |         &mut self,
2018 |         hir_id: hir::HirId,
     |         ------------------
2019 |         pat: &'hir hir::Pat<'hir>,
     |         -------------------------
2020 |         expr: &'hir hir::Expr<'hir>,
help: provide the argument
     |
     |
1545 |             self.arm(/* rustc_hir::HirId */, some_pat, body_expr)

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1555:29
     |
     |
1555 |             let iter = self.expr_ident(head_span, iter, iter_pat_nid);
     |                             ^^^^^^^^^^ --------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1899:19
     |
1899 |     pub(super) fn expr_ident(
1899 |     pub(super) fn expr_ident(
     |                   ^^^^^^^^^^
1900 |         &mut self,
1901 |         hir_id: hir::HirId,
1902 |         sp: Span,
     |         --------
1903 |         ident: Ident,
     |         ------------
     |         ------------
1904 |         binding: hir::HirId,
help: provide the argument
     |
     |
1555 |             let iter = self.expr_ident(/* rustc_hir::HirId */, head_span, iter, iter_pat_nid);

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1582:29
     |
     |
1582 |         let iter_arm = self.arm(iter_pat, loop_expr);
     |                             ^^^ -------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2016:8
     |
2016 |     fn arm(
2016 |     fn arm(
     |        ^^^
2017 |         &mut self,
2018 |         hir_id: hir::HirId,
     |         ------------------
2019 |         pat: &'hir hir::Pat<'hir>,
     |         -------------------------
2020 |         expr: &'hir hir::Expr<'hir>,
help: provide the argument
     |
     |
1582 |         let iter_arm = self.arm(/* rustc_hir::HirId */, iter_pat, loop_expr);

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1679:18
     |
     |
1679 |             self.arm(continue_pat, val_expr)
     |                  ^^^ ------------ an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2016:8
     |
2016 |     fn arm(
2016 |     fn arm(
     |        ^^^
2017 |         &mut self,
2018 |         hir_id: hir::HirId,
     |         ------------------
2019 |         pat: &'hir hir::Pat<'hir>,
     |         -------------------------
2020 |         expr: &'hir hir::Expr<'hir>,
help: provide the argument
     |
     |
1679 |             self.arm(/* rustc_hir::HirId */, continue_pat, val_expr)

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1688:38
     |
     |
1688 |             let residual_expr = self.expr_ident_mut(try_span, residual_ident, residual_local_nid);
     |                                      ^^^^^^^^^^^^^^ -------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1909:19
     |
1909 |     pub(super) fn expr_ident_mut(
1909 |     pub(super) fn expr_ident_mut(
     |                   ^^^^^^^^^^^^^^
1910 |         &mut self,
1911 |         hir_id: hir::HirId,
1912 |         sp: Span,
     |         --------
1913 |         ident: Ident,
     |         ------------
     |         ------------
1914 |         binding: hir::HirId,
help: provide the argument
     |
     |
1688 |             let residual_expr = self.expr_ident_mut(/* rustc_hir::HirId */, try_span, residual_ident, residual_local_nid);

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1714:18
     |
     |
1714 |             self.arm(break_pat, ret_expr)
     |                  ^^^ --------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:2016:8
     |
2016 |     fn arm(
2016 |     fn arm(
     |        ^^^
2017 |         &mut self,
2018 |         hir_id: hir::HirId,
     |         ------------------
2019 |         pat: &'hir hir::Pat<'hir>,
     |         -------------------------
2020 |         expr: &'hir hir::Expr<'hir>,
help: provide the argument
     |
     |
1714 |             self.arm(/* rustc_hir::HirId */, break_pat, ret_expr)

error[E0061]: this function takes 5 arguments but 4 arguments were supplied
    --> compiler/rustc_ast_lowering/src/expr.rs:1882:31
     |
     |
1882 |         self.arena.alloc(self.expr_call_lang_item_fn_mut(span, lang_item, args, hir_id))
     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ ---- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1862:8
     |
1862 |     fn expr_call_lang_item_fn_mut(
1862 |     fn expr_call_lang_item_fn_mut(
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^
1863 |         &mut self,
1864 |         fn_mut_hir_id: hir::HirId,
1865 |         span: Span,
     |         ----------
1866 |         lang_item: hir::LangItem,
     |         ------------------------
     |         ------------------------
1867 |         args: &'hir [hir::Expr<'hir>],
     |         -----------------------------
1868 |         hir_id: Option<hir::HirId>,
help: provide the argument
     |
     |
1882 |         self.arena.alloc(self.expr_call_lang_item_fn_mut(/* rustc_hir::HirId */, span, lang_item, args, hir_id))


error[E0599]: no method named `expr_ident_with_attrs_hirid` found for mutable reference `&mut LoweringContext<'_, 'hir>` in the current scope
     |
     |
1916 |         self.expr_ident_with_attrs_hirid(hir_id, sp, ident, binding, AttrVec::new())
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `expr_ident_with_attrs_with_hirid`
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
    --> compiler/rustc_ast_lowering/src/item.rs:1193:42
     |
     |
1193 |                     let move_expr = this.expr_ident(desugared_span, ident, new_parameter_id);
     |                                          ^^^^^^^^^^ -------------- an argument of type `rustc_hir::HirId` is missing
note: associated function defined here
    --> compiler/rustc_ast_lowering/src/expr.rs:1899:19
     |
1899 |     pub(super) fn expr_ident(
1899 |     pub(super) fn expr_ident(
     |                   ^^^^^^^^^^
1900 |         &mut self,
1901 |         hir_id: hir::HirId,
1902 |         sp: Span,
     |         --------
1903 |         ident: Ident,
     |         ------------
     |         ------------
1904 |         binding: hir::HirId,
help: provide the argument
     |
     |
1193 |                     let move_expr = this.expr_ident(/* rustc_hir::HirId */, desugared_span, ident, new_parameter_id);

Some errors have detailed explanations: E0061, E0599.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_ast_lowering` due to 21 previous errors
