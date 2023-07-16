plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking tracing-tree v0.1.6
error: no rules expected the token `#`
   --> compiler/rustc_hir/src/lang_items.rs:49:21
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
49  | |                     #[doc = concat!("The `", stringify!($name), "` lang item.")]
    | |                     ^ no rules expected this token in macro call
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation


error[E0432]: unresolved import `crate::LangItem`
 --> compiler/rustc_hir/src/hir.rs:5:28
  |
5 | use crate::{itemlikevisit, LangItem};
  |                            ^^^^^^^^ no `LangItem` in the root

error[E0432]: unresolved import `crate::LangItem`
 --> compiler/rustc_hir/src/weak_lang_items.rs:4:25
  |
4 | use crate::{lang_items, LangItem, LanguageItems};
  |                         ^^^^^^^^ no `LangItem` in the root

error[E0432]: unresolved import `lang_items::LangItem`
  --> compiler/rustc_hir/src/lib.rs:35:22
   |
35 | pub use lang_items::{LangItem, LanguageItems};
   |                      ^^^^^^^^ no `LangItem` in `lang_items`

error[E0433]: failed to resolve: use of undeclared type `LangItem`
   --> compiler/rustc_hir/src/lang_items.rs:61:24
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
61  | |                     $( LangItem::$variant => $name, )*
    | |                        ^^^^^^^^ use of undeclared type `LangItem`
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation


error[E0433]: failed to resolve: use of undeclared type `LangItem`
   --> compiler/rustc_hir/src/lang_items.rs:70:24
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
70  | |                     $( LangItem::$variant => expand_group!($($group)*), )*
    | |                        ^^^^^^^^ use of undeclared type `LangItem`
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation


error[E0433]: failed to resolve: use of undeclared type `LangItem`
   --> compiler/rustc_hir/src/lang_items.rs:94:45
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
94  | |                     items: vec![$(init_none(LangItem::$variant)),*],
    | |                                             ^^^^^^^^ use of undeclared type `LangItem`
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation


error[E0433]: failed to resolve: use of undeclared type `LangItem`
   --> compiler/rustc_hir/src/lang_items.rs:121:32
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
121 | |                     self.items[LangItem::$variant as usize]
    | |                                ^^^^^^^^ use of undeclared type `LangItem`
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation


error[E0433]: failed to resolve: use of undeclared type `LangItem`
   --> compiler/rustc_hir/src/lang_items.rs:129:41
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
129 | |             $( item_refs.insert($name, (LangItem::$variant as usize, $target)); )*
    | |                                         ^^^^^^^^ use of undeclared type `LangItem`
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation


error[E0412]: cannot find type `LangItem` in this scope
   --> compiler/rustc_hir/src/lang_items.rs:137:31
    |
137 | impl<CTX> HashStable<CTX> for LangItem {
    |
    |
help: there is an enum variant `crate::QPath::LangItem`; try using the variant's enum
    |
137 | impl<CTX> HashStable<CTX> for crate::QPath {


error[E0412]: cannot find type `LangItem` in this scope
   --> compiler/rustc_hir/src/lang_items.rs:55:14
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
55  | |         impl LangItem {
...   |
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation
    |
    |
help: there is an enum variant `crate::QPath::LangItem`; try using the variant's enum
    |
55  |         impl crate::QPath {


error[E0412]: cannot find type `LangItem` in this scope
   --> compiler/rustc_hir/src/lang_items.rs:82:30
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
82  | |             pub missing: Vec<LangItem>,
...   |
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation
    |
    |
help: there is an enum variant `crate::QPath::LangItem`; try using the variant's enum
    |
82  |             pub missing: Vec<crate::QPath>,


error[E0412]: cannot find type `LangItem` in this scope
   --> compiler/rustc_hir/src/lang_items.rs:91:33
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
91  | |                 fn init_none(_: LangItem) -> Option<DefId> { None }
...   |
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation
    |
    |
help: there is an enum variant `crate::QPath::LangItem`; try using the variant's enum
    |
91  |                 fn init_none(_: crate::QPath) -> Option<DefId> { None }


error[E0412]: cannot find type `LangItem` in this scope
   --> compiler/rustc_hir/src/lang_items.rs:108:39
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
108 | |             pub fn require(&self, it: LangItem) -> Result<DefId, String> {
...   |
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation
    |
    |
help: there is an enum variant `crate::QPath::LangItem`; try using the variant's enum
    |
108 |             pub fn require(&self, it: crate::QPath) -> Result<DefId, String> {


error[E0658]: arbitrary expressions in key-value attributes are unstable
   --> compiler/rustc_hir/src/lang_items.rs:118:25
39  | / macro_rules! language_item_table {
40  | |     (
40  | |     (
41  | |         $( $variant:ident $($group:expr)?, $name:expr, $method:ident, $target:expr; )*
42  | |     ) => {
...   |
118 | |                 #[doc = concat!("Returns the [`DefId`] of the `", stringify!($name), "` lang item if it is defined.")]
...   |
134 | |     }
135 | | }
135 | | }
    | |_- in this expansion of `language_item_table!`
165 | / language_item_table! {
165 | / language_item_table! {
166 | | //  Variant name,            Name,                     Method name,                Target;
167 | |     Bool,                    sym::bool,                bool_impl,                  Target::Impl;
168 | |     Char,                    sym::char,                char_impl,                  Target::Impl;
...   |
343 | |     RangeTo,                 sym::RangeTo,             range_to_struct,            Target::Struct;
    | |_- in this macro invocation
    |
    = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
    = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable
    = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable

    Checking chalk-solve v0.55.0
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
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_trait_selection" "-p" "rustc_macros" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_ast" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_feature" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_symbol_mangling" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_ast_passes" "-p" "rustc_privacy" "-p" "rustc_ast_lowering" "-p" "rustc_resolve" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_passes" "-p" "rustc_typeck" "-p" "rustc_traits" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_builtin_macros" "-p" "rustc_hir_pretty" "-p" "rustc_metadata" "-p" "rustc_serialize" "-p" "rustc_data_structures" "-p" "rustc_session" "-p" "rustc_hir" "-p" "rustc_error_codes" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_errors" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:52
