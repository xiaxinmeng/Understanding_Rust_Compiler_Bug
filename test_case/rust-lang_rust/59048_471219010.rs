
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: instantiate_poly_trait_ref(TraitRef { path: path(Iterator<Item = PathBuf>), hir_ref_id: HirId { owner: DefIndex(1:115), local_id: 5 } }, def_id=DefId(2/0:2158 ~ core[284b]::iter[0]::traits[0]::iterator[0]::Iterator[0]))
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: create_substs_for_ast_trait_ref(trait_segment=PathSegment { ident: Iterator#0, hir_id: Some(HirId { owner: DefIndex(1:115), local_id: 4 }), def: Some(Trait(DefId(2/0:2158 ~ core[284b]::iter[0]::traits[0]::iterator[0]::Iterator[0]))), args: Some(GenericArgs { args: [], bindings: [TypeBinding { hir_id: HirId { owner: DefIndex(1:115), local_id: 1 }, ident: Item#0, ty: type(PathBuf), span: src/mount_info.rs:226:91: 226:105 }], parenthesized: false }), infer_types: false })
error[E0391]: cycle detected when processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve::{{impl-Trait}}`
   --> src/mount_info.rs:226:76
    |
226 |     pub fn resolve< 'a, P: AsRef< Path > >( &'a self, path: P ) -> Option< impl Iterator< Item = PathBuf > + 'a > {
    |                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: ...which requires processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve::{{impl-Trait}}`...
   --> src/mount_info.rs:226:76
    |
226 |     pub fn resolve< 'a, P: AsRef< Path > >( &'a self, path: P ) -> Option< impl Iterator< Item = PathBuf > + 'a > {
    |                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve::{{impl-Trait}}`...
   --> src/mount_info.rs:226:76
    |
226 |     pub fn resolve< 'a, P: AsRef< Path > >( &'a self, path: P ) -> Option< impl Iterator< Item = PathBuf > + 'a > {
    |                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: ...which again requires processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve::{{impl-Trait}}`, completing the cycle
note: cycle used when processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve`
   --> src/mount_info.rs:226:5
    |
226 |     pub fn resolve< 'a, P: AsRef< Path > >( &'a self, path: P ) -> Option< impl Iterator< Item = PathBuf > + 'a > {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: create_substs_for_ast_path(def_id=DefId(2/0:2158 ~ core[284b]::iter[0]::traits[0]::iterator[0]::Iterator[0]), self_ty=Some(impl ?Sized), generic_args=GenericArgs { args: [], bindings: [TypeBinding { hir_id: HirId { owner: DefIndex(1:115), local_id: 1 }, ident: Item#0, ty: type(PathBuf), span: src/mount_info.rs:226:91: 226:105 }], parenthesized: false })
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: check_kind_count: kind: lifetime required: 0 permitted: 0 provided: 0 offset: 0
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: check_kind_count: kind: const required: 0 permitted: 0 provided: 0 offset: 0
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: check_kind_count: kind: type required: 0 permitted: 0 provided: 0 offset: 0
DEBUG 2019-03-09T19:35:06Z: rustc::ty::fold: HasTypeFlagsVisitor: t=impl ?Sized t.flags=HAS_PARAMS | HAS_RE_EARLY_BOUND | HAS_FREE_REGIONS | HAS_PROJECTION | HAS_FREE_LOCAL_NAMES self.flags=KEEP_IN_LOCAL_TCX
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: ast_ty_to_ty(id=HirId { owner: DefIndex(1:115), local_id: 2 }, ast_ty=type(PathBuf) ty_ty=Path(Resolved(None, path(PathBuf))))
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: ast_ty_to_ty: maybe_qself=None path=path(PathBuf)
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: def_to_ty(def=Struct(DefId(1/0:5192 ~ std[c57b]::path[0]::PathBuf[0])), opt_self_ty=None, path_segments=[PathSegment { ident: PathBuf#0, hir_id: Some(HirId { owner: DefIndex(1:115), local_id: 3 }), def: Some(Err), args: None, infer_types: false }])
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: create_substs_for_ast_path(def_id=DefId(1/0:5192 ~ std[c57b]::path[0]::PathBuf[0]), self_ty=None, generic_args=GenericArgs { args: [], bindings: [], parenthesized: false })
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: check_kind_count: kind: lifetime required: 0 permitted: 0 provided: 0 offset: 0
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: check_kind_count: kind: const required: 0 permitted: 0 provided: 0 offset: 0
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: check_kind_count: kind: type required: 0 permitted: 0 provided: 0 offset: 0
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: create_substs_for_ast_path(generic_params=Generics { parent: None, parent_count: 0, params: [], param_def_id_to_index: {}, has_self: false, has_late_bound_regions: None }, self_ty=None) -> []
DEBUG 2019-03-09T19:35:06Z: rustc::ty::fold: HasTypeFlagsVisitor: t=std::path::PathBuf t.flags=(empty) self.flags=HAS_PARAMS | HAS_SELF | HAS_RE_EARLY_BOUND | NEEDS_SUBST
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: create_substs_for_ast_path(generic_params=Generics { parent: None, parent_count: 0, params: [Type(Self, DefId(2/0:2158 ~ core[284b]::iter[0]::traits[0]::iterator[0]::Iterator[0]), 0)], param_def_id_to_index: {DefId(2/0:2158 ~ core[284b]::iter[0]::traits[0]::iterator[0]::Iterator[0]): 0}, has_self: true, has_late_bound_regions: None }, self_ty=Some(impl ?Sized)) -> [impl ?Sized]
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: late_bound_in_trait_ref = {}
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: late_bound_in_ty = {}
DEBUG 2019-03-09T19:35:06Z: rustc::middle::stability: stability: inspecting def_id=DefId(2/0:2159 ~ core[284b]::iter[0]::traits[0]::iterator[0]::Iterator[0]::Item[0]) span=src/mount_info.rs:226:91: 226:105 of stability=Some(Stability { level: Stable { since: 1.0.0 }, feature: rust1, rustc_depr: None, const_stability: None, promotable: false })
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: instantiate_poly_trait_ref(TraitRef { path: path(Iterator<Item = PathBuf>), hir_ref_id: HirId { owner: DefIndex(1:115), local_id: 5 } }, projections=[(Binder(ProjectionPredicate(ProjectionTy { substs: [impl ?Sized], item_def_id: DefId(2/0:2159 ~ core[284b]::iter[0]::traits[0]::iterator[0]::Iterator[0]::Item[0]) }, std::path::PathBuf)), src/mount_info.rs:226:91: 226:105)]) -> Binder(<impl ?Sized as std::iter::Iterator>)
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::astconv: ast_region_to_region(lifetime=lifetime(HirId { owner: DefIndex(1:115), local_id: 6 }: 'a)) yields ReEarlyBound(1, 'a)
DEBUG 2019-03-09T19:35:06Z: rustc::ty::fold: HasTypeFlagsVisitor: t=impl ?Sized t.flags=HAS_PARAMS | HAS_RE_EARLY_BOUND | HAS_FREE_REGIONS | HAS_PROJECTION | HAS_FREE_LOCAL_NAMES self.flags=KEEP_IN_LOCAL_TCX
error[E0391]: cycle detected when processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve::{{impl-Trait}}`
   --> src/mount_info.rs:226:76
    |
226 |     pub fn resolve< 'a, P: AsRef< Path > >( &'a self, path: P ) -> Option< impl Iterator< Item = PathBuf > + 'a > {
    |                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: ...which requires processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve::{{impl-Trait}}`...
   --> src/mount_info.rs:226:76
    |
226 |     pub fn resolve< 'a, P: AsRef< Path > >( &'a self, path: P ) -> Option< impl Iterator< Item = PathBuf > + 'a > {
    |                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: ...which again requires processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve::{{impl-Trait}}`, completing the cycle
