plain
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
error[E0412]: cannot find type `LocalDefId` in this scope
   --> compiler/rustc_metadata/src/rmeta/decoder.rs:975:18
    |
975 |     ) -> &'tcx [(LocalDefId, Ident, ast::MetaItem)] {
    |
help: consider importing one of these items
    |
    |
3   | use crate::rmeta::rustc_span::def_id::LocalDefId;
3   | use rustc_hir::def_id::LocalDefId;
    |
3   | use rustc_span::def_id::LocalDefId;
    |
    |

error[E0277]: the trait bound `rustc_ast::MetaItem: std::marker::Copy` is not satisfied in `(DefId, rustc_span::symbol::Ident, rustc_ast::MetaItem)`
   --> compiler/rustc_metadata/src/rmeta/decoder.rs:978:19
978 |         tcx.arena.alloc_from_iter(item_names)
978 |         tcx.arena.alloc_from_iter(item_names)
    |                   ^^^^^^^^^^^^^^^ within `(DefId, rustc_span::symbol::Ident, rustc_ast::MetaItem)`, the trait `std::marker::Copy` is not implemented for `rustc_ast::MetaItem`
    |
    = help: the following other types implement trait `rustc_middle::arena::ArenaAllocatable<'tcx, C>`:
              (LocalDefId, rustc_span::symbol::Ident, rustc_ast::MetaItem)
              (rustc_span::Span, rustc_middle::hir::place::Place<'tcx>)
    = note: required because it appears within the type `(DefId, Ident, MetaItem)`
    = note: required for `(DefId, rustc_span::symbol::Ident, rustc_ast::MetaItem)` to implement `rustc_middle::arena::ArenaAllocatable<'_, rustc_arena::IsCopy>`
note: required by a bound in `rustc_middle::arena::Arena::<'tcx>::alloc_from_iter`
   --> /checkout/compiler/rustc_arena/src/lib.rs:632:39
    |
563 |    pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
    |    ----------------------- in this expansion of `rustc_arena::declare_arena!` (#2)
...
632 |            pub fn alloc_from_iter<'a, T: ArenaAllocatable<'tcx, C>, C>(
    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Arena::<'tcx>::alloc_from_iter`
   ::: /checkout/compiler/rustc_middle/src/arena.rs:8:1
    |
8   | /  macro_rules! arena_types {
8   | /  macro_rules! arena_types {
9   | |      ($macro:path) => (
10  | |          $macro!([
    | | ________________-
11  | ||             [] layout: rustc_target::abi::LayoutS,
12  | ||             [] fn_abi: rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
13  | ||             // AdtDef are interned and compared by address
...   ||
122 | ||             [] stripped_out_item_names: (rustc_span::def_id::LocalDefId, rustc_span::symbol::Ident, rustc_ast::MetaItem),
    | ||__________- in this macro invocation (#2)
124 | |      )
125 | |  }
125 | |  }
    | |__- in this expansion of `arena_types!` (#1)
126 |
127 |    arena_types!(rustc_arena::declare_arena);


error[E0277]: the trait bound `&(LocalDefId, rustc_span::symbol::Ident, rustc_ast::MetaItem): std::borrow::Borrow<(DefId, rustc_span::symbol::Ident, rustc_ast::MetaItem)>` is not satisfied
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1866:14
     |
1866 |         self.lazy_array(self.tcx.stripped_out_item_names(LOCAL_CRATE))
     |              ^^^^^^^^^^ the trait `std::borrow::Borrow<(DefId, rustc_span::symbol::Ident, rustc_ast::MetaItem)>` is not implemented for `&(LocalDefId, rustc_span::symbol::Ident, rustc_ast::MetaItem)`
     |
note: required by a bound in `EncodeContext::<'a, 'tcx>::lazy_array`
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:441:74
     |
441  |     fn lazy_array<T: ParameterizedOverTcx, I: IntoIterator<Item = B>, B: Borrow<T::Value<'tcx>>>(
     |                                                                          ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `EncodeContext::<'a, 'tcx>::lazy_array`
Some errors have detailed explanations: E0277, E0412.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_metadata` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
