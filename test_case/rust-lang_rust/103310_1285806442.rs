plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error[E0275]: overflow evaluating the requirement `Vec<TokenTree>: Send`
    |
    |
181 |     tcx.hir().par_body_owners(|body_owner_def_id| tcx.ensure().typeck(body_owner_def_id));
    |
    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`rustc_hir_typeck`)
    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`rustc_hir_typeck`)
    = note: required for `Arc<Vec<TokenTree>>` to implement `Send`
    = note: required because it appears within the type `TokenStream`
    = note: required because it appears within the type `MacArgs`
    = note: required for `std::ptr::Unique<MacArgs>` to implement `Send`
    = note: required because it appears within the type `std::boxed::Box<MacArgs>`
    = note: required because it appears within the type `P<MacArgs>`
    = note: required for `std::ptr::Unique<rustc_ast::MacCall>` to implement `Send`
    = note: required because it appears within the type `std::boxed::Box<rustc_ast::MacCall>`
    = note: required because it appears within the type `P<rustc_ast::MacCall>`
    = note: required because it appears within the type `rustc_ast::PatKind`
---
    = note: required for `std::ptr::Unique<rustc_ast::Ty>` to implement `Send`
    = note: required because it appears within the type `std::boxed::Box<rustc_ast::Ty>`
    = note: required because it appears within the type `P<rustc_ast::Ty>`
    = note: required because it appears within the type `rustc_ast::GenericArg`
    = note: required because it appears within the type `AngleBracketedArg`
    = note: required for `std::ptr::Unique<AngleBracketedArg>` to implement `Send`
    = note: required because it appears within the type `smallvec::alloc::raw_vec::RawVec<AngleBracketedArg>`
    = note: required because it appears within the type `Vec<AngleBracketedArg>`
    = note: required because it appears within the type `AngleBracketedArgs`
    = note: required for `std::ptr::Unique<rustc_ast::GenericArgs>` to implement `Send`
    = note: required because it appears within the type `std::boxed::Box<rustc_ast::GenericArgs>`
    = note: required because it appears within the type `P<rustc_ast::GenericArgs>`
    = note: required because it appears within the type `Option<P<rustc_ast::GenericArgs>>`
    = note: required because it appears within the type `Option<P<rustc_ast::GenericArgs>>`
    = note: required because it appears within the type `rustc_ast::PathSegment`
    = note: required for `std::ptr::Unique<rustc_ast::PathSegment>` to implement `Send`
    = note: required because it appears within the type `smallvec::alloc::raw_vec::RawVec<rustc_ast::PathSegment>`
    = note: required because it appears within the type `Vec<rustc_ast::PathSegment>`
    = note: required because it appears within the type `rustc_ast::Path`
    = note: required because it appears within the type `AttrItem`
    = note: required because it appears within the type `NormalAttr`
    = note: required for `std::ptr::Unique<NormalAttr>` to implement `Send`
    = note: required because it appears within the type `std::boxed::Box<NormalAttr>`
    = note: required because it appears within the type `P<NormalAttr>`
    = note: required because it appears within the type `Attribute`
    = note: required because it appears within the type `Attribute`
    = note: required for `thin_vec::ThinVec<Attribute>` to implement `Send`
    = note: required for `std::ptr::Unique<rustc_ast::Item>` to implement `Send`
    = note: required because it appears within the type `std::boxed::Box<rustc_ast::Item>`
    = note: required because it appears within the type `P<rustc_ast::Item>`
    = note: required because it appears within the type `Nonterminal`
    = note: required because it appears within the type `Nonterminal`
    = note: required for `Arc<Nonterminal>` to implement `std::marker::Sync`
    = note: required because it appears within the type `TokenKind`
    = note: required because it appears within the type `rustc_ast::token::Token`
    = note: required because it appears within the type `TokenTree`
    = note: required for `std::ptr::Unique<TokenTree>` to implement `std::marker::Sync`
    = note: required because it appears within the type `smallvec::alloc::raw_vec::RawVec<TokenTree>`
    = note: required because it appears within the type `Vec<TokenTree>`
    = note: required for `Arc<Vec<TokenTree>>` to implement `std::marker::Sync`
    = note: required because it appears within the type `TokenStream`
note: required by a bound in `rustc_middle::hir::map::Map::<'hir>::par_body_owners`
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:489:59
    |
489 |     pub fn par_body_owners(self, f: impl Fn(LocalDefId) + Sync + Send) {
    |                                                           ^^^^ required by this bound in `rustc_middle::hir::map::Map::<'hir>::par_body_owners`
For more information about this error, try `rustc --explain E0275`.
error: could not compile `rustc_hir_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_hir_typeck` due to previous error