note: cycle used when processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve`
   --> src/mount_info.rs:226:5
    |
226 |     pub fn resolve< 'a, P: AsRef< Path > >( &'a self, path: P ) -> Option< impl Iterator< Item = PathBuf > + 'a > {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::collect: predicates_defined_on: explicit_predicates_of(DefId(0/1:115 ~ nperf[6bb5]::mount_info[0]::{{impl}}[5]::resolve[0]::{{impl-Trait}}[0])) = GenericPredicates([(Binder(TraitPredicate(<impl ?Sized as std::marker::Sized>)), src/mount_info.rs:226:76: 226:112), (Binder(OutlivesPredicate(impl ?Sized, ReEarlyBound(1, 'a))), src/mount_info.rs:226:110: 226:112), (Binder(TraitPredicate(<impl ?Sized as std::iter::Iterator>)), src/mount_info.rs:226:81: 226:107), (Binder(ProjectionPredicate(ProjectionTy { substs: [impl ?Sized], item_def_id: DefId(2/0:2159 ~ core[284b]::iter[0]::traits[0]::iterator[0]::Iterator[0]::Item[0]) }, std::path::PathBuf)), src/mount_info.rs:226:91: 226:105)])
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::collect: predicates_defined_on(DefId(0/1:115 ~ nperf[6bb5]::mount_info[0]::{{impl}}[5]::resolve[0]::{{impl-Trait}}[0])) = GenericPredicates([(Binder(TraitPredicate(<impl ?Sized as std::marker::Sized>)), src/mount_info.rs:226:76: 226:112), (Binder(OutlivesPredicate(impl ?Sized, ReEarlyBound(1, 'a))), src/mount_info.rs:226:110: 226:112), (Binder(TraitPredicate(<impl ?Sized as std::iter::Iterator>)), src/mount_info.rs:226:81: 226:107), (Binder(ProjectionPredicate(ProjectionTy { substs: [impl ?Sized], item_def_id: DefId(2/0:2159 ~ core[284b]::iter[0]::traits[0]::iterator[0]::Iterator[0]::Item[0]) }, std::path::PathBuf)), src/mount_info.rs:226:91: 226:105)])
error[E0391]: cycle detected when processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve::{{impl-Trait}}`
   --> src/mount_info.rs:226:76
    |
