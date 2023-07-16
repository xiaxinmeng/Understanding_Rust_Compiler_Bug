plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0412]: cannot find type `DtorckConstraint` in module `rustc_middle::traits::query`
   --> compiler/rustc_middle/src/arena.rs:55:64
    |
6   | / macro_rules! arena_types {
7   | |     ($macro:path) => (
8   | |         $macro!([
9   | |             [] layout: rustc_target::abi::LayoutS<'tcx>,
...   |
55  | |             [] dtorck_constraint: rustc_middle::traits::query::DtorckConstraint<'tcx>,
    | |                                                                ^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `DropckConstraint`
106 | |     )
107 | | }
107 | | }
    | |_- in this expansion of `arena_types!`
108 | 
109 |   arena_types!(rustc_arena::declare_arena);
    |
   ::: compiler/rustc_middle/src/traits/query.rs:146:1
    |
    |
146 |   pub struct DropckConstraint<'tcx> {
    |   --------------------------------- similarly named struct `DropckConstraint` defined here

error[E0412]: cannot find type `DtorckConstraint` in this scope
     |
18   | / rustc_queries! {
19   | |     query trigger_delay_span_bug(key: DefId) -> () {
20   | |         desc { "trigger a delay span bug" }
20   | |         desc { "trigger a delay span bug" }
21   | |     }
...    |
552  | |     ) -> Result<&'tcx DtorckConstraint<'tcx>, NoSolution> {
     | |                       ^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `DropckConstraint`
1963 | |     }
1964 | | }
     | |_- in this expansion of `rustc_query_append!`
     |
     |
    ::: compiler/rustc_middle/src/traits/query.rs:146:1
     |
146  |   pub struct DropckConstraint<'tcx> {
     |   --------------------------------- similarly named struct `DropckConstraint` defined here
    ::: compiler/rustc_middle/src/ty/query.rs:335:1
     |
335  |   rustc_query_append! { [define_callbacks!][<'tcx>] }
     |   --------------------------------------------------- in this macro invocation
     |   --------------------------------------------------- in this macro invocation

error: unused import: `DropckConstraint`
  --> compiler/rustc_middle/src/ty/query.rs:26:27
   |
26 |     DropckOutlivesResult, DropckConstraint, MethodAutoderefStepsResult, NormalizationResult,
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`

error[E0599]: no method named `alloc_from_iter` found for struct `DroplessArena` in the current scope
   --> /checkout/compiler/rustc_arena/src/lib.rs:588:36
    |
542 | /  pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
544 | |      pub struct Arena<'tcx> {
544 | |      pub struct Arena<'tcx> {
545 | |          pub dropless: $crate::DroplessArena,
...   |
588 | |                      arena.dropless.alloc_from_iter(iter)
    | |                                     ^^^^^^^^^^^^^^^ method not found in `DroplessArena`
617 | |      }
618 | |  }
618 | |  }
    | |__- in this expansion of `rustc_arena::declare_arena!` (#2)
    |
   ::: compiler/rustc_middle/src/arena.rs:6:1
    |
6   |  / macro_rules! arena_types {
7   |  |     ($macro:path) => (
8   |  |         $macro!([
    |  |________________-
9   | ||             [] layout: rustc_target::abi::LayoutS<'tcx>,
10  | ||             [] fn_abi: rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
11  | ||             // AdtDef are interned and compared by address
...   ||
104 | ||             [] dep_kind: rustc_middle::dep_graph::DepKindStruct,
    | ||__________- in this macro invocation (#2)
106 |  |     )
107 |  | }
107 |  | }
    |  |_- in this expansion of `arena_types!` (#1)
108 | 
109 |    arena_types!(rustc_arena::declare_arena);

error[E0283]: type annotations needed
   --> /checkout/compiler/rustc_arena/src/lib.rs:546:11
    |
    |
542 | /  pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
    | |               ------- in this derive macro expansion (#3)
544 | |      pub struct Arena<'tcx> {
544 | |      pub struct Arena<'tcx> {
545 | |          pub dropless: $crate::DroplessArena,
546 | |          $($name: $crate::TypedArena<$ty>,)*
...   |
617 | |      }
618 | |  }
618 | |  }
    | |__- in this expansion of `rustc_arena::declare_arena!` (#2)
    |
   ::: compiler/rustc_middle/src/arena.rs:6:1
    |
6   |  / macro_rules! arena_types {
7   |  |     ($macro:path) => (
8   |  |         $macro!([
    |  |________________-
9   | ||             [] layout: rustc_target::abi::LayoutS<'tcx>,
10  | ||             [] fn_abi: rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
11  | ||             // AdtDef are interned and compared by address
...   ||
104 | ||             [] dep_kind: rustc_middle::dep_graph::DepKindStruct,
    | ||__________- in this macro invocation (#2)
106 |  |     )
107 |  | }
107 |  | }
    |  |_- in this expansion of `arena_types!` (#1)
108 | 
109 |    arena_types!(rustc_arena::declare_arena);
    |
   ::: /checkout/library/core/src/default.rs:168:1
    |
168 | /  pub macro Default($item:item) {
---

error[E0283]: type annotations needed
    --> compiler/rustc_middle/src/ty/query.rs:213:28
     |
175  | /   macro_rules! define_callbacks {
176  | |       (<$tcx:tt>
177  | |        $($(#[$attr:meta])*
178  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
211  | |           #[derive(Default)]
     | |                    ------- in this derive macro expansion (#3)
     | |                    ------- in this derive macro expansion (#3)
212  | |           pub struct QueryCaches<$tcx> {
213  | |               $($(#[$attr])* pub $name: query_storage::$name<$tcx>,)*
...    |
320  | |       };
321  | |   }
321  | |   }
     | |___- in this expansion of `define_callbacks!` (#2)
335  |     rustc_query_append! { [define_callbacks!][<'tcx>] }
     |     --------------------------------------------------- in this macro invocation (#1)
     |
    ::: compiler/rustc_middle/src/query/mod.rs:18:1
