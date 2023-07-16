plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `NonNull<list::List<Predicate<'_>>>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/ty/context/tls.rs:98:29
98   |         sync::assert_sync::<ImplicitCtxt<'_, '_>>();
98   |         sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |                             ^^^^^^^^^^^^^^^^^^^^ `NonNull<list::List<Predicate<'_>>>` cannot be shared between threads safely
     |
     = help: within `ImplicitCtxt<'_, '_>`, the trait `std::marker::Sync` is not implemented for `NonNull<list::List<Predicate<'_>>>`
     = note: required because it appears within the type `CopyTaggedPtr<&List<Predicate<'_>>, ParamTag, true>`
note: required because it appears within the type `ParamEnv<'_>`
     |
1606 | pub struct ParamEnv<'tcx> {
     |            ^^^^^^^^
     |            ^^^^^^^^
     = note: required because it appears within the type `PhantomData<ParamEnv<'_>>`
note: required because it appears within the type `QueryPhantomValues<'_>`
     |
215  |  / macro_rules! define_callbacks {
216  |  |     (
216  |  |     (
217  |  |      $($(#[$attr:meta])*
218  |  |         [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
342  |  |         pub struct QueryPhantomValues<'tcx> {
     |  |                    ^^^^^^^^^^^^^^^^^^
...     |
473  |  |     };
473  |  |     };
474  |  | }
     |  |_- in this expansion of `define_callbacks!` (#2)
...
540  |    rustc_query_append! { define_callbacks! }
     |
    ::: compiler/rustc_middle/src/query/mod.rs:25:1
     |
25   | /  rustc_queries! {
---
2229 | |      }
2230 | |  }
     | |  -
     | |  |
     | |__in this expansion of `rustc_query_append!` (#1)
     |    in this macro invocation (#2)
note: required because it appears within the type `QuerySystem<'_>`
     |
     |
80   | pub struct QuerySystem<'tcx> {
note: required because it appears within the type `GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:486:12
     |
486  | pub struct GlobalCtxt<'tcx> {
486  | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:474:12
     |
474  | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
---
     |            ^^^^^^^^^^^^
note: required by a bound in `assert_sync`
    --> /checkout/compiler/rustc_data_structures/src/sync.rs:381:32
     |
381  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ^^^^ required by this bound in `assert_sync`

error[E0277]: `NonNull<list::List<Predicate<'_>>>` cannot be sent between threads safely
    --> compiler/rustc_middle/src/ty/context/tls.rs:98:29
98   |         sync::assert_sync::<ImplicitCtxt<'_, '_>>();
98   |         sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |                             ^^^^^^^^^^^^^^^^^^^^ `NonNull<list::List<Predicate<'_>>>` cannot be sent between threads safely
     |
     = help: within `((ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<std::option::Option<SelectionCandidate<'_>>, SelectionError<'_>>>)`, the trait `Send` is not implemented for `NonNull<list::List<Predicate<'_>>>`
     = note: required because it appears within the type `CopyTaggedPtr<&List<Predicate<'_>>, ParamTag, true>`
note: required because it appears within the type `ParamEnv<'_>`
     |
1606 | pub struct ParamEnv<'tcx> {
     |            ^^^^^^^^
     |            ^^^^^^^^
     = note: required because it appears within the type `(ParamEnv<'_>, TraitPredicate<'_>)`
     = note: required because it appears within the type `((ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>)`
     = note: required for `RawTable<((ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, ...>>)>` to implement `Send`
     = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_middle-0c361dfa0a6ca371.long-type-3122591590547517625.txt'
     = note: required because it appears within the type `HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>, BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>, BuildHasherDefault<FxHasher>>`
     = note: required for `Mutex<RawMutex, HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<...>, ...>>, ...>>` to implement `std::marker::Sync`
     = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_middle-0c361dfa0a6ca371.long-type-14471099073340213314.txt'
     = note: required because it appears within the type `Lock<HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>, BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `Cache<(ParamEnv<'_>, TraitPredicate<'_>), Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>`
note: required because it appears within the type `GlobalCtxt<'_>`
     |
486  | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:474:12
     |
474  | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
---
     |            ^^^^^^^^^^^^
note: required by a bound in `assert_sync`
    --> /checkout/compiler/rustc_data_structures/src/sync.rs:381:32
     |
381  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ^^^^ required by this bound in `assert_sync`

error[E0277]: `NonNull<list::List<Predicate<'_>>>` cannot be sent between threads safely
    --> compiler/rustc_middle/src/ty/context/tls.rs:98:29
98   |         sync::assert_sync::<ImplicitCtxt<'_, '_>>();
98   |         sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |                             ^^^^^^^^^^^^^^^^^^^^ `NonNull<list::List<Predicate<'_>>>` cannot be sent between threads safely
     |
     = help: within `((ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<std::option::Option<SelectionCandidate<'_>>, SelectionError<'_>>>)`, the trait `Send` is not implemented for `NonNull<list::List<Predicate<'_>>>`
     = note: required because it appears within the type `CopyTaggedPtr<&List<Predicate<'_>>, ParamTag, true>`
note: required because it appears within the type `ParamEnv<'_>`
     |
1606 | pub struct ParamEnv<'tcx> {
     |            ^^^^^^^^
     |            ^^^^^^^^
     = note: required because it appears within the type `(ParamEnv<'_>, TraitPredicate<'_>)`
     = note: required because it appears within the type `((ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>)`
     = note: required for `RawTable<((ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, ...>>)>` to implement `Send`
     = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_middle-85bee07204df7470.long-type-18126289194737104644.txt'
     = note: required because it appears within the type `HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>, BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>, BuildHasherDefault<FxHasher>>`
     = note: required for `Mutex<RawMutex, HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<...>, ...>>, ...>>` to implement `std::marker::Sync`
     = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_middle-85bee07204df7470.long-type-15009621601780697506.txt'
     = note: required because it appears within the type `Lock<HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>, BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `Cache<(ParamEnv<'_>, TraitPredicate<'_>), Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>`
note: required because it appears within the type `GlobalCtxt<'_>`
     |
486  | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:474:12
     |
474  | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
---
     |            ^^^^^^^^^^^^
note: required by a bound in `assert_sync`
    --> /checkout/compiler/rustc_data_structures/src/sync.rs:381:32
     |
381  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ^^^^ required by this bound in `assert_sync`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to 2 previous errors
