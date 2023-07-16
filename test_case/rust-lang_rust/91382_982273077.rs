plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0053]: method `fold_ty` has an incompatible type for trait
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:913:43
    |
913 |     fn fold_ty(&mut self, t: Ty<'tcx>) -> Ty<'tcx> {
    |                                           |
    |                                           |
    |                                           expected enum `Result`, found `&TyS<'tcx>`
    |                                           help: change the output type to match the trait: `Result<&'tcx TyS<'tcx>, !>`
    |
    = note: expected fn pointer `fn(&mut ResolvedTypeParamEraser<'tcx>, &'tcx TyS<'tcx>) -> Result<&'tcx TyS<'tcx>, !>`
               found fn pointer `fn(&mut ResolvedTypeParamEraser<'tcx>, &'tcx TyS<'tcx>) -> &'tcx TyS<'tcx>`

error[E0053]: method `fold_ty` has an incompatible type for trait
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:990:43
    |
990 |     fn fold_ty(&mut self, t: Ty<'tcx>) -> Ty<'tcx> {
    |                                           |
    |                                           |
    |                                           expected enum `Result`, found `&TyS<'tcx>`
    |                                           help: change the output type to match the trait: `Result<&'tcx TyS<'tcx>, !>`
    |
    = note: expected fn pointer `fn(&mut ErrTypeParamEraser<'tcx>, &'tcx TyS<'tcx>) -> Result<&'tcx TyS<'tcx>, !>`
               found fn pointer `fn(&mut ErrTypeParamEraser<'tcx>, &'tcx TyS<'tcx>) -> &'tcx TyS<'tcx>`
error[E0308]: mismatched types
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:603:63
    |
    |
603 |                 let ty = ErrTypeParamEraser(self.tcx).fold_ty(ty);
    |                                                               ^^ expected `&TyS<'_>`, found enum `Result`
    |
    = note: expected reference `&TyS<'_>`
                    found enum `Result<&TyS<'_>, !>`
error[E0308]: mismatched types
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:604:39
    |
    |
604 |                 let ty = ty_to_string(ty);
    |                                       ^^ expected `&TyS<'tcx>`, found enum `Result`
    |
    = note: expected reference `&'tcx TyS<'tcx>`
                    found enum `Result<&TyS<'_>, !>`

error[E0308]: `match` arms have incompatible types
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:930:30
    |
925 |                       .map(|(subst, param)| match &(subst.unpack(), &param.kind) {
    |  ___________________________________________-
926 | |                         (_, ty::GenericParamDefKind::Type { has_default: true, .. }) => subst,
    | |                                                                                         ----- this is found to be of type `rustc_middle::ty::subst::GenericArg<'_>`
927 | |                         (crate::infer::GenericArgKind::Const(c), _) => {
928 | |                             self.replace_infers(c, param.index, param.name).into()
    | |                             ------------------------------------------------------ this is found to be of type `rustc_middle::ty::subst::GenericArg<'_>`
929 | |                         }
930 | |                         _ => subst.super_fold_with(self),
    | |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_middle::ty::subst::GenericArg`, found enum `Result`
931 | |                     })
    | |_____________________- `match` arms have incompatible types
    |
    = note: expected type `rustc_middle::ty::subst::GenericArg<'_>`
               found enum `Result<rustc_middle::ty::subst::GenericArg<'_>, !>`
error[E0599]: no method named `kind` found for enum `Result` in the current scope
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:952:26
    |
952 |                 match ty.kind() {
952 |                 match ty.kind() {
    |                          ^^^^ method not found in `Result<&TyS<'_>, !>`

error[E0308]: `match` arms have incompatible types
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:955:26
952 | /                 match ty.kind() {
952 | /                 match ty.kind() {
953 | |                     // Avoid `&_`, these can be safely presented as `_`.
954 | |                     ty::Error(_) => self.tcx().ty_error(),
    | |                                     --------------------- this is found to be of type `&TyS<'_>`
955 | |                     _ => t.super_fold_with(self),
    | |                          ^^^^^^^^^^^^^^^^^^^^^^^ expected `&TyS<'_>`, found enum `Result`
956 | |                 }
    | |_________________- `match` arms have incompatible types
    |
    = note: expected type `&TyS<'_>`
               found enum `Result<&TyS<'_>, !>`
error[E0308]: mismatched types
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:969:34
    |
    |
969 |                 .mk_ty(ty::Array(self.fold_ty(ty), self.replace_infers(c, 0, Symbol::intern("N")))),
    |                                  ^^^^^^^^^^^^^^^^ expected `&TyS<'_>`, found enum `Result`
    |
    = note: expected reference `&TyS<'_>`
                    found enum `Result<&TyS<'_>, !>`

error[E0308]: `match` arms have incompatible types
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:993:18
991 | /         match t.kind() {
991 | /         match t.kind() {
992 | |             ty::Error(_) => self.tcx().mk_ty_var(ty::TyVid::from_u32(0)),
    | |                             -------------------------------------------- this is found to be of type `&'tcx TyS<'tcx>`
993 | |             _ => t.super_fold_with(self),
    | |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected `&TyS<'tcx>`, found enum `Result`
994 | |         }
    | |_________- `match` arms have incompatible types
    |
    = note: expected reference `&'tcx TyS<'tcx>`
                    found enum `Result<&TyS<'_>, !>`
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
Some errors have detailed explanations: E0053, E0308, E0599.
For more information about an error, try `rustc --explain E0053`.
