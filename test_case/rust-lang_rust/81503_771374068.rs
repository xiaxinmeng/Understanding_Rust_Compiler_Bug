plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error: unknown start of token: `
   --> compiler/rustc_middle/src/traits/mod.rs:238:53
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const` item if the `fn` in the array repeat expression is a `const fn`)
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a 'const` item if the `fn` in the array repeat expression is a `const fn`)

error: unknown start of token: `
   --> compiler/rustc_middle/src/traits/mod.rs:238:59
    |
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const` item if the `fn` in the array repeat expression is a `const fn`)
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const' item if the `fn` in the array repeat expression is a `const fn`)

error: unknown start of token: `
   --> compiler/rustc_middle/src/traits/mod.rs:238:73
    |
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const` item if the `fn` in the array repeat expression is a `const fn`)
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const` item if the 'fn` in the array repeat expression is a `const fn`)

error: unknown start of token: `
   --> compiler/rustc_middle/src/traits/mod.rs:238:76
    |
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const` item if the `fn` in the array repeat expression is a `const fn`)
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const` item if the `fn' in the array repeat expression is a `const fn`)

error: unknown start of token: `
   --> compiler/rustc_middle/src/traits/mod.rs:238:114
    |
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const` item if the `fn` in the array repeat expression is a `const fn`)
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const` item if the `fn` in the array repeat expression is a 'const fn`)

error: unknown start of token: `
   --> compiler/rustc_middle/src/traits/mod.rs:238:123
    |
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const` item if the `fn` in the array repeat expression is a `const fn`)
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
238 | >>>>>>> 0c81f0f5814 (added a suggestion to create a `const` item if the `fn` in the array repeat expression is a `const fn')


error[E0585]: found a documentation comment that doesn't document anything
    |
    |
230 |     /// `[T, ..n]` implies that `T` must be `Copy`.
    |
    |
    = help: doc comments must come before what they document, maybe a comment was intended with `//`?
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0432]: unresolved import `crate::traits::query`
 --> compiler/rustc_middle/src/query/mod.rs:4:20
