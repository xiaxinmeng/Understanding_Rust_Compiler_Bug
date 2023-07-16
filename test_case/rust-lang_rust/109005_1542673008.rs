plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0277]: the trait bound `SubdiagnosticMessage: From<&std::string::String>` is not satisfied
     |
     |
2230 |             err.span_note(name.span, &format!("found an item that was configured out"));
     |                 ---------            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<&std::string::String>` is not implemented for `SubdiagnosticMessage`
     |                 required by a bound introduced by this call
     |
     |
     = note: required for `&std::string::String` to implement `Into<SubdiagnosticMessage>`
note: required by a bound in `Diagnostic::span_note`
     |
     |
496  |         msg: impl Into<SubdiagnosticMessage>,
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Diagnostic::span_note`
    -->  /checkout/library/alloc/src/macros.rs:119:23
     |
119  |     ($($arg:tt)*) => {*{
     |                       +
     |                       +

error[E0277]: `ParamKindInTyOfConstParam` doesn't implement `std::fmt::Debug`
    |
180 | #[derive(Debug)]
    |          ----- in this derive macro expansion
...
...
236 |     ParamInTyOfConstParam { name: Symbol, param_kind: Option<ParamKindInTyOfConstParam> },
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ParamKindInTyOfConstParam` cannot be formatted using `{:?}`
   ::: /checkout/library/core/src/fmt/mod.rs:588:5
    |
588 |     pub macro Debug($item:item) {
    |     --------------- in this expansion of `#[derive(Debug)]`
    |     --------------- in this expansion of `#[derive(Debug)]`
    |
    = help: the trait `std::fmt::Debug` is not implemented for `ParamKindInTyOfConstParam`
    = note: add `#[derive(Debug)]` to `ParamKindInTyOfConstParam` or manually `impl std::fmt::Debug for ParamKindInTyOfConstParam`
    = help: the trait `std::fmt::Debug` is implemented for `std::option::Option<T>`
help: consider annotating `ParamKindInTyOfConstParam` with `#[derive(Debug)]`
    |
334 + #[derive(Debug)]
334 + #[derive(Debug)]
335 |             ident: seg.ident,


error[E0277]: `ParamKindInNonTrivialAnonConst` doesn't implement `std::fmt::Debug`
    |
180 | #[derive(Debug)]
    |          ----- in this derive macro expansion
...
...
240 |     ParamInNonTrivialAnonConst { name: Symbol, param_kind: ParamKindInNonTrivialAnonConst },
    |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ParamKindInNonTrivialAnonConst` cannot be formatted using `{:?}`
   ::: /checkout/library/core/src/fmt/mod.rs:588:5
    |
588 |     pub macro Debug($item:item) {
    |     --------------- in this expansion of `#[derive(Debug)]`
    |     --------------- in this expansion of `#[derive(Debug)]`
    |
    = help: the trait `std::fmt::Debug` is not implemented for `ParamKindInNonTrivialAnonConst`
    = note: add `#[derive(Debug)]` to `ParamKindInNonTrivialAnonConst` or manually `impl std::fmt::Debug for ParamKindInNonTrivialAnonConst`
help: consider annotating `ParamKindInNonTrivialAnonConst` with `#[derive(Debug)]`
    |
369 + #[derive(Debug)]
369 + #[derive(Debug)]
370 |     CrateRootAndExternPrelude,


error[E0277]: `ParamKindInEnumDiscriminant` doesn't implement `std::fmt::Debug`
    |
180 | #[derive(Debug)]
    |          ----- in this derive macro expansion
...
...
244 |     ParamInEnumDiscriminant { name: Symbol, param_kind: ParamKindInEnumDiscriminant },
    |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ParamKindInEnumDiscriminant` cannot be formatted using `{:?}`
   ::: /checkout/library/core/src/fmt/mod.rs:588:5
    |
588 |     pub macro Debug($item:item) {
    |     --------------- in this expansion of `#[derive(Debug)]`
    |     --------------- in this expansion of `#[derive(Debug)]`
    |
    = help: the trait `std::fmt::Debug` is not implemented for `ParamKindInEnumDiscriminant`
    = note: add `#[derive(Debug)]` to `ParamKindInEnumDiscriminant` or manually `impl std::fmt::Debug for ParamKindInEnumDiscriminant`
help: consider annotating `ParamKindInEnumDiscriminant` with `#[derive(Debug)]`
    |
569 + #[derive(Debug)]
570 |             let mut collected_traits = Vec::new();
    |
