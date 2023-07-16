plain
    Checking tempfile v3.2.0
    Checking synstructure v0.12.6
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling tracing-attributes v0.1.17
   Compiling chalk-derive v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking chalk-ir v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking tracing-subscriber v0.3.3
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking tracing-tree v0.2.0
    Checking tracing-tree v0.2.0
    Checking chalk-solve v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
    Checking chalk-engine v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
---
    Checking tracing-tree v0.2.0
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
  --> src/librustdoc/html/render/cache.rs:67:9
   |
67 |         std::cmp::Ord::cmp(&k1, &k2)
   |         ^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
   |
   = note: required because of the requirements on the impl of `Ord` for `std::option::Option<rustc_span::def_id::DefId>`
   = note: 2 redundant requirements hidden
   = note: required because of the requirements on the impl of `Ord` for `(&std::string::String, &std::string::String, &ItemType, &std::option::Option<rustc_span::def_id::DefId>)`

error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
     |
     |
544  |   #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
     |                                              ---------- in this derive macro expansion
545  |   crate enum ImplTraitParam {
546  |       DefId(DefId),
     |             ^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`

error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
    |
    |
544 |   #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    |                                                          --- in this derive macro expansion
545 |   crate enum ImplTraitParam {
546 |       DefId(DefId),
    |             ^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
   ::: /checkout/library/core/src/cmp.rs:832:1
    |
    |
832 | / pub macro Ord($item:item) {
834 | | }
834 | | }
    | |_- in this expansion of `#[derive(Ord)]`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustdoc` due to 3 previous errors
Build completed unsuccessfully in 0:02:38
