
WARN:rustc_typeck::check::regionck: regionck::visit_pat(pat=pat(87: _g))
WARN:rustc::ty::util: dtorck_constraint_for_ty(1.rs:26:13: 26:15, Group<'_, std::iter::Map<std::slice::Iter<'_, i32>, [closure@1.rs:25:39: 25:44]>>, 0, Group<'_, std::iter::Map<std::slice::Iter<'_, i32>, [closure@1.rs:25:39: 25:44]>>)
WARN:rustc::ty::util: dtorck_constraint_for_ty(Group<'_, std::iter::Map<std::slice::Iter<'_, i32>, [closure@1.rs:25:39: 25:44]>>) = Ok(DtorckConstraint { outlives: [], dtorck_types: [<std::iter::Map<std::slice::Iter<'_, i32>, [closure@1.rs:25:39: 25:44]> as std::iter::Iterator>::Item] })
WARN:rustc::ty::util: dtorck_constraint_for_ty(1.rs:26:13: 26:15, Group<'_, std::iter::Map<std::slice::Iter<'_, i32>, [closure@1.rs:25:39: 25:44]>>, 1, _)
WARN:rustc::ty::util: ty.sty = TyInfer(_#45t)
