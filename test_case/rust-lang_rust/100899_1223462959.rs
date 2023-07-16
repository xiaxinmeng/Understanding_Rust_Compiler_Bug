plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `RefCell<usize>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/ty/context.rs:1944:13
     |
1944 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<usize>` cannot be shared between threads safely
     |
     = help: within `Allocation`, the trait `std::marker::Sync` is not implemented for `RefCell<usize>`
note: required because it appears within the type `AddrAllocBytes`
    --> compiler/rustc_middle/src/mir/interpret/allocation.rs:34:12
     |
34   | pub struct AddrAllocBytes {
note: required because it appears within the type `AllocBytes`
    --> compiler/rustc_middle/src/mir/interpret/allocation.rs:70:10
     |
     |
70   | pub enum AllocBytes {
note: required because it appears within the type `Allocation`
    --> compiler/rustc_middle/src/mir/interpret/allocation.rs:167:12
     |
167  | pub struct Allocation<Prov = AllocId, Extra = ()> {
167  | pub struct Allocation<Prov = AllocId, Extra = ()> {
     |            ^^^^^^^^^^
     = note: required because of the requirements on the impl of `Send` for `&Allocation`
     = note: required because it appears within the type `Interned<'_, Allocation>`
    --> compiler/rustc_middle/src/mir/interpret/allocation.rs:241:12
     |
241  | pub struct ConstAllocation<'tcx, Prov = AllocId, Extra = ()>(
     |            ^^^^^^^^^^^^^^^
     |            ^^^^^^^^^^^^^^^
note: required because it appears within the type `mir::interpret::GlobalAlloc<'_>`
    --> compiler/rustc_middle/src/mir/interpret/mod.rs:399:10
     |
399  | pub enum GlobalAlloc<'tcx> {
     |          ^^^^^^^^^^^
     = note: required because it appears within the type `(AllocId, mir::interpret::GlobalAlloc<'_>)`
     = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<(AllocId, mir::interpret::GlobalAlloc<'_>)>`
     = note: required because it appears within the type `hashbrown::map::HashMap<AllocId, mir::interpret::GlobalAlloc<'_>, BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<AllocId, mir::interpret::GlobalAlloc<'_>, BuildHasherDefault<FxHasher>>`
note: required because it appears within the type `AllocMap<'_>`
     |
     |
443  | pub(crate) struct AllocMap<'tcx> {
     |                   ^^^^^^^^
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, AllocMap<'_>>`
     = note: required because it appears within the type `rustc_data_structures::sync::Lock<AllocMap<'_>>`
note: required because it appears within the type `GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:1047:12
1047 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:1035:12
     |
1035 | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
note: required because it appears within the type `ImplicitCtxt<'_, '_>`
    --> compiler/rustc_middle/src/ty/context.rs:1848:16
     |
1848 |     pub struct ImplicitCtxt<'a, 'tcx> {
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
