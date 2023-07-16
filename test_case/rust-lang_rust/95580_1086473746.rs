plain
    Checking structopt v0.3.25
error[E0063]: missing field `span2` in initializer of `rustc_ast::token::Token`
   --> src/tools/rustfmt/src/macros.rs:686:55
    |
686 |         TokenTree::Delimited(delim_span, delim, _) => Token {
    |                                                       ^^^^^ missing `span2`
error[E0063]: missing field `span2` in initializer of `rustc_ast::token::Token`
   --> src/tools/rustfmt/src/macros.rs:698:23
    |
    |
698 |             last_tok: Token {
    |                       ^^^^^ missing `span2`
error[E0063]: missing field `span2` in initializer of `rustc_ast::token::Token`
   --> src/tools/rustfmt/src/macros.rs:702:24
    |
    |
702 |             start_tok: Token {
    |                        ^^^^^ missing `span2`
error[E0027]: pattern does not mention field `span2`
   --> src/tools/rustfmt/src/macros.rs:862:34
    |
    |
862 |                   TokenTree::Token(Token {
863 | |                     kind: TokenKind::Dollar,
864 | |                     span,
865 | |                 }) => {
    | |_________________^ missing field `span2`
    | |_________________^ missing field `span2`
    |
help: include the missing field in the pattern
    |
864 |                     span, span2 }) => {
    |                         ~~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
864 |                     span, .. }) => {
    |                         ~~~~~~

error[E0063]: missing field `span2` in initializer of `rustc_ast::token::Token`
---

error[E0027]: pattern does not mention field `span2`
    --> src/tools/rustfmt/src/macros.rs:1150:38
     |
1150 |           if let Some(TokenTree::Token(Token {
1151 | |             kind: TokenKind::Semi,
1152 | |             span,
1152 | |             span,
1153 | |         })) = self.toks.look_ahead(0)
     | |_________^ missing field `span2`
help: include the missing field in the pattern
     |
     |
1152 |             span, span2 })) = self.toks.look_ahead(0)
     |                 ~~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
     |
1152 |             span, .. })) = self.toks.look_ahead(0)

Some errors have detailed explanations: E0027, E0063.
For more information about an error, try `rustc --explain E0027`.
error: could not compile `rustfmt-nightly` due to 6 previous errors
