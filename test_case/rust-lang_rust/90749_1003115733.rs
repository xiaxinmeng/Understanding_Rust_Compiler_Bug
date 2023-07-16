plain
    Checking tempfile v3.2.0
    Checking synstructure v0.12.6
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling tracing-attributes v0.1.17
   Compiling chalk-derive v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking chalk-ir v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking tracing-subscriber v0.3.3
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking tracing-tree v0.2.0
    Checking tracing-tree v0.2.0
    Checking chalk-solve v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
    Checking chalk-engine v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
---
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0433]: failed to resolve: use of undeclared type `Res`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1948:13
     |
1948 |             Res::Def(DefKind::Trait | DefKind::TraitAlias, def_id) => {
     |             ^^^ use of undeclared type `Res`
error[E0433]: failed to resolve: use of undeclared type `DefKind`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1948:22
     |
     |
1948 |             Res::Def(DefKind::Trait | DefKind::TraitAlias, def_id) => {
     |                      ^^^^^^^ use of undeclared type `DefKind`
error[E0433]: failed to resolve: use of undeclared type `DefKind`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1948:39
     |
     |
1948 |             Res::Def(DefKind::Trait | DefKind::TraitAlias, def_id) => {
     |                                       ^^^^^^^ use of undeclared type `DefKind`
error[E0433]: failed to resolve: use of undeclared type `Res`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1951:13
     |
     |
1951 |             Res::Def(DefKind::Mod, def_id) => {
     |             ^^^ use of undeclared type `Res`
error[E0433]: failed to resolve: use of undeclared type `DefKind`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1951:22
     |
     |
1951 |             Res::Def(DefKind::Mod, def_id) => {
     |                      ^^^^^^^ use of undeclared type `DefKind`
error[E0433]: failed to resolve: use of undeclared type `DefKind`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1964:76
     |
     |
1964 |         handle_external_res(tcx, &mut traits, &mut external_mods, Res::Def(DefKind::Mod, def_id));
     |                                                                            ^^^^^^^ use of undeclared type `DefKind`
error[E0412]: cannot find type `Res` in this scope
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1945:14
     |
1945 |         res: Res<!>,
---

error[E0425]: cannot find value `CRATE_DEF_INDEX` in this scope
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1963:50
     |
1963 |         let def_id = DefId { krate: cnum, index: CRATE_DEF_INDEX };
     |
help: consider importing one of these items
     |
     |
4    | use crate::check::method::suggest::source_map::def_id::CRATE_DEF_INDEX;
4    | use rustc_hir::def_id::CRATE_DEF_INDEX;
     |
4    | use rustc_span::def_id::CRATE_DEF_INDEX;
     |
     |

error[E0433]: failed to resolve: use of undeclared type `Res`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1964:67
     |
1964 |         handle_external_res(tcx, &mut traits, &mut external_mods, Res::Def(DefKind::Mod, def_id));
     |
help: consider importing one of these items
     |
4    | use crate::check::Res;
4    | use crate::check::Res;
     |
4    | use rustc_hir::def::Res;
     |

error[E0061]: this function takes 0 arguments but 1 argument was supplied
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1904:9
     |
1904 |     tcx.all_traits(()).iter().map(|&def_id| TraitInfo { def_id }).collect()
     |         |
     |         expected 0 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/compiler/rustc_middle/src/ty/context.rs:1581:12
     |
1581 |     pub fn all_traits(self) -> impl Iterator<Item = DefId> + 'tcx {


error[E0599]: no method named `iter` found for opaque type `impl Iterator<Item = DefId>` in the current scope
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1904:24
     |
1904 |     tcx.all_traits(()).iter().map(|&def_id| TraitInfo { def_id }).collect()
     |                        ^^^^ method not found in `impl Iterator<Item = DefId>`
error[E0609]: no field `all_traits` on type `&mut Providers`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1971:15
     |
1971 |     providers.all_traits = compute_all_traits;
1971 |     providers.all_traits = compute_all_traits;
     |               ^^^^^^^^^^ unknown field
     |
     = note: available fields are: `trigger_delay_span_bug`, `resolutions`, `source_span`, `hir_crate`, `hir_module_items` ... and 259 others
Some errors have detailed explanations: E0061, E0412, E0425, E0433, E0599, E0609.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_typeck` due to 12 previous errors
warning: build failed, waiting for other jobs to finish...
