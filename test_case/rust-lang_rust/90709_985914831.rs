plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0053]: method `fold_ty` has an incompatible type for trait
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:913:43
    |
913 |     fn fold_ty(&mut self, t: Ty<'tcx>) -> Result<Ty<'tcx>, !> {
    |                                           |
    |                                           |
    |                                           expected `&TyS<'tcx>`, found enum `Result`
    |                                           help: change the output type to match the trait: `&'tcx TyS<'tcx>`
    |
    = note: expected fn pointer `fn(&mut ResolvedTypeParamEraser<'tcx>, &'tcx TyS<'tcx>) -> &'tcx TyS<'tcx>`
               found fn pointer `fn(&mut ResolvedTypeParamEraser<'tcx>, &'tcx TyS<'tcx>) -> Result<&'tcx TyS<'tcx>, !>`

error[E0053]: method `fold_ty` has an incompatible type for trait
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:991:43
    |
991 |     fn fold_ty(&mut self, t: Ty<'tcx>) -> Result<Ty<'tcx>, !> {
    |                                           |
    |                                           |
    |                                           expected `&TyS<'tcx>`, found enum `Result`
    |                                           help: change the output type to match the trait: `&'tcx TyS<'tcx>`
    |
    = note: expected fn pointer `fn(&mut ErrTypeParamEraser<'tcx>, &'tcx TyS<'tcx>) -> &'tcx TyS<'tcx>`
               found fn pointer `fn(&mut ErrTypeParamEraser<'tcx>, &'tcx TyS<'tcx>) -> Result<&'tcx TyS<'tcx>, !>`

error[E0599]: no method named `unwrap` found for reference `&TyS<'_>` in the current scope
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:602:77
    |
602 |                 let ty = ResolvedTypeParamEraser::new(self.tcx).fold_ty(ty).unwrap();
    |                                                                             ^^^^^^ method not found in `&TyS<'_>`

error[E0599]: no method named `unwrap` found for reference `&TyS<'_>` in the current scope
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:603:67
    |
603 |                 let ty = ErrTypeParamEraser(self.tcx).fold_ty(ty).unwrap();
    |                                                                   ^^^^^^ method not found in `&TyS<'_>`

error[E0308]: `match` arms have incompatible types
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:930:30
    |
925 |                       .map(|(subst, param)| match &(subst.unpack(), &param.kind) {
    |  ___________________________________________-
926 | |                         (_, ty::GenericParamDefKind::Type { has_default: true, .. }) => Ok(subst),
    | |                                                                                         --------- this is found to be of type `Result<rustc_middle::ty::subst::GenericArg<'_>, _>`
927 | |                         (crate::infer::GenericArgKind::Const(c), _) => {
928 | |                             Ok(self.replace_infers(c, param.index, param.name).into())
    | |                             ---------------------------------------------------------- this is found to be of type `Result<rustc_middle::ty::subst::GenericArg<'_>, _>`
929 | |                         }
930 | |                         _ => subst.super_fold_with(self),
    | |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Result`, found struct `rustc_middle::ty::subst::GenericArg`
931 | |                     })
    | |_____________________- `match` arms have incompatible types
    |
    = note: expected enum `Result<rustc_middle::ty::subst::GenericArg<'_>, _>`
             found struct `rustc_middle::ty::subst::GenericArg<'_>`
help: try wrapping the expression in a variant of `Result`
    |
930 |                         _ => Ok(subst.super_fold_with(self)),
    |                              +++                           +
930 |                         _ => Err(subst.super_fold_with(self)),
    |                              ++++                           +
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:951:26
    |
    |
951 |                 let ty = self.fold_ty(ty)?;
    |                          ^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `&TyS<'_>`
    |
    = help: the trait `std::ops::Try` is not implemented for `&TyS<'_>`
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:955:26
    |
    |
955 |                     _ => t.super_fold_with(self)?,
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `&TyS<'_>`
    |
    = help: the trait `std::ops::Try` is not implemented for `&TyS<'_>`
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:966:28
    |
    |
966 |             | ty::Never => t.super_fold_with(self)?,
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `&TyS<'_>`
    |
    = help: the trait `std::ops::Try` is not implemented for `&TyS<'_>`
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:968:17
    |
    |
968 |                 self.fold_ty(ty)?,
    |                 ^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `&TyS<'_>`
    |
    = help: the trait `std::ops::Try` is not implemented for `&TyS<'_>`
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:976:37
    |
    |
976 |             _ if self.level == 1 => t.super_fold_with(self)?,
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `&TyS<'_>`
    |
    = help: the trait `std::ops::Try` is not implemented for `&TyS<'_>`

error[E0308]: `match` arms have incompatible types
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:994:18
992 | /         match t.kind() {
992 | /         match t.kind() {
993 | |             ty::Error(_) => Ok(self.tcx().mk_ty_var(ty::TyVid::from_u32(0))),
    | |                             ------------------------------------------------ this is found to be of type `Result<&'tcx TyS<'tcx>, !>`
994 | |             _ => t.super_fold_with(self),
    | |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Result`, found `&TyS<'_>`
995 | |         }
    | |_________- `match` arms have incompatible types
    |
    = note:   expected enum `Result<&'tcx TyS<'tcx>, !>`
            found reference `&TyS<'_>`
help: try wrapping the expression in `Ok`
    |
994 |             _ => Ok(t.super_fold_with(self)),
    |                  +++                       +
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
Some errors have detailed explanations: E0053, E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0053`.
