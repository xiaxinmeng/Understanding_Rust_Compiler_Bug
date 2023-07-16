plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `rustc_data_structures::stable_set::StableSet<rustc_span::Symbol>: ArenaAllocatable<'_, _>` is not satisfied
    --> compiler/rustc_middle/src/ty/context.rs:2950:25
     |
2950 |         tcx.arena.alloc(tcx.resolutions(()).glob_map.get(&id).cloned().unwrap_or_default())
     |                   ----- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an implementor of trait `ArenaAllocatable<'_, _>`
     |                   required by a bound introduced by this call
     |
     |
     = note: required because of the requirements on the impl of `ArenaAllocatable<'_, IsCopy>` for `rustc_data_structures::stable_set::StableSet<rustc_span::Symbol>`
note: required by a bound in `Arena::<'tcx>::alloc`
    --> /checkout/compiler/rustc_arena/src/lib.rs:598:25
     |
542  | /  pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
544  | |      pub struct Arena<'tcx> {
544  | |      pub struct Arena<'tcx> {
545  | |          pub dropless: $crate::DroplessArena,
...    |
598  | |          pub fn alloc<T: ArenaAllocatable<'tcx, C>, C>(&self, value: T) -> &mut T {
     | |                          ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Arena::<'tcx>::alloc`
617  | |      }
618  | |  }
618  | |  }
     | |__- in this expansion of `rustc_arena::declare_arena!` (#2)
     |
    ::: compiler/rustc_middle/src/arena.rs:6:1
     |
6    |  / macro_rules! arena_types {
7    |  |     ($macro:path) => (
8    |  |         $macro!([
     |  |________________-
9    | ||             [] layout: rustc_target::abi::LayoutS<'tcx>,
10   | ||             [] fn_abi: rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
11   | ||             // AdtDef are interned and compared by address
...    ||
104  | ||             [] dep_kind: rustc_middle::dep_graph::DepKindStruct,
     | ||__________- in this macro invocation (#2)
106  |  |     )
107  |  | }
107  |  | }
     |  |_- in this expansion of `arena_types!` (#1)
108  | 
109  |    arena_types!(rustc_arena::declare_arena);
help: consider borrowing here
     |
     |
2950 |         tcx.arena.alloc(&tcx.resolutions(()).glob_map.get(&id).cloned().unwrap_or_default())

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
