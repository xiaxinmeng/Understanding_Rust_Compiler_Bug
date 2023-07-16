plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `(dyn CrateStore + std::marker::Sync + 'static)` cannot be sent between threads safely
     |
     |
2001 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |                                 ^^^^^^^^^^^^^^^^^^^^ `(dyn CrateStore + std::marker::Sync + 'static)` cannot be sent between threads safely
     |
     = help: the trait `Send` is not implemented for `(dyn CrateStore + std::marker::Sync + 'static)`
     = note: required for `Unique<(dyn CrateStore + std::marker::Sync + 'static)>` to implement `Send`
     = note: required because it appears within the type `std::boxed::Box<(dyn CrateStore + std::marker::Sync + 'static)>`
     = note: required because it appears within the type `(std::boxed::Box<(dyn CrateStore + std::marker::Sync + 'static)>, DepNodeIndex)`
     = note: required for `TypedArena<(std::boxed::Box<(dyn CrateStore + std::marker::Sync + 'static)>, DepNodeIndex)>` to implement `Send`
     = note: required for `WorkerLocal<TypedArena<(std::boxed::Box<(dyn CrateStore + std::marker::Sync + 'static)>, DepNodeIndex)>>` to implement `std::marker::Sync`
     = note: required because it appears within the type `rustc_query_system::query::caches::ArenaCache<'_, (), std::boxed::Box<(dyn CrateStore + std::marker::Sync + 'static)>>`
note: required because it appears within the type `QueryCaches<'_>`
     |
179  | /  macro_rules! define_callbacks {
180  | |      (
180  | |      (
181  | |       $($(#[$attr:meta])*
182  | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
216  | |          pub struct QueryCaches<'tcx> {
     | |                     ^^^^^^^^^^^
...    |
328  | |      };
328  | |      };
329  | |  }
     | |__- in this expansion of `define_callbacks!` (#2)
...
393  |    rustc_query_append! { define_callbacks! }
     |
    ::: compiler/rustc_middle/src/query/mod.rs:24:1
     |
24   |  / rustc_queries! {
---
    --> compiler/rustc_middle/src/ty/context.rs:1083:12
     |
1083 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:1071:12
     |
1071 | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
note: required because it appears within the type `ImplicitCtxt<'_, '_>`
    --> compiler/rustc_middle/src/ty/context.rs:1905:16
     |
1905 |     pub struct ImplicitCtxt<'a, 'tcx> {
note: required by a bound in `assert_sync`
    --> /checkout/compiler/rustc_data_structures/src/sync.rs:363:32
     |
     |
363  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ^^^^ required by this bound in `assert_sync`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to previous error
