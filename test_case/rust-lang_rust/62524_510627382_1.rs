
=========================
(BytePos(10), BytePos(11))

  15: syntax_pos::span_encoding::Span::new at /home/lampam/asd/clone/rust/src/libsyntax_pos/span_encoding.rs:81
  16: syntax_pos::SpanData::with_lo at /home/lampam/asd/clone/rust/src/libsyntax_pos/lib.rs:218
  17: syntax_pos::<impl syntax_pos::span_encoding::Span>::with_lo at /home/lampam/asd/clone/rust/src/libsyntax_pos/lib.rs:268
  18: syntax::tokenstream::TokenTree::close_tt at src/libsyntax/tokenstream.rs:141
  19: syntax::parse::parser::TokenCursor::next at src/libsyntax/parse/parser.rs:315
  20: syntax::parse::parser::Parser::next_tok at src/libsyntax/parse/parser.rs:528
  21: syntax::parse::parser::Parser::bump at src/libsyntax/parse/parser.rs:1031
  22: syntax::parse::parser::Parser::parse_ident_common at src/libsyntax/parse/parser.rs:632
  23: syntax::parse::parser::Parser::parse_ident at src/libsyntax/parse/parser.rs:617
  24: syntax::parse::parser::Parser::parse_pat_ident at src/libsyntax/parse/parser.rs:4132
  25: syntax::parse::parser::Parser::parse_pat_with_range_pat at src/libsyntax/parse/parser.rs:3987
  26: syntax::parse::parser::Parser::parse_pat at src/libsyntax/parse/parser.rs:3908
  27: syntax::parse::parser::Parser::parse_pat_list at src/libsyntax/parse/parser.rs:3582
  28: syntax::parse::parser::Parser::parse_parenthesized_pat_list at src/libsyntax/parse/parser.rs:3547
  29: syntax::parse::parser::Parser::parse_pat_with_range_pat at src/libsyntax/parse/parser.rs:3938
  30: syntax::parse::parser::Parser::parse_pat at src/libsyntax/parse/parser.rs:3908
  31: syntax::parse::parser::Parser::parse_arg_general at src/libsyntax/parse/parser.rs:1510
  32: syntax::parse::parser::Parser::parse_fn_args::{{closure}} at src/libsyntax/parse/parser.rs:5380
  33: syntax::parse::parser::Parser::parse_seq_to_before_tokens at src/libsyntax/parse/parser.rs:983
  34: syntax::parse::parser::Parser::parse_seq_to_before_end at src/libsyntax/parse/parser.rs:916
  35: syntax::parse::parser::Parser::parse_fn_args at src/libsyntax/parse/parser.rs:5368
  36: syntax::parse::parser::Parser::parse_fn_decl at src/libsyntax/parse/parser.rs:5429
  37: syntax::parse::parser::Parser::parse_item_fn at src/libsyntax/parse/parser.rs:5660
