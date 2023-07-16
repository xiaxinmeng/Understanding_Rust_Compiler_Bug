
$ PATH=/home/ajeffrey/github/rust-lang/rust/x86_64-unknown-linux-gnu/stage2/bin/:$PATH cargo build
   Compiling wasm_sexpr v0.0.1 (file:///home/ajeffrey/github/asajeffrey/wasm/wasm-sexpr)
src/lexer.rs:194:26: 194:40 error: type mismatch resolving `for<'a> <fn(core::option::Option<char>) -> core::result::Result<lexer::Token<'a>, lexer::LexError> {lexer::mk_unexpected_char_err} as parsell::Function<core::option::Option<char>>>::Output == core::result::Result<lexer::Token<'a>, lexer::LexError>`:
 expected bound lifetime parameter 'a,
    found concrete lifetime [E0271]
src/lexer.rs:194         WASM_TOKEN.boxed(mk_lexer_state).init(data)
                                          ^~~~~~~~~~~~~~
src/lexer.rs:194:26: 194:40 help: run `rustc --explain E0271` to see a detailed explanation
src/lexer.rs:194:26: 194:40 note: required by `lexer::mk_lexer_state`
error: aborting due to previous error
Could not compile `wasm_sexpr`.
