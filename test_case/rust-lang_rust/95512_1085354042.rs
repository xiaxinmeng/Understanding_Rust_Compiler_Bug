plain
   Compiling chalk-derive v0.80.0
    Checking unic-langid-macros v0.9.0
    Checking unic-langid v0.9.0
    Checking chalk-ir v0.80.0
    Checking intl_pluralrules v7.0.1
    Checking fluent-langneg v0.13.0
    Checking intl-memoizer v0.5.1
    Checking thiserror v1.0.30
    Checking tracing-subscriber v0.3.3
    Checking fluent-syntax v0.11.0
    Checking thorin-dwp v0.2.0
    Checking thorin-dwp v0.2.0
    Checking fluent-bundle v0.15.2
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking tracing-tree v0.2.0
    Checking chalk-solve v0.80.0
    Checking rustc_log v0.0.0 (/checkout/compiler/rustc_log)
---
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0277]: `RefCell<type_map::TypeMap>` cannot be shared between threads safely
   |
   |
59 |         let handler = Handler::with_emitter(true, None, Box::new(je));
   |                       ---------------------             ^^^^^^^^^^^^ `RefCell<type_map::TypeMap>` cannot be shared between threads safely
   |                       required by a bound introduced by this call
   |
   |
   = help: within `fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>`, the trait `Sync` is not implemented for `RefCell<type_map::TypeMap>`
   = note: required because it appears within the type `intl_memoizer::IntlLangMemoizer`
   = note: required because it appears within the type `fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>`
   = note: required because of the requirements on the impl of `Send` for `std::sync::Arc<fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>>`
note: required because it appears within the type `JsonEmitter`
   |
   |
