plain
[00:08:52]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]   --> librustc/hir/def_id.rs:39:29
[00:08:57]    |
[00:08:57] 39 |     fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
[00:08:57] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/middle/region.rs:110:29
[00:08:57]     |
[00:08:57]     |
[00:08:57] 110 |     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
[00:08:57]     |                             ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:57] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:304:27
[00:08:57]     |
[00:08:57] 304 |     pub fn iter(&self) -> hash_map::Iter<hir::ItemLocalId, V> {
[00:08:57]     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `hash_map::Iter<'_, hir::ItemLocalId, V>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:328:48
[00:08:57]     |
[00:08:57]     |
[00:08:57] 328 |     pub fn entry(&mut self, id: hir::HirId) -> Entry<hir::ItemLocalId, V> {
[00:08:57]     |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `Entry<'_, hir::ItemLocalId, V>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:486:42
[00:08:57]     |
[00:08:57]     |
[00:08:57] 486 |     pub fn type_dependent_defs(&self) -> LocalTableInContext<Def> {
[00:08:57]     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, Def>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:493:50
[00:08:57]     |
[00:08:57]     |
[00:08:57] 493 |     pub fn type_dependent_defs_mut(&mut self) -> LocalTableInContextMut<Def> {
[00:08:57]     |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, Def>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:500:36
[00:08:57]     |
[00:08:57]     |
[00:08:57] 500 |     pub fn field_indices(&self) -> LocalTableInContext<usize> {
[00:08:57]     |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, usize>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:507:44
[00:08:57]     |
[00:08:57]     |
[00:08:57] 507 |     pub fn field_indices_mut(&mut self) -> LocalTableInContextMut<usize> {
[00:08:57]     |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, usize>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:514:40
[00:08:57]     |
[00:08:57]     |
[00:08:57] 514 |     pub fn user_provided_tys(&self) -> LocalTableInContext<CanonicalTy<'tcx>> {
[00:08:57]     |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, CanonicalTy<'tcx>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:521:48
[00:08:57]     |
[00:08:57]     |
[00:08:57] 521 |     pub fn user_provided_tys_mut(&mut self) -> LocalTableInContextMut<CanonicalTy<'tcx>> {
[00:08:57]     |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, CanonicalTy<'tcx>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:528:33
[00:08:57]     |
[00:08:57]     |
[00:08:57] 528 |     pub fn node_types(&self) -> LocalTableInContext<Ty<'tcx>> {
[00:08:57]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, Ty<'tcx>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:535:41
[00:08:57]     |
[00:08:57]     |
[00:08:57] 535 |     pub fn node_types_mut(&mut self) -> LocalTableInContextMut<Ty<'tcx>> {
[00:08:57]     |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, Ty<'tcx>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:560:42
[00:08:57]     |
[00:08:57]     |
[00:08:57] 560 |     pub fn node_substs_mut(&mut self) -> LocalTableInContextMut<&'tcx Substs<'tcx>> {
[00:08:57]     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, &'tcx Substs<'tcx>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:577:42
[00:08:57]     |
[00:08:57]     |
[00:08:57] 577 |     pub fn user_substs_mut(&mut self) -> LocalTableInContextMut<CanonicalSubsts<'tcx>> {
[00:08:57]     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, CanonicalSubsts<'tcx>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:617:34
[00:08:57]     |
[00:08:57]     |
[00:08:57] 617 |     pub fn adjustments(&self) -> LocalTableInContext<Vec<ty::adjustment::Adjustment<'tcx>>> {
[00:08:57]     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, Vec<ty::adjustment::Adjustment<'tcx>>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:625:31
[00:08:57]     |
[00:08:57]     |
[00:08:57] 625 |                            -> LocalTableInContextMut<Vec<ty::adjustment::Adjustment<'tcx>>> {
[00:08:57]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, Vec<ty::adjustment::Adjustment<'tcx>>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:666:40
[00:08:57]     |
[00:08:57]     |
[00:08:57] 666 |     pub fn pat_binding_modes(&self) -> LocalTableInContext<BindingMode> {
[00:08:57]     |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, BindingMode>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:674:31
[00:08:57]     |
[00:08:57]     |
[00:08:57] 674 |                            -> LocalTableInContextMut<BindingMode> {
[00:08:57]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, BindingMode>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:681:38
[00:08:57]     |
[00:08:57]     |
[00:08:57] 681 |     pub fn pat_adjustments(&self) -> LocalTableInContext<Vec<Ty<'tcx>>> {
[00:08:57]     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, Vec<Ty<'tcx>>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:689:31
[00:08:57]     |
[00:08:57]     |
[00:08:57] 689 |                            -> LocalTableInContextMut<Vec<Ty<'tcx>>> {
[00:08:57]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, Vec<Ty<'tcx>>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:700:43
[00:08:57]     |
[00:08:57]     |
[00:08:57] 700 |     pub fn closure_kind_origins(&self) -> LocalTableInContext<(Span, ast::Name)> {
[00:08:57]     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, (Span, ast::Name)>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:707:51
[00:08:57]     |
[00:08:57]     |
[00:08:57] 707 |     pub fn closure_kind_origins_mut(&mut self) -> LocalTableInContextMut<(Span, ast::Name)> {
[00:08:57]     |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, (Span, ast::Name)>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:714:40
[00:08:57]     |
[00:08:57]     |
[00:08:57] 714 |     pub fn liberated_fn_sigs(&self) -> LocalTableInContext<ty::FnSig<'tcx>> {
[00:08:57]     |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, ty::FnSig<'tcx>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:721:48
[00:08:57]     |
[00:08:57]     |
[00:08:57] 721 |     pub fn liberated_fn_sigs_mut(&mut self) -> LocalTableInContextMut<ty::FnSig<'tcx>> {
[00:08:57]     |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, ty::FnSig<'tcx>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:728:38
[00:08:57]     |
[00:08:57]     |
[00:08:57] 728 |     pub fn fru_field_types(&self) -> LocalTableInContext<Vec<Ty<'tcx>>> {
[00:08:57]     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, Vec<Ty<'tcx>>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:735:46
[00:08:57]     |
[00:08:57]     |
[00:08:57] 735 |     pub fn fru_field_types_mut(&mut self) -> LocalTableInContextMut<Vec<Ty<'tcx>>> {
[00:08:57]     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, Vec<Ty<'tcx>>>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:742:33
[00:08:57]     |
[00:08:57]     |
[00:08:57] 742 |     pub fn cast_kinds(&self) -> LocalTableInContext<ty::cast::CastKind> {
[00:08:57]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContext<'_, ty::cast::CastKind>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> librustc/ty/context.rs:749:41
[00:08:57]     |
[00:08:57]     |
[00:08:57] 749 |     pub fn cast_kinds_mut(&mut self) -> LocalTableInContextMut<ty::cast::CastKind> {
[00:08:57]     |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `LocalTableInContextMut<'_, ty::cast::CastKind>`
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]     --> librustc/ty/context.rs:1899:51
[00:08:57]      |
[00:08:57]      |
[00:08:57] 1899 |     fn span_debug(span: syntax_pos::Span, f: &mut fmt::Formatter) -> fmt::Result {
[00:08:57] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]     --> librustc/ty/context.rs:2022:33
[00:08:57]      |
[00:08:57]      |
[00:08:57] 2022 |             sync::assert_sync::<ImplicitCtxt>();
[00:08:57] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]     --> librustc/ty/context.rs:2024:50
[00:08:57]      |
[00:08:57]      |
[00:08:57] 2024 |             unsafe { f(Some(&*(context as *const ImplicitCtxt))) }
[00:08:57] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]     --> librustc/ty/context.rs:2048:31
[00:08:57]      |
[00:08:57]      |
[00:08:57] 2048 |                 let context: &ImplicitCtxt = mem::transmute(context);
[00:08:57] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]     --> librustc/ty/context.rs:2068:31
[00:08:57]      |
[00:08:57]      |
[00:08:57] 2068 |                 let context: &ImplicitCtxt = mem::transmute(context);
[00:08:57] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]     --> librustc/ty/context.rs:2108:28
[00:08:57]      |
[00:08:57]      |
[00:08:57] 2108 |               pub fn go(tcx: TyCtxt) {
[00:08:57] ...
[00:08:57] 2157 | /         sty_debug_print!(
[00:08:57] 2158 | |             self,
[00:08:57] 2158 | |             self,
[00:08:57] 2159 | |             Adt, Array, Slice, RawPtr, Ref, FnDef, FnPtr,
[00:08:57] 2160 | |             Generator, GeneratorWitness, Dynamic, Closure, Tuple,
[00:08:57] 2161 | |             Param, Infer, Projection, Opaque, Foreign);
[00:08:57] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]     --> librustc/ty/context.rs:2350:27
[00:08:57]      |
[00:08:57]      |
[00:08:57] 2350 |     const_: mk_const(|c: &Const| keep_local(&c.ty) || keep_local(&c.val)) -> Const<'tcx>
[00:08:57] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]     --> librustc/ty/context.rs:2359:20
[00:08:57]      |
[00:08:57]      |
[00:08:57] 2359 |               |xs: &[$ty]| xs.iter().any(keep_local)) -> List<$ty<'tcx>>);)+
[00:08:57]      |                      ^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:57] ...
[00:08:57] 2363 | / slice_interners!(
[00:08:57] 2364 | |     existential_predicates: _intern_existential_predicates(ExistentialPredicate),
[00:08:57] 2365 | |     predicates: _intern_predicates(Predicate),
[00:08:57] 2366 | |     type_list: _intern_type_list(Ty),
[00:08:57] ...    |
[00:08:57] 2369 | |     goals: _intern_goals(Goal)
[00:08:57] 2370 | | );
[00:08:57]      | |__- in this macro invocation
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]     --> librustc/ty/context.rs:2749:53
[00:08:57]      |
[00:08:57]      |
[00:08:57] 2749 |     pub fn mk_goal(self, goal: Goal<'tcx>) -> &'tcx Goal {
[00:08:57] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]     --> librustc/ty/context.rs:2916:32
[00:08:57]      |
[00:08:57]      |
[00:08:57] 2916 | pub fn provide(providers: &mut ty::query::Providers) {
[00:08:57] 
