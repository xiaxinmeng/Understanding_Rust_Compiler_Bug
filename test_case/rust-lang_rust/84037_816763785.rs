plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0423]: expected value, found struct variant `TokenKind::Ident`
   --> src/librustdoc/html/highlight.rs:191:30
    |
191 |                 return vec![(TokenKind::Ident, start, pos)];
    | 
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:67:5
    |
    |
67  |     Ident { followed_by: char },
    |     ----- `TokenKind::Ident` defined here
help: use struct literal syntax instead
    |
    |
191 |                 return vec![(TokenKind::Ident { /* fields */ }, start, pos)];
help: consider importing one of these items instead
    |
8   | use crate::html::highlight::Class::Ident;
    |
    |
8   | use rustc_ast::PatKind::Ident;
    |
8   | use rustc_ast::token::Ident;
    |
8   | use rustc_ast::token::NonterminalKind::Ident;
      and 1 other candidate


error[E0423]: expected value, found struct variant `TokenKind::Ident`
   --> src/librustdoc/html/highlight.rs:194:34
    |
194 |                     return vec![(TokenKind::Ident, start, pos), (TokenKind::Colon, pos, pos + nb)];
    | 
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:67:5
    |
    |
67  |     Ident { followed_by: char },
    |     ----- `TokenKind::Ident` defined here
help: use struct literal syntax instead
    |
    |
194 |                     return vec![(TokenKind::Ident { /* fields */ }, start, pos), (TokenKind::Colon, pos, pos + nb)];
help: consider importing one of these items instead
    |
8   | use crate::html::highlight::Class::Ident;
    |
    |
8   | use rustc_ast::PatKind::Ident;
    |
8   | use rustc_ast::token::Ident;
    |
8   | use rustc_ast::token::NonterminalKind::Ident;
      and 1 other candidate


error[E0423]: expected value, found struct variant `TokenKind::Ident`
   --> src/librustdoc/html/highlight.rs:201:30
    |
