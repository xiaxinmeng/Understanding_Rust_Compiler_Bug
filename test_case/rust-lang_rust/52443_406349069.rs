
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: >> typechecking: expr=expr(15: [(); ]) expected=NoExpectation
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::collect: explicit_predicates_of(def_id=DefId(0/1:9 ~ issue_52443[317d]::main[0]::{{constant}}[0]))
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: >> typechecking: expr=expr(14: &{ loop  { continue  } }) expected=ExpectHasType(usize)
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: >> typechecking: expr=expr(13: { loop  { continue  } }) expected=NoExpectation
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: >> typechecking: expr=expr(12: loop  { continue  }) expected=NoExpectation
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: >> typechecking: expr=expr(11: continue) expected=ExpectHasType(())
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: write_ty(HirId { owner: DefIndex(0:3), local_id: ItemLocalId(3) }, [type error]) in fcx 0x7ffe425f2ea8
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: type of expr continue (id=11) is...
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: ... [type error], expected is ExpectHasType(())
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: write_ty(HirId { owner: DefIndex(0:3), local_id: ItemLocalId(4) }, [type error]) in fcx 0x7ffe425f2ea8
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: write_ty(HirId { owner: DefIndex(0:3), local_id: ItemLocalId(5) }, !) in fcx 0x7ffe425f2ea8
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: type of expr loop  { continue  } (id=12) is...
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: ... !, expected is NoExpectation
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: resolve_type_vars_with_obligations(ty=!)
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: resolve_type_vars_with_obligations: ty=!
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check::coercion: coercion::try(expr(12: loop  { continue  }): ! -> _)
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check::coercion: Coerce.tys(! => _)
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: register_predicate(Obligation(predicate=Binder(SubtypePredicate { a_is_expected: false, a: _, b: _ }),depth=0))
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: apply_adjustments(expr=expr(12: loop  { continue  }), adj=[NeverToAny -> _])
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: write_ty(HirId { owner: DefIndex(0:3), local_id: ItemLocalId(6) }, [type error]) in fcx 0x7ffe425f2ea8
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: write_ty(HirId { owner: DefIndex(0:3), local_id: ItemLocalId(7) }, [type error]) in fcx 0x7ffe425f2ea8
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: type of expr { loop  { continue  } } (id=13) is...
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: ... [type error], expected is NoExpectation
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: warn_if_unreachable: id=NodeId(14) span=/home/oliver/Projects/rust/rust4/src/test/ui/const-eval/issue-52443.rs:12:10: 12:33 kind=expression
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: write_ty(HirId { owner: DefIndex(0:3), local_id: ItemLocalId(8) }, [type error]) in fcx 0x7ffe425f2ea8
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: type of expr &{ loop  { continue  } } (id=14) is...
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: ... [type error], expected is ExpectHasType(usize)
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: resolve_type_vars_with_obligations(ty=usize)
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: resolve_type_vars_with_obligations: ty=usize
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: resolve_type_vars_with_obligations(ty=[type error])
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: resolve_type_vars_with_obligations: ty=[type error]
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check::coercion: coercion::try(expr(14: &{ loop  { continue  } }): [type error] -> usize)
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check::coercion: Coerce.tys([type error] => usize)
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: apply_adjustments(expr=expr(14: &{ loop  { continue  } }), adj=[])
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: default_type_parameters: defaulting `_` to `[type error]`
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: default_type_parameters: defaulting `_` to `[type error]`
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: default_type_parameters: defaulting `_` to `[type error]`
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check: select_all_obligations_or_error
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check::regionck: regionck::visit_expr(e=expr(14: &{ loop  { continue  } }), repeating_scope=14)
DEBUG 2018-07-19T16:49:23Z: rustc_typeck::check::regionck: constrain_adjustments(expr=expr(14: &{ loop  { continue  } }))
error: internal compiler error: cat_expr Err ()
  --> /home/oliver/Projects/rust/rust4/src/test/ui/const-eval/issue-52443.rs:12:10
   |
LL |     [(); & { loop { continue } } ];
   |          ^^^^^^^^^^^^^^^^^^^^^^^
