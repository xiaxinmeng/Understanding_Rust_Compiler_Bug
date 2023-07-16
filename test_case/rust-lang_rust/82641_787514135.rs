plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error: no rules expected the token `doc`
   --> compiler/rustc_hir/src/lang_items.rs:201:5
39  | macro_rules! language_item_table {
    | -------------------------------- when calling this macro
...
...
201 |     /// Trait injected by `#[derive(PartialEq)]`, (i.e. "Partial EQ").
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no rules expected this token in macro call

error[E0432]: unresolved import `crate::LangItem`
 --> compiler/rustc_hir/src/hir.rs:5:28
  |
5 | use crate::{itemlikevisit, LangItem};
  |                            ^^^^^^^^ no `LangItem` in the root

error[E0432]: unresolved imports `crate::LangItem`, `crate::LanguageItems`
 --> compiler/rustc_hir/src/weak_lang_items.rs:4:25
  |
4 | use crate::{lang_items, LangItem, LanguageItems};
  |                         ^^^^^^^^  ^^^^^^^^^^^^^ no `LanguageItems` in the root
  |                         |
  |                         no `LangItem` in the root

error[E0432]: unresolved imports `lang_items::LangItem`, `lang_items::LanguageItems`
  --> compiler/rustc_hir/src/lib.rs:36:22
   |
36 | pub use lang_items::{LangItem, LanguageItems};
   |                      ^^^^^^^^  ^^^^^^^^^^^^^ no `LanguageItems` in `lang_items`
   |                      |
   |                      no `LangItem` in `lang_items`

error[E0412]: cannot find type `LangItem` in this scope
   --> compiler/rustc_hir/src/lang_items.rs:139:31
    |
139 | impl<CTX> HashStable<CTX> for LangItem {
    |
    |
help: there is an enum variant `crate::QPath::LangItem`; try using the variant's enum
    |
139 | impl<CTX> HashStable<CTX> for crate::QPath {

error: unused import: `crate::def_id::DefId`
  --> compiler/rustc_hir/src/lang_items.rs:10:5
   |
   |
10 | use crate::def_id::DefId;
   |     ^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `MethodKind`, `Target`
  --> compiler/rustc_hir/src/lang_items.rs:11:13
   |
11 | use crate::{MethodKind, Target};
   |             ^^^^^^^^^^  ^^^^^^

error: unused import: `rustc_data_structures::fx::FxHashMap`
  --> compiler/rustc_hir/src/lang_items.rs:14:5
   |
14 | use rustc_data_structures::fx::FxHashMap;


error: unused import: `rustc_macros::HashStable_Generic`
  --> compiler/rustc_hir/src/lang_items.rs:16:5
16 | use rustc_macros::HashStable_Generic;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `kw`
error: unused import: `kw`
  --> compiler/rustc_hir/src/lang_items.rs:17:26
   |
17 | use rustc_span::symbol::{kw, sym, Symbol};

error: unused import: `std::lazy::SyncLazy`
  --> compiler/rustc_hir/src/lang_items.rs:20:5
   |
---
34 | |     };
35 | | }
   | |_^
   |
   = note: `-D unused-macros` implied by `-D warnings`
    Checking tracing-tree v0.1.6
    Checking chalk-solve v0.55.0
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `72_usize`, found `::std::mem::size_of::<$ty>()`
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_hir/src/hir.rs:3094:5
    ::: compiler/rustc_hir/src/hir.rs:3094:5
     |
3094 |       rustc_data_structures::static_assert_size!(super::Expr<'static>, 72);
     |
     |
     = note: expected array `[(); 72]`
                found array `[(); _]`

error: constant expression depends on a generic parameter
     |
3    | / macro_rules! static_assert_size {
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
6    | |     };
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
     | 
    ::: compiler/rustc_hir/src/hir.rs:3094:5
     |
3094 |       rustc_data_structures::static_assert_size!(super::Expr<'static>, 72);
     |
     = note: this may fail depending on what value the parameter takes

error[E0308]: mismatched types
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `88_usize`, found `::std::mem::size_of::<$ty>()`
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_hir/src/hir.rs:3095:5
    ::: compiler/rustc_hir/src/hir.rs:3095:5
     |
3095 |       rustc_data_structures::static_assert_size!(super::Pat<'static>, 88);
     |
     |
     = note: expected array `[(); 88]`
                found array `[(); _]`

error: constant expression depends on a generic parameter
     |
3    | / macro_rules! static_assert_size {
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
6    | |     };
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
     | 
    ::: compiler/rustc_hir/src/hir.rs:3095:5
     |
3095 |       rustc_data_structures::static_assert_size!(super::Pat<'static>, 88);
     |
     = note: this may fail depending on what value the parameter takes

error[E0308]: mismatched types
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `24_usize`, found `::std::mem::size_of::<$ty>()`
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_hir/src/hir.rs:3096:5
    ::: compiler/rustc_hir/src/hir.rs:3096:5
     |
3096 |       rustc_data_structures::static_assert_size!(super::QPath<'static>, 24);
     |
     |
     = note: expected array `[(); 24]`
                found array `[(); _]`

error: constant expression depends on a generic parameter
     |
3    | / macro_rules! static_assert_size {
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
6    | |     };
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
     | 
    ::: compiler/rustc_hir/src/hir.rs:3096:5
     |
3096 |       rustc_data_structures::static_assert_size!(super::QPath<'static>, 24);
     |
     = note: this may fail depending on what value the parameter takes

error[E0308]: mismatched types
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `72_usize`, found `::std::mem::size_of::<$ty>()`
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_hir/src/hir.rs:3097:5
    ::: compiler/rustc_hir/src/hir.rs:3097:5
     |
3097 |       rustc_data_structures::static_assert_size!(super::Ty<'static>, 72);
     |
     |
     = note: expected array `[(); 72]`
                found array `[(); _]`

error: constant expression depends on a generic parameter
     |
3    | / macro_rules! static_assert_size {
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
6    | |     };
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
     | 
    ::: compiler/rustc_hir/src/hir.rs:3097:5
     |
3097 |       rustc_data_structures::static_assert_size!(super::Ty<'static>, 72);
     |
     = note: this may fail depending on what value the parameter takes

    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_target" "-p" "rustc_metadata" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ast_passes" "-p" "rustc_errors" "-p" "rustc_serialize" "-p" "rustc_save_analysis" "-p" "rustc_feature" "-p" "rustc_parse" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_parse_format" "-p" "rustc_ast" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_interface" "-p" "rustc_typeck" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_builtin_macros" "-p" "rustc_incremental" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "-p" "rustc_ast_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_data_structures" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:53
