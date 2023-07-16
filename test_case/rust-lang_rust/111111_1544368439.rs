plain
   Compiling basic-toml v0.1.2
   Compiling askama_derive v0.12.1
    Checking askama v0.12.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0433]: failed to resolve: could not find `ArrayLen` in `hir`
    --> src/librustdoc/clean/mod.rs:1673:22
     |
1673 |                 hir::ArrayLen::Infer(_, _) => "_".to_string(),
     |                      ^^^^^^^^ could not find `ArrayLen` in `hir`

error[E0433]: failed to resolve: could not find `ArrayLen` in `hir`
    --> src/librustdoc/clean/mod.rs:1674:22
     |
1674 |                 hir::ArrayLen::Body(anon_const) => {
     |                      ^^^^^^^^ could not find `ArrayLen` in `hir`
error: hidden lifetime parameters in types are deprecated
   --> src/librustdoc/clean/mod.rs:249:49
    |
    |
249 | pub(crate) fn clean_const<'tcx>(constant: &hir::ConstArg, cx: &mut DocContext<'tcx>) -> Constant {
    |                                            |
    |                                            expected lifetime parameter
    |
    |
    = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
help: indicate the anonymous lifetime
    |
249 | pub(crate) fn clean_const<'tcx>(constant: &hir::ConstArg<'_>, cx: &mut DocContext<'tcx>) -> Constant {


error[E0609]: no field `value` on type `&ConstArg<'_>`
   --> src/librustdoc/clean/mod.rs:250:58
    |
250 |     let def_id = cx.tcx.hir().body_owner_def_id(constant.value.body).to_def_id();
    |
    = note: available fields are: `kind`


error[E0609]: no field `value` on type `&ConstArg<'_>`
   --> src/librustdoc/clean/mod.rs:257:56
    |
257 |         kind: ConstantKind::Anonymous { body: constant.value.body },
    |
    = note: available fields are: `kind`


error[E0609]: no field `def_id` on type `&ConstArg<'_>`
   --> src/librustdoc/clean/mod.rs:605:78
    |
605 |                     .map(|ct| Box::new(ty::Const::from_anon_const(cx.tcx, ct.def_id).to_string())),
    |
    = note: available fields are: `kind`

Some errors have detailed explanations: E0433, E0609.
