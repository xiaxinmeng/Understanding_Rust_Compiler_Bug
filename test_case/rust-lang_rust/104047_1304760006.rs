plain
    Checking zerovec v0.9.0
    Checking rls-data v0.19.1
    Checking tinystr v0.7.0
    Checking unic-langid-impl v0.9.1
    Checking icu_locid v1.0.0
    Checking icu_provider v1.0.1
    Checking icu_provider_adapters v1.0.0
    Checking icu_list v1.0.0
   Compiling unic-langid-macros-impl v0.9.1
    Checking rustc_baked_icu_data v0.0.0 (/checkout/compiler/rustc_baked_icu_data)
   Compiling unic-langid v0.9.1
   Compiling intl_pluralrules v7.0.2
   Compiling intl-memoizer v0.5.1
   Compiling fluent-langneg v0.13.0
---
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
error[E0277]: `Rc<Box<[u8]>>` cannot be shared between threads safely
    |
    |
583 |                 .with_try_get::<MemoizableListFormatter, _, _>((), |list_formatter| {
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^ `Rc<Box<[u8]>>` cannot be shared between threads safely
    |
    = help: within `MemoizableListFormatter`, the trait `Sync` is not implemented for `Rc<Box<[u8]>>`
    = note: required because it appears within the type `icu_provider::response::Cart`
    = note: required because it appears within the type `Option<icu_provider::response::Cart>`
    = note: required because it appears within the type `yoke::yoke::Yoke<ListFormatterPatternsV1<'static>, Option<icu_provider::response::Cart>>`
    = note: required because it appears within the type `icu_provider::response::DataPayload<icu_list::provider::ErasedListV1Marker>`
    = note: required because it appears within the type `ListFormatter`
note: required because it appears within the type `MemoizableListFormatter`
    |
    |
591 |     struct MemoizableListFormatter(icu_list::ListFormatter);
note: required by a bound in `intl_memoizer::concurrent::IntlLangMemoizer::with_try_get`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/intl-memoizer-0.5.1/src/concurrent.rs:21:25
    |
21  |         I: Memoizable + Sync + Send + 'static,
21  |         I: Memoizable + Sync + Send + 'static,
    |                         ^^^^ required by this bound in `intl_memoizer::concurrent::IntlLangMemoizer::with_try_get`

error[E0277]: `Rc<Box<[u8]>>` cannot be sent between threads safely
    |
    |
583 |                 .with_try_get::<MemoizableListFormatter, _, _>((), |list_formatter| {
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^ `Rc<Box<[u8]>>` cannot be sent between threads safely
    |
    = help: within `MemoizableListFormatter`, the trait `Send` is not implemented for `Rc<Box<[u8]>>`
    = note: required because it appears within the type `icu_provider::response::Cart`
    = note: required because it appears within the type `Option<icu_provider::response::Cart>`
    = note: required because it appears within the type `yoke::yoke::Yoke<ListFormatterPatternsV1<'static>, Option<icu_provider::response::Cart>>`
    = note: required because it appears within the type `icu_provider::response::DataPayload<icu_list::provider::ErasedListV1Marker>`
    = note: required because it appears within the type `ListFormatter`
note: required because it appears within the type `MemoizableListFormatter`
    |
    |
591 |     struct MemoizableListFormatter(icu_list::ListFormatter);
note: required by a bound in `intl_memoizer::concurrent::IntlLangMemoizer::with_try_get`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/intl-memoizer-0.5.1/src/concurrent.rs:21:32
    |
21  |         I: Memoizable + Sync + Send + 'static,
