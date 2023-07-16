
#0  0x00007ffff4d6485d in rustc_trans::trans::foreign::trans_rust_fn_with_foreign_abi::build_rust_fn (ccx=0x7fffec3386e8, decl=0x7fffe4031290, body=0x7fffe4028110, param_substs=0x7fffe40bebc0, attrs=&[syntax::codemap::Spanned<syntax::ast::Attribute_>](len: 0), id=11, hash=...)
    at src/librustc_trans/trans/foreign.rs:632
#1  0x00007ffff4b600ec in rustc_trans::trans::foreign::trans_rust_fn_with_foreign_abi (ccx=0x7fffec3386e8, decl=0x7fffe4031290, body=0x7fffe4028110, attrs=&[syntax::codemap::Spanned<syntax::ast::Attribute_>](len: 0), llwrapfn=0x7fffe410ae58, param_substs=0x7fffe40bebc0, id=11, hash=...)
   at src/librustc_trans/trans/foreign.rs:607
#2  0x00007ffff4b5d2fc in rustc_trans::trans::base::trans_item (ccx=0x7fffec33b018, item=0x7fffe4043710)
  at src/librustc_trans/trans/base.rs:2511
#3  0x00007ffff4bbc55a in fnfn () at src/librustc_trans/trans/base.rs:3379
#4  0x00007ffff4bbc469 in rustc_trans::dep_graph::DepGraph::with_task<closure,()> (self=0x7fffec33e8d8, key=TransCrateItem = {...}, op=closure = {...})
  at src/librustc/dep_graph/mod.rs:162
#5  0x00007ffff4bbc3e1 in rustc_trans::trans::base::TransItemsWithinModVisitor<'a, 'tcx>.Visitor<'v>::visit_item (self=0x7fffec33b000, i=0x7fffe4043710)
  at src/librustc_trans/trans/base.rs:3369
#6  0x00007ffff4ba5490 in rustc_trans::trans::base::TransItemsWithinModVisitor<'a, 'tcx>.Visitor<'v>::visit_nested_item (self=0x7fffec33b000, item_id=ItemId = {...})
  at src/librustc_trans/trans/base.rs:3355
