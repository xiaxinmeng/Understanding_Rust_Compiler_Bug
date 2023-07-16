plain
    Checking structopt v0.3.25
error[E0532]: expected unit struct, unit variant or constant, found struct variant `Delimiter::Invisible`
   --> src/tools/rustfmt/src/macros.rs:565:9
    |
565 |         Delimiter::Invisible => unreachable!(),
    |         ^^^^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `Delimiter::Invisible { /* fields */ }`
   ::: /checkout/compiler/rustc_ast/src/token.rs:61:5
    |
    |
61  |     Invisible { skip: bool },
    |     --------- `Delimiter::Invisible` defined here
error[E0423]: expected value, found struct variant `Delimiter::Invisible`
  --> src/tools/rustfmt/src/parse/macros/mod.rs:84:56
   |
   |
84 |                     || t.kind == TokenKind::CloseDelim(Delimiter::Invisible)
   |                                                        ^^^^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `Delimiter::Invisible { /* fields */ }`
  ::: /checkout/compiler/rustc_ast/src/token.rs:61:5
   |
   |
61 |     Invisible { skip: bool },
   |     --------- `Delimiter::Invisible` defined here
Some errors have detailed explanations: E0423, E0532.
For more information about an error, try `rustc --explain E0423`.
error: could not compile `rustfmt-nightly` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
