
WARN:rustc_typeck::check::regionck: regionck::visit_pat(pat=pat(87: _g))
WARN:rustc::ty::util: dtorck_constraint_for_ty(1.rs:26:13: 26:15, Group<'_, std::iter::Map<std::slice::Iter<'_, i32>, [closure@1.rs:25:42: 25:47]>>, 0, Group<'_, std::iter::Map<std::slice::Iter<'_, i32>, [closure@1.rs:25:42: 25:47]>>)
WARN:rustc::ty::util: dtorck_constraint_for_ty(Group<'_, std::iter::Map<std::slice::Iter<'_, i32>, [closure@1.rs:25:42: 25:47]>>) = Ok(DtorckConstraint { outlives: [], dtorck_types: [<std::iter::Map<std::slice::Iter<'_, i32>, [closure@1.rs:25:42: 25:47]> as std::iter::Iterator>::Item] })
WARN:rustc::ty::util: dtorck_constraint_for_ty(1.rs:26:13: 26:15, Group<'_, std::iter::Map<std::slice::Iter<'_, i32>, [closure@1.rs:25:42: 25:47]>>, 1, i32)
WARN:rustc::ty::util: dtorck_constraint_for_ty(i32) = Ok(DtorckConstraint { outlives: [], dtorck_types: [] })
