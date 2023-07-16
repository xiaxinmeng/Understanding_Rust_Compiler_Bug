plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
   Compiling log v0.4.14
   Compiling anyhow v1.0.65
    Checking clippy_utils v0.1.68 (/checkout/src/tools/clippy/clippy_utils)
    Checking rand_core v0.6.4
error[E0432]: unresolved import `rustc_ast::util::parser::AssocOp::Colon`
   --> src/tools/clippy/clippy_utils/src/sugg.rs:612:59
    |
612 |         Add, As, Assign, AssignOp, BitAnd, BitOr, BitXor, Colon, Divide, DotDot, DotDotEq, Equal, Greater,
    |                                                           ^^^^^ no `Colon` in `util::parser::AssocOp`
help: consider importing one of these items instead
    |
    |
612 |         Add, As, Assign, AssignOp, BitAnd, BitOr, BitXor, crate::TokenKind::Colon;
    |                                                           ~~~~~~~~~~~~~~~~~~~~~~~~
612 |         Add, As, Assign, AssignOp, BitAnd, BitOr, BitXor, crate::sugg::token::Colon;
    |                                                           ~~~~~~~~~~~~~~~~~~~~~~~~~~
612 |         Add, As, Assign, AssignOp, BitAnd, BitOr, BitXor, rustc_ast::token::Colon;
    |                                                           ~~~~~~~~~~~~~~~~~~~~~~~~
612 |         Add, As, Assign, AssignOp, BitAnd, BitOr, BitXor, rustc_lexer::TokenKind::Colon;

error[E0408]: variable `Colon` is not bound in all patterns
   --> src/tools/clippy/clippy_utils/src/sugg.rs:618:9
    |
    |
618 |         Add | BitAnd | BitOr | BitXor | LAnd | LOr | Multiply | As | Colon => Associativity::Both,
    |         ^^^   ^^^^^^   ^^^^^   ^^^^^^   ^^^^   ^^^   ^^^^^^^^   ^^   ----- variable not in all patterns
    |         |     |        |       |        |      |     |          pattern doesn't bind `Colon`
    |         |     |        |       |        |      |     pattern doesn't bind `Colon`
    |         |     |        |       |        |      pattern doesn't bind `Colon`
    |         |     |        |       |        pattern doesn't bind `Colon`
    |         |     |        |       |        pattern doesn't bind `Colon`
    |         |     |        |       pattern doesn't bind `Colon`
    |         |     |        pattern doesn't bind `Colon`
    |         |     pattern doesn't bind `Colon`
    |         pattern doesn't bind `Colon`
    |
help: if you meant to match on items, use the full path in the pattern
    |
618 |         Add | BitAnd | BitOr | BitXor | LAnd | LOr | Multiply | As | crate::TokenKind::Colon => Associativity::Both,
    |                                                                      ~~~~~~~~~~~~~~~~~~~~~~~
618 |         Add | BitAnd | BitOr | BitXor | LAnd | LOr | Multiply | As | crate::sugg::token::Colon => Associativity::Both,
    |                                                                      ~~~~~~~~~~~~~~~~~~~~~~~~~
618 |         Add | BitAnd | BitOr | BitXor | LAnd | LOr | Multiply | As | rustc_ast::token::Colon => Associativity::Both,
    |                                                                      ~~~~~~~~~~~~~~~~~~~~~~~
618 |         Add | BitAnd | BitOr | BitXor | LAnd | LOr | Multiply | As | rustc_lexer::TokenKind::Colon => Associativity::Both,

   Compiling clippy v0.1.68 (/checkout/src/tools/clippy)
   Compiling parking_lot_core v0.9.4
    Checking unicode-script v0.5.5
---
    Checking rand v0.8.5
    Checking bytes v1.0.1
    Checking diff v0.1.13
    Checking bstr v0.2.17
error[E0599]: no variant or associated item named `Colon` found for enum `AssocOp` in the current scope
   --> src/tools/clippy/clippy_utils/src/sugg.rs:174:66
    |
174 |             hir::ExprKind::Type(lhs, ty) => Sugg::BinOp(AssocOp::Colon, get_snippet(lhs.span), get_snippet(ty.span)),
    |                                                                  ^^^^^ variant or associated item not found in `AssocOp`

error[E0599]: no variant or associated item named `Colon` found for enum `AssocOp` in the current scope
   --> src/tools/clippy/clippy_utils/src/sugg.rs:269:26
    |
269 |                 AssocOp::Colon,
    |                          ^^^^^ variant or associated item not found in `AssocOp`

error[E0599]: no variant or associated item named `Colon` found for enum `AssocOp` in the current scope
   --> src/tools/clippy/clippy_utils/src/sugg.rs:402:18
    |
402 |         AssocOp::Colon => format!("{lhs}: {rhs}"),
    |                  ^^^^^ variant or associated item not found in `AssocOp`
   Compiling libnghttp2-sys v0.1.4+1.41.0
   Compiling libz-sys v1.1.3
   Compiling curl-sys v0.4.59+curl-7.86.0
    Checking idna v0.2.0
