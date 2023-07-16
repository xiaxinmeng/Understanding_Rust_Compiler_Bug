plain
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
error[E0277]: `RefCell<Vec<u8>>` cannot be shared between threads safely
     |
54   |     Token: Send + Sync,
54   |     Token: Send + Sync,
     |     ^^^^^^^^^^^^^^^^^^ `RefCell<Vec<u8>>` cannot be shared between threads safely
     |
     = help: within `ast::Expr`, the trait `Sync` is not implemented for `RefCell<Vec<u8>>`
note: required because it appears within the type `ast::ExprKind`
     |
1306 | pub enum ExprKind {
     |          ^^^^^^^^
note: required because it appears within the type `ast::Expr`
note: required because it appears within the type `ast::Expr`
    --> compiler/rustc_ast/src/ast.rs:1107:12
     |
1107 | pub struct Expr {
     |            ^^^^
     = note: required for `Unique<ast::Expr>` to implement `Sync`
     = note: required because it appears within the type `std::boxed::Box<ast::Expr>`
note: required because it appears within the type `P<ast::Expr>`
     |
     |
33   | pub struct P<T: ?Sized> {
note: required because it appears within the type `token::Nonterminal`
    --> compiler/rustc_ast/src/token.rs:732:10
     |
     |
732  | pub enum Nonterminal {
     = note: required for `Arc<token::Nonterminal>` to implement `Sync`
note: required because it appears within the type `token::TokenKind`
    --> compiler/rustc_ast/src/token.rs:187:10
     |
---
     |
260  | pub struct Token {
     |            ^^^^^
     = help: see issue #48214
     = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable

error[E0277]: `RefCell<Vec<u8>>` cannot be shared between threads safely
     |
     |
124  | impl ToAttrTokenStream for AttrTokenStream {
     |      ^^^^^^^^^^^^^^^^^ `RefCell<Vec<u8>>` cannot be shared between threads safely
     |
     = help: within `ast::Expr`, the trait `Sync` is not implemented for `RefCell<Vec<u8>>`
note: required because it appears within the type `ast::ExprKind`
     |
1306 | pub enum ExprKind {
     |          ^^^^^^^^
note: required because it appears within the type `ast::Expr`
note: required because it appears within the type `ast::Expr`
    --> compiler/rustc_ast/src/ast.rs:1107:12
     |
1107 | pub struct Expr {
     |            ^^^^
     = note: required for `Unique<ast::Expr>` to implement `Sync`
     = note: required because it appears within the type `std::boxed::Box<ast::Expr>`
note: required because it appears within the type `P<ast::Expr>`
     |
     |
33   | pub struct P<T: ?Sized> {
note: required because it appears within the type `token::Nonterminal`
    --> compiler/rustc_ast/src/token.rs:732:10
     |
     |
732  | pub enum Nonterminal {
     = note: required for `Arc<token::Nonterminal>` to implement `Sync`
note: required because it appears within the type `token::TokenKind`
    --> compiler/rustc_ast/src/token.rs:187:10
     |
---
     |            ^^^^^
note: required because it appears within the type `tokenstream::AttrTokenTree`
    --> compiler/rustc_ast/src/tokenstream.rs:180:10
     |
180  | pub enum AttrTokenTree {
     |          ^^^^^^^^^^^^^
     = note: required for `Unique<tokenstream::AttrTokenTree>` to implement `Sync`
     = note: required because it appears within the type `smallvec::alloc::raw_vec::RawVec<tokenstream::AttrTokenTree>`
     = note: required because it appears within the type `Vec<tokenstream::AttrTokenTree>`
     = note: required for `Arc<Vec<tokenstream::AttrTokenTree>>` to implement `Sync`
note: required because it appears within the type `tokenstream::AttrTokenStream`
     |
     |
176  | pub struct AttrTokenStream(pub Lrc<Vec<AttrTokenTree>>);
note: required by a bound in `tokenstream::ToAttrTokenStream`
    --> compiler/rustc_ast/src/tokenstream.rs:120:43
     |
     |
120  | pub trait ToAttrTokenStream: sync::Send + sync::Sync {
     |                                           ^^^^^^^^^^ required by this bound in `tokenstream::ToAttrTokenStream`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_ast` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_ast` due to 2 previous errors
