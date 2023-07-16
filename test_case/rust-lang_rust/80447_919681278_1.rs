
   Compiling playground v0.0.1 (/playground)
error: expected one of `)` or `,`, found `{`
 --> src/lib.rs:7:10
  |
7 |     fn ox(&mut self {
  |          ^         -^
  |          |         |
  |          |         help: `)` may belong here
  |          unclosed delimiter

error: non-item in item list
 --> src/lib.rs:9:1
  |
5 | impl al {
  |         - item list starts here
...
9 | }
  | ^
  | |
  | non-item starts here
  | item list ends here

thread 'rustc' panicked at 'Stack should be empty: final_buf=FrameData { open: src/lib.rs:7:10: 7:11 (#0), open_delim: Paren, inner: [(Token(Token { kind: BinOp(And), span: src/lib.rs:7:11: 7:12 (#0) }), Alone), (Token(Token { kind: Ident("mut", false), span: src/lib.rs:7:12: 7:15 (#0) }), Alone), (Token(Token { kind: Ident("self", false), span: src/lib.rs:7:16: 7:20 (#0) }), Alone), (Delimited(DelimSpan { open: src/lib.rs:7:21: 7:22 (#0), close: src/lib.rs:8:5: 8:6 (#0) }, Brace, AttrAnnotatedTokenStream([])), Alone)] } stack=[FrameData { open: no-location (#0), open_delim: NoDelim, inner: [(Token(Token { kind: Ident("fn", false), span: src/lib.rs:7:5: 7:7 (#0) }), Alone), (Token(Token { kind: Ident("ox", false), span: src/lib.rs:7:8: 7:10 (#0) }), Alone)] }]', compiler/rustc_parse/src/parser/attr_wrapper.rs:501:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/c3c0f80d6081092faff801542dd82f0e2420152b/library/std/src/panicking.rs:517:5
   1: std::panicking::begin_panic_fmt
             at /rustc/c3c0f80d6081092faff801542dd82f0e2420152b/library/std/src/panicking.rs:460:5
   2: <rustc_parse::parser::attr_wrapper::LazyTokenStreamImpl as rustc_ast::tokenstream::CreateTokenStream>::create_token_stream
   3: rustc_parse::prepend_attrs
   4: rustc_parse::nt_to_tokenstream
   5: rustc_expand::base::Annotatable::into_tokens
   6: rustc_expand::expand::MacroExpander::fully_expand_fragment
   7: rustc_expand::expand::MacroExpander::expand_crate
   8: rustc_session::utils::<impl rustc_session::session::Session>::time
   9: rustc_interface::passes::configure_and_expand
  10: rustc_interface::queries::Queries::expansion
  11: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  12: rustc_span::with_source_map
  13: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (c3c0f80d6 2021-09-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
