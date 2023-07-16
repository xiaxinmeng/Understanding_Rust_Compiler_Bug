
###snip###
DEBUG:rustc::middle::expr_use_visitor: consume_expr(expr=expr(6: 42))
DEBUG:rustc::middle::mem_categorization: cat_expr: id=6 expr=expr(6: 42)
DEBUG:rustc::middle::mem_categorization: cat_rvalue ret {cat_rvalue(ReStatic) id:6 m:McDeclared ty:i32}
DEBUG:rustc::middle::mem_categorization: cat_rvalue_node ret {cat_rvalue(ReStatic) id:6 m:McDeclared ty:i32}
DEBUG:rustc::middle::expr_use_visitor: delegate_consume(consume_id=6, cmt={cat_rvalue(ReStatic) id:6 m:McDeclared ty:i32})
DEBUG:rustc::middle::check_const: CheckCrateVisitor::consume: cur.cat=cat_rvalue(ReStatic), mode=Static
DEBUG:rustc::middle::expr_use_visitor: walk_expr(expr=expr(6: 42))
DEBUG:rustc::middle::check_const: visit_item(item=item(static B::B (id=7)))
###snip###
DEBUG:rustc::middle::expr_use_visitor: consume_expr(expr=expr(9: *&A))
DEBUG:rustc::middle::mem_categorization: cat_expr: id=9 expr=expr(9: *&A)
DEBUG:rustc::middle::mem_categorization: cat_expr: id=10 expr=expr(10: &A)
DEBUG:rustc::middle::region: temporary_scope(10) = None
DEBUG:rustc::middle::mem_categorization: cat_rvalue ret {cat_rvalue(ReStatic) id:10 m:McDeclared ty:&i32}
DEBUG:rustc::middle::mem_categorization: cat_rvalue_node ret {cat_rvalue(ReStatic) id:10 m:McDeclared ty:&i32}
DEBUG:rustc::middle::mem_categorization: cat_deref: method_call=MethodCall { expr_id: 9, autoderef: 0 } method_ty=None
DEBUG:rustc::middle::mem_categorization: MutabilityCategory::from_borrow_kind(ImmBorrow) => McImmutable
DEBUG:rustc::middle::mem_categorization: MutabilityCategory::from_pointer_kind(McDeclared, BorrowedPtr(ImmBorrow, ReScope(Misc(9)))) => McImmutable
DEBUG:rustc::middle::mem_categorization: cat_deref_common ret {cat_rvalue(ReStatic)-&ReScope(Misc(9))0-> id:9 m:McImmutable ty:i32}
DEBUG:rustc::middle::mem_categorization: cat_deref ret {cat_rvalue(ReStatic)-&ReScope(Misc(9))0-> id:9 m:McImmutable ty:i32}
DEBUG:rustc::middle::expr_use_visitor: delegate_consume(consume_id=9, cmt={cat_rvalue(ReStatic)-&ReScope(Misc(9))0-> id:9 m:McImmutable ty:i32})
DEBUG:rustc::middle::check_const: CheckCrateVisitor::consume: cur.cat=cat_deref(cmt_ { id: 10, span: Span { lo: BytePos(515), hi: BytePos(517), expn_id: ExpnId(4294967295) }, cat: cat_rvalue(ReStatic), mutbl: McDeclared, ty: TyS { sty: ty_rptr(ReScope(Misc(9)), mt { ty: TyS { sty: ty_int(i32), flags: Cell { value: 262144 }, region_depth: 0 }, mutbl: MutImmutable }), flags: Cell { value: 32 }, region_depth: 0 }, note: NoteNone }, 0, BorrowedPtr(ImmBorrow, ReScope(Misc(9)))), mode=Static
DEBUG:rustc::middle::check_const: CheckCrateVisitor::consume: cur.cat=cat_rvalue(ReStatic), mode=Static
DEBUG:rustc::middle::expr_use_visitor: walk_expr(expr=expr(9: *&A))
DEBUG:rustc::middle::expr_use_visitor: walk_expr(expr=expr(10: &A))
DEBUG:rustc::middle::expr_use_visitor: borrow_expr(expr=expr(11: A), r=ReScope(Misc(9)), bk=ImmBorrow)
DEBUG:rustc::middle::mem_categorization: cat_expr: id=11 expr=expr(11: A)
DEBUG:rustc::middle::mem_categorization: cat_def: id=11 expr=i32 def=DefStatic(DefId { krate: 0, node: 4 }, false)
DEBUG:rustc::middle::check_const: CheckCrateVisitor::borrow: cur.cat=cat_static_item, is_interior=false, mode=Static
DEBUG:rustc::middle::expr_use_visitor: walk_expr(expr=expr(11: A))
DEBUG:rustc::middle::check_const: visit_item(item=item(fn main::main (id=12)))
###snip###