201 |                 if *token == TokenKind::Ident {
    |                              ^^^^^^^^^^^^^^^^ you might want to surround a struct literal with parentheses: `(TokenKind::Ident { /* fields */ })`?
help: consider importing one of these items instead
    |
8   | use crate::html::highlight::Class::Ident;
    |
    |
8   | use rustc_ast::PatKind::Ident;
    |
8   | use rustc_ast::token::Ident;
    |
8   | use rustc_ast::token::NonterminalKind::Ident;
      and 1 other candidate


error[E0423]: expected value, found struct variant `TokenKind::Ident`
   --> src/librustdoc/html/highlight.rs:214:30
    |
214 |                 return vec![(TokenKind::Ident, start, pos), (TokenKind::Colon, pos, pos + nb)];
    | 
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:67:5
    |
    |
67  |     Ident { followed_by: char },
    |     ----- `TokenKind::Ident` defined here
help: use struct literal syntax instead
    |
    |
214 |                 return vec![(TokenKind::Ident { /* fields */ }, start, pos), (TokenKind::Colon, pos, pos + nb)];
help: consider importing one of these items instead
    |
8   | use crate::html::highlight::Class::Ident;
    |
    |
8   | use rustc_ast::PatKind::Ident;
    |
8   | use rustc_ast::token::Ident;
    |
8   | use rustc_ast::token::NonterminalKind::Ident;
      and 1 other candidate


error[E0423]: expected value, found struct variant `TokenKind::Ident`
   --> src/librustdoc/html/highlight.rs:218:30
    |
218 |                 return vec![(TokenKind::Ident, start, pos)];
    | 
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:67:5
    |
    |
67  |     Ident { followed_by: char },
    |     ----- `TokenKind::Ident` defined here
help: use struct literal syntax instead
    |
    |
218 |                 return vec![(TokenKind::Ident { /* fields */ }, start, pos)];
help: consider importing one of these items instead
    |
8   | use crate::html::highlight::Class::Ident;
    |
    |
8   | use rustc_ast::PatKind::Ident;
    |
8   | use rustc_ast::token::Ident;
    |
8   | use rustc_ast::token::NonterminalKind::Ident;
      and 1 other candidate


error[E0532]: expected unit struct, unit variant or constant, found struct variant `TokenKind::Ident`
   --> src/librustdoc/html/highlight.rs:246:63
    |
246 |                     .map(|t| matches!(t.0, TokenKind::Colon | TokenKind::Ident))
    | 
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:67:5
    |
    |
67  |     Ident { followed_by: char },
    |     ----- `TokenKind::Ident` defined here
help: use struct pattern syntax instead
    |
    |
246 |                     .map(|t| matches!(t.0, TokenKind::Colon | TokenKind::Ident { /* fields */ }))
help: consider importing one of these items instead
    |
8   | use crate::html::highlight::Class::Ident;
    |
    |
8   | use rustc_ast::token::NonterminalKind::Ident;


error[E0532]: expected unit struct, unit variant or constant, found struct variant `TokenKind::Ident`
   --> src/librustdoc/html/highlight.rs:339:22
339 |                 Some(TokenKind::Ident) => {
    |                      ^^^^^^^^^^^^^^^^
    | 
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:67:5
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:67:5
    |
67  |     Ident { followed_by: char },
    |     ----- `TokenKind::Ident` defined here
help: use struct pattern syntax instead
    |
    |
339 |                 Some(TokenKind::Ident { /* fields */ }) => {
help: consider importing one of these items instead
    |
8   | use crate::html::highlight::Class::Ident;
    |
    |
8   | use rustc_ast::token::NonterminalKind::Ident;


error[E0532]: expected unit struct, unit variant or constant, found struct variant `TokenKind::Ident`
   --> src/librustdoc/html/highlight.rs:392:13
    |
392 |             TokenKind::Ident | TokenKind::RawIdent if lookahead == Some(TokenKind::Bang) => {
    | 
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:67:5
    |
    |
67  |     Ident { followed_by: char },
    |     ----- `TokenKind::Ident` defined here
help: use struct pattern syntax instead
    |
    |
392 |             TokenKind::Ident { /* fields */ } | TokenKind::RawIdent if lookahead == Some(TokenKind::Bang) => {
help: consider importing one of these items instead
    |
8   | use crate::html::highlight::Class::Ident;
    |
    |
8   | use rustc_ast::token::NonterminalKind::Ident;


error[E0532]: expected unit struct, unit variant or constant, found struct variant `TokenKind::RawIdent`
   --> src/librustdoc/html/highlight.rs:392:32
    |
392 |             TokenKind::Ident | TokenKind::RawIdent if lookahead == Some(TokenKind::Bang) => {
    |                                ^^^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `TokenKind::RawIdent { /* fields */ }`
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:69:5
    |
    |
69  |     RawIdent { followed_by: char },
    |     -------- `TokenKind::RawIdent` defined here

error[E0532]: expected unit struct, unit variant or constant, found struct variant `TokenKind::Ident`
   --> src/librustdoc/html/highlight.rs:398:13
    |
398 |             TokenKind::Ident => match get_real_ident_class(text, self.edition) {
    | 
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:67:5
    |
    |
67  |     Ident { followed_by: char },
    |     ----- `TokenKind::Ident` defined here
help: use struct pattern syntax instead
    |
    |
398 |             TokenKind::Ident { /* fields */ } => match get_real_ident_class(text, self.edition) {
help: consider importing one of these items instead
    |
8   | use crate::html::highlight::Class::Ident;
    |
    |
8   | use rustc_ast::token::NonterminalKind::Ident;


error[E0532]: expected unit struct, unit variant or constant, found struct variant `TokenKind::RawIdent`
   --> src/librustdoc/html/highlight.rs:410:13
    |
410 |             TokenKind::RawIdent => Class::Ident,
    |             ^^^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `TokenKind::RawIdent { /* fields */ }`
   ::: /checkout/compiler/rustc_lexer/src/lib.rs:69:5
    |
    |
69  |     RawIdent { followed_by: char },
    |     -------- `TokenKind::RawIdent` defined here
error: aborting due to 11 previous errors

Some errors have detailed explanations: E0423, E0532.
For more information about an error, try `rustc --explain E0423`.
