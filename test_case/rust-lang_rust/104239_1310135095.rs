plain
   Compiling rustc_hir_analysis v0.0.0 (/checkout/compiler/rustc_hir_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0407]: method `replace_late_bound_regions_with_nll_infer_vars` is not a member of trait `InferCtxtExt`
    |
    |
894 |       #[instrument(skip(self, indices))]
    |       |
    |       not a member of trait `InferCtxtExt`
    |       in this procedural macro expansion
    |       in this procedural macro expansion
895 |       fn replace_late_bound_regions_with_nll_infer_vars(
    |          ---------------------------------------------- help: there is an associated function with a similar name: `replace_bound_regions_with_nll_infer_vars`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-attributes-0.1.22/src/lib.rs:552:1
    |
552 | / pub fn instrument(
553 | |     args: proc_macro::TokenStream,
553 | |     args: proc_macro::TokenStream,
554 | |     item: proc_macro::TokenStream,
555 | | ) -> proc_macro::TokenStream {
    | |____________________________- in this expansion of `#[instrument]`

error[E0425]: cannot find function `for_each_late_bound_region_defined_on` in this scope
    |
    |
901 |           for_each_late_bound_region_defined_on(self.tcx, typeck_root_def_id, |r| {
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `for_each_late_bound_region_in_item`
...
982 | / fn for_each_late_bound_region_in_item<'tcx>(
983 | |     tcx: TyCtxt<'tcx>,
984 | |     mir_def_id: LocalDefId,
985 | |     mut f: impl FnMut(ty::Region<'tcx>),
996 | |     }
997 | | }
997 | | }
    | |_- similarly named function `for_each_late_bound_region_in_item` defined here

error[E0046]: not all trait items implemented, missing: `replace_late_bound_regions_with_nll_infer_vars_in_recursive_scope`
    |
    |
734 | /     fn replace_late_bound_regions_with_nll_infer_vars_in_recursive_scope(
735 | |         &self,
736 | |         mir_def_id: LocalDefId,
737 | |         indices: &mut UniversalRegionIndices<'tcx>,
738 | |     );
    | |______- `replace_late_bound_regions_with_nll_infer_vars_in_recursive_scope` from trait
...
747 |   impl<'cx, 'tcx> InferCtxtExt<'tcx> for BorrowckInferCtxt<'cx, 'tcx> {
    |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `replace_late_bound_regions_with_nll_infer_vars_in_recursive_scope` in implementation
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
   --> compiler/rustc_borrowck/src/type_check/relate_tys.rs:135:45
    |
    |
135 |                 ty::BoundRegionKind::BrAnon(_) => Symbol::intern("anon"),
    |
   ::: /checkout/compiler/rustc_middle/src/ty/sty.rs:62:12
    |
    |
62  |     BrAnon(u32, Option<Span>),
    |
help: use `_` to explicitly ignore each field
    |
    |
135 |                 ty::BoundRegionKind::BrAnon(_, _) => Symbol::intern("anon"),
help: use `..` to ignore all fields
    |
    |
135 |                 ty::BoundRegionKind::BrAnon(..) => Symbol::intern("anon"),

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> compiler/rustc_borrowck/src/type_check/mod.rs:1431:57
     |
     |
1431 | ...                   ty::BoundRegionKind::BrAnon(_) => Symbol::intern("anon"),
     |
    ::: /checkout/compiler/rustc_middle/src/ty/sty.rs:62:12
     |
     |
62   |     BrAnon(u32, Option<Span>),
     |
help: use `_` to explicitly ignore each field
     |
     |
1431 |                             ty::BoundRegionKind::BrAnon(_, _) => Symbol::intern("anon"),
help: use `..` to ignore all fields
     |
     |
1431 |                             ty::BoundRegionKind::BrAnon(..) => Symbol::intern("anon"),

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_borrowck/src/universal_regions.rs:446:53
    |
    |
446 |                         let region_vid = self.infcx.next_nll_region_var(FR);
    |                                                     ^^^^^^^^^^^^^^^^^^^---- an argument of type `RegionCtxt` is missing
note: associated function defined here
   --> compiler/rustc_borrowck/src/lib.rs:546:19
    |
    |
546 |     pub(crate) fn next_nll_region_var(
547 |         &self,
547 |         &self,
548 |         origin: NllRegionVariableOrigin,
    |         -------------------------------
549 |         ctxt: RegionCtxt,
help: provide the argument
    |
    |
446 |                         let region_vid = self.infcx.next_nll_region_var(FR, /* RegionCtxt */);

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_borrowck/src/universal_regions.rs:474:45
    |
    |
474 |                 let region_vid = self.infcx.next_nll_region_var(FR);
    |                                             ^^^^^^^^^^^^^^^^^^^---- an argument of type `RegionCtxt` is missing
note: associated function defined here
   --> compiler/rustc_borrowck/src/lib.rs:546:19
    |
    |
546 |     pub(crate) fn next_nll_region_var(
547 |         &self,
547 |         &self,
548 |         origin: NllRegionVariableOrigin,
    |         -------------------------------
549 |         ctxt: RegionCtxt,
help: provide the argument
    |
    |
474 |                 let region_vid = self.infcx.next_nll_region_var(FR, /* RegionCtxt */);

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_borrowck/src/universal_regions.rs:877:39
    |
    |
877 |                 let region_vid = self.next_nll_region_var(FR);
    |                                       ^^^^^^^^^^^^^^^^^^^---- an argument of type `RegionCtxt` is missing
note: associated function defined here
   --> compiler/rustc_borrowck/src/lib.rs:546:19
    |
    |
546 |     pub(crate) fn next_nll_region_var(
547 |         &self,
547 |         &self,
548 |         origin: NllRegionVariableOrigin,
    |         -------------------------------
549 |         ctxt: RegionCtxt,
help: provide the argument
    |
    |
877 |                 let region_vid = self.next_nll_region_var(FR, /* RegionCtxt */);

Some errors have detailed explanations: E0023, E0046, E0061, E0407, E0425.
For more information about an error, try `rustc --explain E0023`.
error: could not compile `rustc_borrowck` due to 8 previous errors
