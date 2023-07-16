plain
    Checking url v2.1.1
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
error[E0599]: no method named `eq_unspanned` found for reference `&rustc_ast::token::Token` in the current scope
   --> src/tools/clippy/clippy_lints/src/utils/ast_utils.rs:544:41
    |
544 |         (Eq(_, lts), Eq(_, rts)) => lts.eq_unspanned(rts),
    |                                         ^^^^^^^^^^^^ method not found in `&rustc_ast::token::Token`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_lints`
