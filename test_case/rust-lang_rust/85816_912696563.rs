plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking chalk-solve v0.55.0
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error: no rules expected the token `<<`
   --> compiler/rustc_hir/src/lang_items.rs:174:1
39  | macro_rules! language_item_table {
    | -------------------------------- when calling this macro
...
...
174 | <<<<<<< HEAD
    | ^^ no rules expected this token in macro call

error[E0432]: unresolved import `crate::LangItem`
 --> compiler/rustc_hir/src/hir.rs:4:28
  |
4 | use crate::{itemlikevisit, LangItem};
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
   --> compiler/rustc_hir/src/lang_items.rs:145:31
    |
145 | impl<CTX> HashStable<CTX> for LangItem {
    |
    |
help: there is an enum variant `crate::QPath::LangItem`; try using the variant's enum
    |
145 | impl<CTX> HashStable<CTX> for crate::QPath {

error: unused `#[macro_use]` import
  --> compiler/rustc_hir/src/lib.rs:14:1
   |
   |
14 | #[macro_use]
   | ^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `crate::def_id::DefId`
  --> compiler/rustc_hir/src/lang_items.rs:10:5
   |
10 | use crate::def_id::DefId;
10 | use crate::def_id::DefId;
   |     ^^^^^^^^^^^^^^^^^^^^

error: unused imports: `MethodKind`, `Target`
  --> compiler/rustc_hir/src/lang_items.rs:11:13
   |
11 | use crate::{MethodKind, Target};


error: unused import: `rustc_data_structures::fx::FxHashMap`
  --> compiler/rustc_hir/src/lang_items.rs:14:5
   |
14 | use rustc_data_structures::fx::FxHashMap;

error: unused import: `rustc_macros::HashStable_Generic`
  --> compiler/rustc_hir/src/lang_items.rs:16:5
   |
   |
16 | use rustc_macros::HashStable_Generic;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `64_usize`, found `::std::mem::size_of::<$ty>()`
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_hir/src/hir.rs:3292:5
    ::: compiler/rustc_hir/src/hir.rs:3292:5
     |
3292 |       rustc_data_structures::static_assert_size!(super::Expr<'static>, 64);
     |
     |
     = note: expected array `[(); 64]`
                found array `[(); _]`
error: constant expression depends on a generic parameter
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:37
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
    ::: compiler/rustc_hir/src/hir.rs:3292:5
     |
3292 |       rustc_data_structures::static_assert_size!(super::Expr<'static>, 64);
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
    ::: compiler/rustc_hir/src/hir.rs:3293:5
    ::: compiler/rustc_hir/src/hir.rs:3293:5
     |
3293 |       rustc_data_structures::static_assert_size!(super::Pat<'static>, 88);
     |
     |
     = note: expected array `[(); 88]`
                found array `[(); _]`
error: constant expression depends on a generic parameter
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:37
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
    ::: compiler/rustc_hir/src/hir.rs:3293:5
     |
3293 |       rustc_data_structures::static_assert_size!(super::Pat<'static>, 88);
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
    ::: compiler/rustc_hir/src/hir.rs:3294:5
    ::: compiler/rustc_hir/src/hir.rs:3294:5
     |
3294 |       rustc_data_structures::static_assert_size!(super::QPath<'static>, 24);
     |
     |
     = note: expected array `[(); 24]`
                found array `[(); _]`
error: constant expression depends on a generic parameter
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:37
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
    ::: compiler/rustc_hir/src/hir.rs:3294:5
     |
3294 |       rustc_data_structures::static_assert_size!(super::QPath<'static>, 24);
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
    ::: compiler/rustc_hir/src/hir.rs:3295:5
    ::: compiler/rustc_hir/src/hir.rs:3295:5
     |
3295 |       rustc_data_structures::static_assert_size!(super::Ty<'static>, 72);
     |
     |
     = note: expected array `[(); 72]`
                found array `[(); _]`
error: constant expression depends on a generic parameter
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:37
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
    ::: compiler/rustc_hir/src/hir.rs:3295:5
     |
3295 |       rustc_data_structures::static_assert_size!(super::Ty<'static>, 72);
     |
     = note: this may fail depending on what value the parameter takes

    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
