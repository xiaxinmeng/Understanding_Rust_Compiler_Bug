plain
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling chalk-engine v0.36.0
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error: cannot find macro `CloneTypeFoldableAndLiftImpls` in this scope
  --> compiler/rustc_middle/src/mir/graph_cyclic_cache.rs:60:1
   |
60 | CloneTypeFoldableAndLiftImpls! {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `TrivialTypeFoldableAndLiftImpls`
  ::: compiler/rustc_middle/src/macros.rs:83:1
   |
   |
83 | macro_rules! TrivialTypeFoldableAndLiftImpls {
   | -------------------------------------------- similarly named macro `TrivialTypeFoldableAndLiftImpls` defined here
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `GraphIsCyclicCache: ty::fold::TypeFoldable<'_>` is not satisfied
    |
    |
148 |   #[derive(Clone, TyEncodable, TyDecodable, Debug, HashStable, TypeFoldable)]
    |                                                                |
    |                                                                |
    |                                                                the trait `ty::fold::TypeFoldable<'_>` is not implemented for `GraphIsCyclicCache`
    | 
   ::: compiler/rustc_middle/src/ty/fold.rs:48:5
    |
    |
48  |       fn fold_with<F: TypeFolder<'tcx>>(self, folder: &mut F) -> Self {
    |       --------------------------------------------------------------- required by `ty::fold::TypeFoldable::fold_with`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(TypeFoldable)]`

error[E0277]: the trait bound `GraphIsCyclicCache: ty::fold::TypeFoldable<'_>` is not satisfied
    |
    |
148 |   #[derive(Clone, TyEncodable, TyDecodable, Debug, HashStable, TypeFoldable)]
    |                                                                |
    |                                                                |
    |                                                                the trait `ty::fold::TypeFoldable<'_>` is not implemented for `GraphIsCyclicCache`
    | 
   ::: compiler/rustc_middle/src/ty/fold.rs:53:5
    |
    |
53  |       fn visit_with<V: TypeVisitor<'tcx>>(&self, visitor: &mut V) -> ControlFlow<V::BreakTy> {
    |       -------------------------------------------------------------------------------------- required by `ty::fold::TypeFoldable::visit_with`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(TypeFoldable)]`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle`
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest
Build completed unsuccessfully in 0:09:42