36 | pub struct JsonEmitter {
   = note: required for the cast to the object type `dyn Emitter + Send`


error[E0277]: `(dyn Any + 'static)` cannot be sent between threads safely
   |
   |
59 |         let handler = Handler::with_emitter(true, None, Box::new(je));
   |                       ---------------------             ^^^^^^^^^^^^ `(dyn Any + 'static)` cannot be sent between threads safely
   |                       required by a bound introduced by this call
   |
   |
   = help: the trait `Send` is not implemented for `(dyn Any + 'static)`
   = note: required because of the requirements on the impl of `Send` for `Unique<(dyn Any + 'static)>`
   = note: required because it appears within the type `Box<(dyn Any + 'static)>`
   = note: required because it appears within the type `(TypeId, Box<(dyn Any + 'static)>)`
   = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<(TypeId, Box<(dyn Any + 'static)>)>`
   = note: required because it appears within the type `hashbrown::map::HashMap<TypeId, Box<(dyn Any + 'static)>, BuildHasherDefault<FxHasher>>`
   = note: required because it appears within the type `HashMap<TypeId, Box<(dyn Any + 'static)>, BuildHasherDefault<FxHasher>>`
   = note: required because it appears within the type `Option<HashMap<TypeId, Box<(dyn Any + 'static)>, BuildHasherDefault<FxHasher>>>`
   = note: required because it appears within the type `type_map::TypeMap`
   = note: required because of the requirements on the impl of `Send` for `RefCell<type_map::TypeMap>`
   = note: required because it appears within the type `intl_memoizer::IntlLangMemoizer`
   = note: required because it appears within the type `fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>`
   = note: required because of the requirements on the impl of `Send` for `std::sync::Arc<fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>>`
note: required because it appears within the type `JsonEmitter`
   |
   |
36 | pub struct JsonEmitter {
   = note: required for the cast to the object type `dyn Emitter + Send`


error[E0277]: `RefCell<type_map::TypeMap>` cannot be shared between threads safely
    |
    |
578 |         Self::with_emitter_and_flags(emitter, flags)
    |         ---------------------------- ^^^^^^^ `RefCell<type_map::TypeMap>` cannot be shared between threads safely
    |         required by a bound introduced by this call
    |
    |
    = help: within `fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>`, the trait `Sync` is not implemented for `RefCell<type_map::TypeMap>`
    = note: required because it appears within the type `intl_memoizer::IntlLangMemoizer`
    = note: required because it appears within the type `fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>`
    = note: required because of the requirements on the impl of `Send` for `Arc<fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>>`
note: required because it appears within the type `EmitterWriter`
    |
694 | pub struct EmitterWriter {
    |            ^^^^^^^^^^^^^
    = note: required for the cast to the object type `dyn Emitter + Send`
    = note: required for the cast to the object type `dyn Emitter + Send`

error[E0277]: `(dyn Any + 'static)` cannot be sent between threads safely
    |
    |
578 |         Self::with_emitter_and_flags(emitter, flags)
    |         ---------------------------- ^^^^^^^ `(dyn Any + 'static)` cannot be sent between threads safely
    |         required by a bound introduced by this call
    |
    |
    = help: the trait `Send` is not implemented for `(dyn Any + 'static)`
    = note: required because of the requirements on the impl of `Send` for `Unique<(dyn Any + 'static)>`
    = note: required because it appears within the type `Box<(dyn Any + 'static)>`
    = note: required because it appears within the type `(TypeId, Box<(dyn Any + 'static)>)`
    = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<(TypeId, Box<(dyn Any + 'static)>)>`
    = note: required because it appears within the type `hashbrown::map::HashMap<TypeId, Box<(dyn Any + 'static)>, BuildHasherDefault<FxHasher>>`
    = note: required because it appears within the type `HashMap<TypeId, Box<(dyn Any + 'static)>, BuildHasherDefault<FxHasher>>`
    = note: required because it appears within the type `Option<HashMap<TypeId, Box<(dyn Any + 'static)>, BuildHasherDefault<FxHasher>>>`
    = note: required because it appears within the type `type_map::TypeMap`
    = note: required because of the requirements on the impl of `Send` for `RefCell<type_map::TypeMap>`
    = note: required because it appears within the type `intl_memoizer::IntlLangMemoizer`
    = note: required because it appears within the type `fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>`
    = note: required because of the requirements on the impl of `Send` for `Arc<fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>>`
note: required because it appears within the type `EmitterWriter`
    |
694 | pub struct EmitterWriter {
    |            ^^^^^^^^^^^^^
    = note: required for the cast to the object type `dyn Emitter + Send`
    = note: required for the cast to the object type `dyn Emitter + Send`

error[E0277]: `RefCell<type_map::TypeMap>` cannot be shared between threads safely
    |
    |
578 |         Self::with_emitter_and_flags(emitter, flags)
    |         ---------------------------- ^^^^^^^ `RefCell<type_map::TypeMap>` cannot be shared between threads safely
    |         required by a bound introduced by this call
    |
    |
    = help: within `fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>`, the trait `Sync` is not implemented for `RefCell<type_map::TypeMap>`
    = note: required because it appears within the type `intl_memoizer::IntlLangMemoizer`
    = note: required because it appears within the type `fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>`
    = note: required because of the requirements on the impl of `Send` for `std::sync::Arc<fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>>`
note: required because it appears within the type `EmitterWriter`
    |
694 | pub struct EmitterWriter {
    |            ^^^^^^^^^^^^^
    = note: required for the cast to the object type `dyn Emitter + Send`
    = note: required for the cast to the object type `dyn Emitter + Send`

error[E0277]: `(dyn Any + 'static)` cannot be sent between threads safely
    |
    |
578 |         Self::with_emitter_and_flags(emitter, flags)
    |         ---------------------------- ^^^^^^^ `(dyn Any + 'static)` cannot be sent between threads safely
    |         required by a bound introduced by this call
    |
    |
    = help: the trait `Send` is not implemented for `(dyn Any + 'static)`
    = note: required because of the requirements on the impl of `Send` for `Unique<(dyn Any + 'static)>`
    = note: required because it appears within the type `Box<(dyn Any + 'static)>`
    = note: required because it appears within the type `(TypeId, Box<(dyn Any + 'static)>)`
    = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<(TypeId, Box<(dyn Any + 'static)>)>`
    = note: required because it appears within the type `hashbrown::map::HashMap<TypeId, Box<(dyn Any + 'static)>, BuildHasherDefault<FxHasher>>`
    = note: required because it appears within the type `HashMap<TypeId, Box<(dyn Any + 'static)>, BuildHasherDefault<FxHasher>>`
    = note: required because it appears within the type `Option<HashMap<TypeId, Box<(dyn Any + 'static)>, BuildHasherDefault<FxHasher>>>`
    = note: required because it appears within the type `type_map::TypeMap`
    = note: required because of the requirements on the impl of `Send` for `RefCell<type_map::TypeMap>`
    = note: required because it appears within the type `intl_memoizer::IntlLangMemoizer`
    = note: required because it appears within the type `fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>`
    = note: required because of the requirements on the impl of `Send` for `std::sync::Arc<fluent_bundle::bundle::FluentBundle<fluent_bundle::resource::FluentResource, intl_memoizer::IntlLangMemoizer>>`
note: required because it appears within the type `EmitterWriter`
    |
694 | pub struct EmitterWriter {
    |            ^^^^^^^^^^^^^
    = note: required for the cast to the object type `dyn Emitter + Send`
