plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: lifetime may not live long enough
   --> compiler/rustc_hir_analysis/src/check/inherited.rs:92:9
    |
88  |   impl<'tcx> Inherited<'_, 'tcx> {
    |        ---- lifetime `'tcx` defined here
92  | /         InheritedBuilder {
92  | /         InheritedBuilder {
93  | |             infcx: tcx
94  | |                 .infer_ctxt()
95  | |                 .ignoring_regions()
119 | |             def_id,
120 | |         }
120 | |         }
    | |_________^ returning this value requires that `'tcx` must outlive `'static`
    |
    = note: requirement occurs because of the type `InheritedBuilder<'_>`, which makes the generic argument `'_` invariant
    = note: the struct `InheritedBuilder<'tcx>` is invariant over the parameter `'tcx`
    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error: lifetime may not live long enough
   --> compiler/rustc_hir_analysis/src/check/intrinsicck.rs:113:9
    |
    |
111 |   impl<'a, 'tcx> InlineAsmCtxt<'a, 'tcx> {
    |        --  ---- lifetime `'tcx` defined here
    |        |
    |        lifetime `'a` defined here
112 |       pub fn new_global_asm(tcx: TyCtxt<'tcx>) -> Self {
113 | /         InlineAsmCtxt {
114 | |             tcx,
115 | |             param_env: ty::ParamEnv::empty(),
116 | |             get_operand_ty: Box::new(|e| bug!("asm operand in global asm: {e:?}")),
117 | |         }
    | |_________^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'tcx`
    |
    = help: consider adding the following bound: `'tcx: 'a`
[RUSTC-TIMING] rustc_expand test:false 60.431
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
[RUSTC-TIMING] rustc_hir_analysis test:false 10.528
error: could not compile `rustc_hir_analysis` due to 2 previous errors
