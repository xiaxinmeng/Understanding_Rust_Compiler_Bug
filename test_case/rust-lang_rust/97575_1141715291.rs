plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0277]: `RefCell<SourceFileLines>` cannot be shared between threads safely
   |
   |
59 |         let handler = Handler::with_emitter(true, None, Box::new(je));
   |                       ---------------------             ^^^^^^^^^^^^ `RefCell<SourceFileLines>` cannot be shared between threads safely
   |                       required by a bound introduced by this call
   |
   |
   = help: within `SourceFile`, the trait `Sync` is not implemented for `RefCell<SourceFileLines>`
   = note: required because it appears within the type `SourceFile`
   = note: required because of the requirements on the impl of `Send` for `std::sync::Arc<SourceFile>`
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Send` for `Unique<std::sync::Arc<SourceFile>>`
   = note: required because it appears within the type `alloc::raw_vec::RawVec<std::sync::Arc<SourceFile>>`
   = note: required because it appears within the type `Vec<std::sync::Arc<SourceFile>>`
   = note: required because it appears within the type `MonotonicVec<std::sync::Arc<SourceFile>>`
   = note: required because it appears within the type `source_map::SourceMapFiles`
   = note: required because of the requirements on the impl of `Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, source_map::SourceMapFiles>`
   = note: required because it appears within the type `rustc_data_structures::sync::RwLock<source_map::SourceMapFiles>`
   = note: required because it appears within the type `SourceMap`
   = note: required because of the requirements on the impl of `Send` for `std::sync::Arc<SourceMap>`
note: required because it appears within the type `JsonEmitter`
   |
36 | pub struct JsonEmitter {
   |            ^^^^^^^^^^^
   = note: required for the cast to the object type `dyn Emitter + Send`
   = note: required for the cast to the object type `dyn Emitter + Send`

error[E0277]: `RefCell<SourceFileLines>` cannot be shared between threads safely
    |
    |
551 |         Self::with_emitter_and_flags(emitter, flags)
    |         ---------------------------- ^^^^^^^ `RefCell<SourceFileLines>` cannot be shared between threads safely
    |         required by a bound introduced by this call
    |
    |
    = help: within `SourceFile`, the trait `Sync` is not implemented for `RefCell<SourceFileLines>`
    = note: required because it appears within the type `SourceFile`
    = note: required because of the requirements on the impl of `Send` for `Arc<SourceFile>`
    = note: 1 redundant requirement hidden
    = note: required because of the requirements on the impl of `Send` for `Unique<Arc<SourceFile>>`
    = note: required because it appears within the type `alloc::raw_vec::RawVec<Arc<SourceFile>>`
    = note: required because it appears within the type `Vec<Arc<SourceFile>>`
    = note: required because it appears within the type `MonotonicVec<Arc<SourceFile>>`
    = note: required because it appears within the type `source_map::SourceMapFiles`
    = note: required because of the requirements on the impl of `Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, source_map::SourceMapFiles>`
    = note: required because it appears within the type `rustc_data_structures::sync::RwLock<source_map::SourceMapFiles>`
    = note: required because it appears within the type `SourceMap`
    = note: required because of the requirements on the impl of `Send` for `Arc<SourceMap>`
    = note: required because it appears within the type `Option<Arc<SourceMap>>`
note: required because it appears within the type `EmitterWriter`
    |
700 | pub struct EmitterWriter {
    |            ^^^^^^^^^^^^^
    = note: required for the cast to the object type `dyn Emitter + Send`
    = note: required for the cast to the object type `dyn Emitter + Send`

error[E0277]: `RefCell<SourceFileLines>` cannot be shared between threads safely
    |
    |
551 |         Self::with_emitter_and_flags(emitter, flags)
    |         ---------------------------- ^^^^^^^ `RefCell<SourceFileLines>` cannot be shared between threads safely
    |         required by a bound introduced by this call
    |
    |
    = help: within `SourceFile`, the trait `Sync` is not implemented for `RefCell<SourceFileLines>`
    = note: required because it appears within the type `SourceFile`
    = note: required because of the requirements on the impl of `Send` for `std::sync::Arc<SourceFile>`
    = note: 1 redundant requirement hidden
    = note: required because of the requirements on the impl of `Send` for `Unique<std::sync::Arc<SourceFile>>`
    = note: required because it appears within the type `alloc::raw_vec::RawVec<std::sync::Arc<SourceFile>>`
    = note: required because it appears within the type `Vec<std::sync::Arc<SourceFile>>`
    = note: required because it appears within the type `MonotonicVec<std::sync::Arc<SourceFile>>`
    = note: required because it appears within the type `source_map::SourceMapFiles`
    = note: required because of the requirements on the impl of `Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, source_map::SourceMapFiles>`
    = note: required because it appears within the type `rustc_data_structures::sync::RwLock<source_map::SourceMapFiles>`
    = note: required because it appears within the type `SourceMap`
    = note: required because of the requirements on the impl of `Send` for `std::sync::Arc<SourceMap>`
    = note: required because it appears within the type `Option<std::sync::Arc<SourceMap>>`
note: required because it appears within the type `EmitterWriter`
    |
700 | pub struct EmitterWriter {
    |            ^^^^^^^^^^^^^
    = note: required for the cast to the object type `dyn Emitter + Send`
