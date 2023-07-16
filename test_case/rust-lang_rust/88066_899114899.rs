plain
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0658]: `if let` guards are experimental
   --> compiler/rustc_ast/src/util/literal.rs:220:17
    |
220 |                 if let token::NtExpr(expr) | token::NtLiteral(expr) = &**nt =>
    |
    = note: see issue #51114 <https://github.com/rust-lang/rust/issues/51114> for more information
    = note: see issue #51114 <https://github.com/rust-lang/rust/issues/51114> for more information
    = help: add `#![feature(if_let_guard)]` to the crate attributes to enable
    = help: you can write `if matches!(<expr>, <pattern>)` instead of `if let <pattern> = <expr>`

error[E0658]: `if let` guards are experimental
   --> compiler/rustc_ast/src/attr/mod.rs:568:17
    |
568 |                 if let Ok(lit) = Lit::from_token(token) =>
    |
    = note: see issue #51114 <https://github.com/rust-lang/rust/issues/51114> for more information
    = note: see issue #51114 <https://github.com/rust-lang/rust/issues/51114> for more information
    = help: add `#![feature(if_let_guard)]` to the crate attributes to enable
    = help: you can write `if matches!(<expr>, <pattern>)` instead of `if let <pattern> = <expr>`

error[E0308]: `match` arms have incompatible types
   --> compiler/rustc_ast/src/util/literal.rs:222:17
    |
214 |           let lit = match token.uninterpolate().kind {
    |                     -------------------------------- `match` arms have incompatible types
215 |               token::Ident(name, false) if name.is_bool_lit() => {
216 |                   token::Lit::new(token::Bool, name, None)
    |                   ---------------------------------------- this is found to be of type `token::Lit`
217 |               }
218 |               token::Literal(lit) => lit,
    |                                      --- this is found to be of type `token::Lit`
...
222 | /                 if let ast::ExprKind::Lit(lit) = &expr.kind {
223 | |                     return Ok(lit.clone());
224 | |                 }
    | |_________________^ expected struct `token::Lit`, found `()`
    Checking tracing-serde v0.1.2
    Checking gsgdt v0.1.2
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