4 | use crate::traits::query::{
4 | use crate::traits::query::{
  |                    ^^^^^ could not find `query` in `traits`
error[E0432]: unresolved import `crate::traits::Reveal`
  --> compiler/rustc_middle/src/ty/mod.rs:29:27
   |
   |
29 | use crate::traits::{self, Reveal};
   |                           ^^^^^^ no `Reveal` in `traits`

error[E0432]: unresolved imports `crate::traits::ObligationCause`, `crate::traits::ObligationCauseCode`
 --> compiler/rustc_middle/src/ty/error.rs:1:21
  |
1 | use crate::traits::{ObligationCause, ObligationCauseCode};
  |                     ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^ no `ObligationCauseCode` in `traits`
  |                     |
  |                     no `ObligationCause` in `traits`

error[E0432]: unresolved import `crate::traits::query`
  --> compiler/rustc_middle/src/ty/query/mod.rs:20:20
20 | use crate::traits::query::{
20 | use crate::traits::query::{
   |                    ^^^^^ could not find `query` in `traits`

error[E0432]: unresolved import `crate::traits::query`
  --> compiler/rustc_middle/src/ty/query/mod.rs:25:20
25 | use crate::traits::query::{
25 | use crate::traits::query::{
   |                    ^^^^^ could not find `query` in `traits`
error[E0432]: unresolved import `crate::traits::specialization_graph`
error[E0432]: unresolved import `crate::traits::specialization_graph`
  --> compiler/rustc_middle/src/ty/query/mod.rs:29:5
29 | use crate::traits::specialization_graph;
29 | use crate::traits::specialization_graph;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `specialization_graph` in `traits`
error[E0432]: unresolved import `crate::traits::ImplSource`
error[E0432]: unresolved import `crate::traits::ImplSource`
  --> compiler/rustc_middle/src/ty/query/mod.rs:30:27
   |
30 | use crate::traits::{self, ImplSource};
   |                           ^^^^^^^^^^ no `ImplSource` in `traits`
error[E0432]: unresolved import `crate::traits::specialization_graph`
 --> compiler/rustc_middle/src/ty/trait_def.rs:2:5
  |
2 | use crate::traits::specialization_graph;
2 | use crate::traits::specialization_graph;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `specialization_graph` in `traits`

error[E0433]: failed to resolve: could not find `query` in `traits`
   --> compiler/rustc_middle/src/arena.rs:43:47
    |
11  | / macro_rules! arena_types {
12  | |     ($macro:path, $args:tt, $tcx:lifetime) => (
13  | |         $macro!($args, [
14  | |             [] layouts: rustc_target::abi::Layout,
...   |
43  | |                         rustc_middle::traits::query::DropckOutlivesResult<'tcx>
    | |                                               ^^^^^ could not find `query` in `traits`
108 | |     )
109 | | }
109 | | }
    | |_- in this expansion of `arena_types!`
110 | 
111 |   arena_types!(rustc_arena::declare_arena, [], 'tcx);
    |   --------------------------------------------------- in this macro invocation

error[E0433]: failed to resolve: could not find `query` in `traits`
   --> compiler/rustc_middle/src/arena.rs:49:47
    |
11  | / macro_rules! arena_types {
12  | |     ($macro:path, $args:tt, $tcx:lifetime) => (
13  | |         $macro!($args, [
14  | |             [] layouts: rustc_target::abi::Layout,
49  | |                         rustc_middle::traits::query::NormalizationResult<'tcx>
49  | |                         rustc_middle::traits::query::NormalizationResult<'tcx>
    | |                                               ^^^^^ could not find `query` in `traits`
108 | |     )
109 | | }
109 | | }
    | |_- in this expansion of `arena_types!`
110 | 
111 |   arena_types!(rustc_arena::declare_arena, [], 'tcx);
    |   --------------------------------------------------- in this macro invocation

error[E0433]: failed to resolve: could not find `query` in `traits`
   --> compiler/rustc_middle/src/arena.rs:55:51
    |
11  | / macro_rules! arena_types {
12  | |     ($macro:path, $args:tt, $tcx:lifetime) => (
13  | |         $macro!($args, [
14  | |             [] layouts: rustc_target::abi::Layout,
...   |
55  | |                         Vec<rustc_middle::traits::query::OutlivesBound<'tcx>>
    | |                                                   ^^^^^ could not find `query` in `traits`
108 | |     )
109 | | }
109 | | }
    | |_- in this expansion of `arena_types!`
110 | 
111 |   arena_types!(rustc_arena::declare_arena, [], 'tcx);
    |   --------------------------------------------------- in this macro invocation
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0433]: failed to resolve: could not find `Reveal` in `traits`
    --> compiler/rustc_middle/src/ty/mod.rs:1731:21
     |
1731 |             traits::Reveal::UserFacing => 0,
     |                     ^^^^^^ could not find `Reveal` in `traits`

error[E0433]: failed to resolve: could not find `Reveal` in `traits`
    --> compiler/rustc_middle/src/ty/mod.rs:1732:21
     |
1732 |             traits::Reveal::All => 1,
     |                     ^^^^^^ could not find `Reveal` in `traits`

error[E0433]: failed to resolve: could not find `Reveal` in `traits`
    --> compiler/rustc_middle/src/ty/mod.rs:1737:26
     |
1737 |             0 => traits::Reveal::UserFacing,
     |                          ^^^^^^ could not find `Reveal` in `traits`

error[E0433]: failed to resolve: could not find `Reveal` in `traits`
    --> compiler/rustc_middle/src/ty/mod.rs:1738:26
     |
1738 |             1 => traits::Reveal::All,
     |                          ^^^^^^ could not find `Reveal` in `traits`

error[E0433]: failed to resolve: could not find `Reveal` in `traits`
    --> compiler/rustc_middle/src/ty/mod.rs:1824:41
     |
1824 |         if self.packed.tag() == traits::Reveal::All {
     |                                         ^^^^^^ could not find `Reveal` in `traits`

error[E0412]: cannot find type `CanonicalChalkEnvironmentAndGoal` in module `traits`
    --> compiler/rustc_middle/src/query/mod.rs:1564:27
     |
1564 |             goal: traits::CanonicalChalkEnvironmentAndGoal<'tcx>
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `traits`

error[E0412]: cannot find type `ObjectSafetyViolation` in module `rustc_middle::traits`
   --> compiler/rustc_middle/src/arena.rs:83:64
    |
11  | / macro_rules! arena_types {
12  | |     ($macro:path, $args:tt, $tcx:lifetime) => (
13  | |         $macro!($args, [
14  | |             [] layouts: rustc_target::abi::Layout,
...   |
83  | |             [] object_safety_violations: rustc_middle::traits::ObjectSafetyViolation,
...   |
108 | |     )
109 | | }
109 | | }
    | |_- in this expansion of `arena_types!`
110 | 
111 |   arena_types!(rustc_arena::declare_arena, [], 'tcx);
    |   --------------------------------------------------- in this macro invocation

error[E0412]: cannot find type `CanonicalChalkEnvironmentAndGoal` in module `traits`
    --> compiler/rustc_middle/src/query/mod.rs:1564:27
     |
37   | / rustc_queries! {
38   | |     Other {
39   | |         query trigger_delay_span_bug(key: DefId) -> () {
40   | |             desc { "trigger a delay span bug" }
...    |
1564 | |             goal: traits::CanonicalChalkEnvironmentAndGoal<'tcx>
     | |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `traits`
1717 | |     }
1718 | | }
1718 | | }
     | |_- in this expansion of `rustc_query_append!`
     | 
    ::: compiler/rustc_middle/src/ty/query/mod.rs:104:1
     |
104  |   rustc_query_append! { [define_queries!][<'tcx>] }
     |   ------------------------------------------------- in this macro invocation

error[E0412]: cannot find type `ObjectSafetyViolation` in module `traits`
    --> compiler/rustc_middle/src/query/mod.rs:1017:75
     |
37   | / rustc_queries! {
38   | |     Other {
39   | |         query trigger_delay_span_bug(key: DefId) -> () {
40   | |             desc { "trigger a delay span bug" }
...    |
1017 | |         query object_safety_violations(trait_id: DefId) -> &'tcx [traits::ObjectSafetyViolation] {
     | |                                                                           ^^^^^^^^^^^^^^^^^^^^^ not found in `traits`
1717 | |     }
1718 | | }
1718 | | }
     | |_- in this expansion of `rustc_query_append!`
     | 
    ::: compiler/rustc_middle/src/ty/query/mod.rs:104:1
     |
104  |   rustc_query_append! { [define_queries!][<'tcx>] }
     |   ------------------------------------------------- in this macro invocation

error[E0412]: cannot find type `EvaluationResult` in module `traits`
    --> compiler/rustc_middle/src/query/mod.rs:1559:29
     |
37   | / rustc_queries! {
38   | |     Other {
39   | |         query trigger_delay_span_bug(key: DefId) -> () {
40   | |             desc { "trigger a delay span bug" }
1559 | |         ) -> Result<traits::EvaluationResult, traits::OverflowError> {
     | |                             ^^^^^^^^^^^^^^^^ not found in `traits`
...    |
1717 | |     }
1717 | |     }
1718 | | }
     | |_- in this expansion of `rustc_query_append!`
     | 
    ::: compiler/rustc_middle/src/ty/query/mod.rs:104:1
     |
104  |   rustc_query_append! { [define_queries!][<'tcx>] }
     |   ------------------------------------------------- in this macro invocation

error[E0412]: cannot find type `OverflowError` in module `traits`
    --> compiler/rustc_middle/src/query/mod.rs:1559:55
     |
37   | / rustc_queries! {
38   | |     Other {
39   | |         query trigger_delay_span_bug(key: DefId) -> () {
40   | |             desc { "trigger a delay span bug" }
1559 | |         ) -> Result<traits::EvaluationResult, traits::OverflowError> {
     | |                                                       ^^^^^^^^^^^^^ not found in `traits`
...    |
1717 | |     }
1717 | |     }
1718 | | }
     | |_- in this expansion of `rustc_query_append!`
     | 
    ::: compiler/rustc_middle/src/ty/query/mod.rs:104:1
     |
104  |   rustc_query_append! { [define_queries!][<'tcx>] }
     |   ------------------------------------------------- in this macro invocation

error[E0412]: cannot find type `SelectionCache` in module `traits`
   --> compiler/rustc_middle/src/ty/context.rs:982:34
    |
982 |     pub selection_cache: traits::SelectionCache<'tcx>,
    |                                  ^^^^^^^^^^^^^^ not found in `traits`

error[E0412]: cannot find type `EvaluationCache` in module `traits`
   --> compiler/rustc_middle/src/ty/context.rs:987:35
    |
987 |     pub evaluation_cache: traits::EvaluationCache<'tcx>,
    |                                   ^^^^^^^^^^^^^^^ not found in `traits`

error[E0412]: cannot find type `Reveal` in module `crate::traits`
   --> compiler/rustc_middle/src/ty/structural_impls.rs:242:20
242 |     crate::traits::Reveal,
    |                    ^^^^^^ not found in `crate::traits`
    |
    |
help: there is an enum variant `chalk_ir::DomainGoal::Reveal`; try using the variant's enum
    |
242 |     chalk_ir::DomainGoal,


error[E0412]: cannot find type `Reveal` in module `traits`
    --> compiler/rustc_middle/src/ty/mod.rs:1724:64
     |
1724 |     packed: CopyTaggedPtr<&'tcx List<Predicate<'tcx>>, traits::Reveal, true>,
     |                                                                ^^^^^^ not found in `traits`
     |
help: there is an enum variant `chalk_ir::DomainGoal::Reveal`; try using the variant's enum
     |
1724 |     packed: CopyTaggedPtr<&'tcx List<Predicate<'tcx>>, chalk_ir::DomainGoal, true>,


error[E0412]: cannot find type `Reveal` in module `traits`
    --> compiler/rustc_middle/src/ty/mod.rs:1727:64
     |
1727 | unsafe impl rustc_data_structures::tagged_ptr::Tag for traits::Reveal {
     |                                                                ^^^^^^ not found in `traits`
     |
help: there is an enum variant `chalk_ir::DomainGoal::Reveal`; try using the variant's enum
     |
1727 | unsafe impl rustc_data_structures::tagged_ptr::Tag for chalk_ir::DomainGoal {


error[E0412]: cannot find type `Reveal` in module `traits`
    --> compiler/rustc_middle/src/ty/mod.rs:1787:36
     |
1787 |     pub fn reveal(self) -> traits::Reveal {
     |                                    ^^^^^^ not found in `traits`
     |
help: there is an enum variant `chalk_ir::DomainGoal::Reveal`; try using the variant's enum
     |
1787 |     pub fn reveal(self) -> chalk_ir::DomainGoal {

error[E0283]: type annotations needed
error[E0283]: type annotations needed
    --> compiler/rustc_middle/src/ty/query/plumbing.rs:536:30
     |
250  |  /   macro_rules! define_queries {
251  |  |       (<$tcx:tt>
252  |  |        $($(#[$attr:meta])*
253  |  |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...     |
262  | /|           define_queries_struct! {
263  | ||               tcx: $tcx,
264  | ||               input: ($(([$($modifiers)*] [$($attr)*] [$name]))*)
265  | ||           }
     | ||___________- in this macro invocation (#3)
500  |  |       }
501  |  |   }
501  |  |   }
     |  |___- in this expansion of `define_queries!` (#2)
...
506  | /    macro_rules! define_queries_struct {
507  | |        (tcx: $tcx:tt,
508  | |         input: ($(([$($modifiers:tt)*] [$($attr:tt)*] [$name:ident]))*)) => {
509  | |            pub struct Queries<$tcx> {
...    |
536  | |                        $($name: Default::default()),*
...    |
556  | |        };
557  | |    }
557  | |    }
     | |____- in this expansion of `define_queries_struct!` (#3)
     | 
    ::: compiler/rustc_middle/src/ty/query/mod.rs:104:1
     |
104  |      rustc_query_append! { [define_queries!][<'tcx>] }
     |      ------------------------------------------------- in this macro invocation (#1)
     | 
    ::: compiler/rustc_middle/src/query/mod.rs:37:1
     |
37   |    / rustc_queries! {
38   |    |     Other {
39   |    |         query trigger_delay_span_bug(key: DefId) -> () {
40   |    |             desc { "trigger a delay span bug" }
1717 |    |     }
1718 |    | }
     |    | -
     |    | |
     |    | |
     |    |_in this expansion of `rustc_query_append!` (#1)
     |      in this macro invocation (#2)
     |
     = note: cannot satisfy `_: std::default::Default`
     = note: required by `std::default::Default::default`
error[E0283]: type annotations needed
    --> compiler/rustc_middle/src/ty/context.rs:1159:30
     |
     |
1159 |             selection_cache: Default::default(),
     |
     |
     = note: cannot satisfy `_: std::default::Default`
     = note: required by `std::default::Default::default`
error[E0283]: type annotations needed
   --> /checkout/compiler/rustc_arena/src/lib.rs:662:15
    |
656 | /  macro_rules! declare_arena {
656 | /  macro_rules! declare_arena {
657 | |      ([], [$($a:tt $name:ident: $ty:ty,)*], $tcx:lifetime) => {
658 | |          #[derive(Default)]
    | |                   ------- in this macro invocation (#3)
659 | |          pub struct Arena<$tcx> {
...   |
662 | |              $($name: $crate::arena_for_type!($a[$ty]),)*
...   |
741 | |      }
742 | |  }
742 | |  }
    | |__- in this expansion of `rustc_arena::declare_arena!` (#2)
    | 
   ::: compiler/rustc_middle/src/arena.rs:11:1
    |
11  |  / macro_rules! arena_types {
12  |  |     ($macro:path, $args:tt, $tcx:lifetime) => (
13  |  |         $macro!($args, [
    |  |_________-
14  | ||             [] layouts: rustc_target::abi::Layout,
15  | ||             // AdtDef are interned and compared by address
16  | ||             [] adt_def: rustc_middle::ty::AdtDef,
...   ||
106 | ||             [decode] used_trait_imports: rustc_data_structures::fx::FxHashSet<rustc_hir::def_id::LocalDefId>,
107 | ||         ], $tcx);
    | ||_________________- in this macro invocation (#2)
108 |  |     )
109 |  | }
    |  |_- in this expansion of `arena_types!` (#1)
110 | 
111 |    arena_types!(rustc_arena::declare_arena, [], 'tcx);
    |    --------------------------------------------------- in this macro invocation (#1)
   ::: /checkout/library/core/src/default.rs:166:1
    |
    |
166 | /  pub macro Default($item:item) {
167 | |      /* compiler built-in */
168 | |  }
    | |__- in this expansion of `#[derive(Default)]` (#3)
    |
    = note: cannot satisfy `_: std::default::Default`
    = note: required by `std::default::Default::default`
error: aborting due to 38 previous errors

Some errors have detailed explanations: E0283, E0412, E0432, E0433, E0585.
For more information about an error, try `rustc --explain E0283`.
For more information about an error, try `rustc --explain E0283`.
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_graphviz" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_resolve" "-p" "rustc_arena" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_symbol_mangling" "-p" "rustc_attr" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_trait_selection" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_typeck" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_ast_passes" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_metadata" "-p" "rustc_target" "-p" "rustc_parse" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_session" "-p" "rustc_data_structures" "-p" "rustc_feature" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_errors" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_hir" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_ast_pretty" "-p" "rustc_ast" "-p" "rustc_serialize" "-p" "rustc_save_analysis" "-p" "rustc_span" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:19
