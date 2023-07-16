plain
    |
48  |    pub macro panic_2021 {
    |    -------------------- in this expansion of `$crate::panic::panic_2021!` (#4)
...
57  |            $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'rustc_type_ir::WithCachedTypeInfo<rustc_middle::ty::PredicateKind<'tcx>> is Copy, don't bother adding it to the arena, it can always be allocated.', compiler/rustc_middle/src/arena.rs:126:1
   ::: compiler/rustc_middle/src/arena.rs:8:1
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
121 | ||             [] closure_kind_origin: (rustc_span::Span, rustc_middle::hir::place::Place<'tcx>),
    | ||__________- in this macro invocation (#2)
123 | |      )
124 | |  }
124 | |  }
    | |__- in this expansion of `arena_types!` (#1)
125 |
126 |    arena_types!(rustc_arena::declare_arena);
    |
   ::: /checkout/compiler/rustc_arena/src/lib.rs:563:1
    |
    |
563 |    pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
    |    ----------------------- in this expansion of `rustc_arena::declare_arena!` (#2)
...
660 |                        panic!(concat!(stringify!($ty), " is Copy, don't bother adding it to the arena, it can always be allocated."));
    |                        |
    |                        in this macro invocation (#3)
    |                        in this macro invocation (#4)
    |
    |
   ::: /checkout/library/std/src/macros.rs:14:1
    |
14  |    macro_rules! panic {
    |    ------------------ in this expansion of `panic!` (#3)
    |
note: inside `arena::_::test`
   ::: compiler/rustc_middle/src/arena.rs:8:1
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
121 | ||             [] closure_kind_origin: (rustc_span::Span, rustc_middle::hir::place::Place<'tcx>),
    | ||__________- in this macro invocation (#2)
123 | |      )
124 | |  }
124 | |  }
    | |__- in this expansion of `arena_types!` (#1)
125 |
126 |    arena_types!(rustc_arena::declare_arena);
    |
   ::: /checkout/compiler/rustc_arena/src/lib.rs:563:1
    |
    |
563 |    pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
    |    ----------------------- in this expansion of `rustc_arena::declare_arena!` (#2)
...
660 |                        panic!(concat!(stringify!($ty), " is Copy, don't bother adding it to the arena, it can always be allocated."));
    |                        |
    |                        in this macro invocation (#3)
    |                        in this macro invocation (#4)
    |
---
    |    -------------------- in this expansion of `$crate::panic::panic_2021!` (#4)
...
57  |            $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `arena::_`
   --> /checkout/compiler/rustc_arena/src/lib.rs:663:13
    |
563 |    pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
    |    ----------------------- in this expansion of `rustc_arena::declare_arena!` (#2)
663 |                test();
    |                ^^^^^^
    |
   ::: compiler/rustc_middle/src/arena.rs:8:1
   ::: compiler/rustc_middle/src/arena.rs:8:1
    |
8   | /  macro_rules! arena_types {
9   | |      ($macro:path) => (
10  | |          $macro!([
    | | ________________-
11  | ||             [] layout: rustc_target::abi::LayoutS,
12  | ||             [] fn_abi: rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
13  | ||             // AdtDef are interned and compared by address
...   ||
121 | ||             [] closure_kind_origin: (rustc_span::Span, rustc_middle::hir::place::Place<'tcx>),
    | ||__________- in this macro invocation (#2)
123 | |      )
124 | |  }
124 | |  }
    | |__- in this expansion of `arena_types!` (#1)
125 |
126 |    arena_types!(rustc_arena::declare_arena);

For more information about this error, try `rustc --explain E0080`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