226 |     pub fn resolve< 'a, P: AsRef< Path > >( &'a self, path: P ) -> Option< impl Iterator< Item = PathBuf > + 'a > {
    |                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: ...which again requires processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve::{{impl-Trait}}`, completing the cycle
note: cycle used when processing `mount_info::<impl at src/mount_info.rs:195:1: 246:2>::resolve`
   --> src/mount_info.rs:226:5
    |
226 |     pub fn resolve< 'a, P: AsRef< Path > >( &'a self, path: P ) -> Option< impl Iterator< Item = PathBuf > + 'a > {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
DEBUG 2019-03-09T19:35:06Z: rustc_typeck::collect: predicates_of(def_id=DefId(0/1:115 ~ nperf[6bb5]::mount_info[0]::{{impl}}[5]::resolve[0]::{{impl-Trait}}[0])) = GenericPredicates([(Binder(TraitPredicate(<impl ?Sized as std::marker::Sized>)), src/mount_info.rs:226:76: 226:112), (Binder(OutlivesPredicate(impl ?Sized, ReEarlyBound(1, 'a))), src/mount_info.rs:226:110: 226:112), (Binder(TraitPredicate(<impl ?Sized as std::iter::Iterator>)), src/mount_info.rs:226:81: 226:107), (Binder(ProjectionPredicate(ProjectionTy { substs: [impl ?Sized], item_def_id: DefId(2/0:2159 ~ core[284b]::iter[0]::traits[0]::iterator[0]::Iterator[0]::Item[0]) }, std::path::PathBuf)), src/mount_info.rs:226:91: 226:105)])
thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
[2019-03-09T19:37:29Z DEBUG cargo] exit_with_error; err=CliError { error: Some(ProcessError { desc: "process didn\'t exit successfully: `rustc --crate-name nperf src/main.rs --color never --crate-type bin --emit=dep-info,link -C debuginfo=2 --cfg \'feature=\"default\"\' --cfg \'feature=\"logging\"\' --cfg \'feature=\"nwind\"\' -C metadata=f040a09e21ff6e26 -C extra-filename=-f040a09e21ff6e26 --out-dir /Users/ekuber/personal/not-perf/target/debug/deps -C incremental=/Users/ekuber/personal/not-perf/target/debug/incremental -L dependency=/Users/ekuber/personal/not-perf/target/debug/deps --extern chrono=/Users/ekuber/personal/not-perf/target/debug/deps/libchrono-5ff88e354277f83f.rlib --extern env_logger=/Users/ekuber/personal/not-perf/target/debug/deps/libenv_logger-78c077948f2c02ce.rlib --extern lazy_static=/Users/ekuber/personal/not-perf/target/debug/deps/liblazy_static-ad42470069693248.rlib --extern libc=/Users/ekuber/personal/not-perf/target/debug/deps/liblibc-6a79fbdcb2893d3f.rlib --extern log=/Users/ekuber/personal/not-perf/target/debug/deps/liblog-bf648d25a1b76985.rlib --extern nperf=/Users/ekuber/personal/not-perf/target/debug/deps/libnperf-3b4c16f1b25149b2.rlib --extern num_cpus=/Users/ekuber/personal/not-perf/target/debug/deps/libnum_cpus-296401efbf15c812.rlib --extern nwind=/Users/ekuber/personal/not-perf/target/debug/deps/libnwind-520014cbc8c13870.rlib --extern parking_lot=/Users/ekuber/personal/not-perf/target/debug/deps/libparking_lot-4d5bb6c64055548b.rlib --extern perf_event_open=/Users/ekuber/personal/not-perf/target/debug/deps/libperf_event_open-f6968ba9ab7abf76.rlib --extern proc_maps=/Users/ekuber/personal/not-perf/target/debug/deps/libproc_maps-2626b87765fa0129.rlib --extern regex=/Users/ekuber/personal/not-perf/target/debug/deps/libregex-7c1527dad8c6100a.rlib --extern serde=/Users/ekuber/personal/not-perf/target/debug/deps/libserde-1b9624f31c5af207.rlib --extern serde_derive=/Users/ekuber/personal/not-perf/target/debug/deps/libserde_derive-743003eaf65dae3a.dylib --extern serde_json=/Users/ekuber/personal/not-perf/target/debug/deps/libserde_json-88b42f27d496342d.rlib --extern speedy=/Users/ekuber/personal/not-perf/target/debug/deps/libspeedy-a7d7df9a114a8bee.rlib --extern speedy_derive=/Users/ekuber/personal/not-perf/target/debug/deps/libspeedy_derive-af1054e99f3503c8.dylib --extern structopt=/Users/ekuber/personal/not-perf/target/debug/deps/libstructopt-42fb8d13ffa79d89.rlib` (signal: 6, SIGABRT: process abort signal)", exit: Some(ExitStatus(ExitStatus(6))), output: None }
