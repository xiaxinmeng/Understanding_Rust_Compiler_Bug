
...
DEBUG:rustc::ty::maps: ty::queries::extern_crate::try_get_with(key=DefId { krate: CrateNum(1), node: DefIndex(0) => std/8dd0374 }, span=src/test/ui/type-check/unknown_type_for_closure.rs:1:1: 1:1)
DEBUG:rustc::traits::error_reporting: maybe_report_ambiguity(predicate=Binder(TraitPredicate(<_ as std::marker::Sized>)), obligation=Obligation(predicate=Binder(TraitPredicate(<_ as std::marker::Sized>)),depth=0))
DEBUG:rustc::infer: is_tainted_by_errors(err_count=0, err_count_on_creation=0, tainted_by_errors_flag=false)
DEBUG:rustc::ty::fold: HasTypeFlagsVisitor: t=_ t.flags=c04 self.flags=80
DEBUG:rustc::ty::fold: HasTypeFlagsVisitor: t=_ t.flags=c04 self.flags=c
DEBUG:rustc::ty::fold: HasTypeFlagsVisitor: t=_ t.flags=c04 self.flags=4
DEBUG:rustc::infer::error_reporting::need_type_info: self.tcx.hir.find >> Some(NodeExpr(expr(13: { })))
error[E0282]: type annotations needed
  --> src/test/ui/type-check/unknown_type_for_closure.rs:12:17
   |
12 |     let x = |_| {    };
   |                 ^ cannot infer type for `_`

DEBUG:rustc_typeck::check::regionck: regionck_fn(id=4)
...
