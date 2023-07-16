  14: core::panicking::panic_fmt
             at src/libcore/panicking.rs:111
  15: core::str::slice_error_fail
             at src/libcore/str/mod.rs:0
  16: core::str::traits::<impl core::slice::SliceIndex<str> for core::ops::range::Range<usize>>::index::{{closure}}
             at ./src/libcore/str/mod.rs:1920
  17: rustc_parse::lexer::StringReader::cook_lexer_token
             at src/librustc_parse/lexer/mod.rs:0
  18: rustc_parse::lexer::StringReader::next_token
             at src/librustc_parse/lexer/mod.rs:133
  19: rustc_parse::lexer::tokentrees::TokenTreesReader::real_token
             at src/librustc_parse/lexer/tokentrees.rs:248
  20: rustc_parse::lexer::tokentrees::TokenTreesReader::parse_all_token_trees
             at src/librustc_parse/lexer/tokentrees.rs:52
  21: rustc_parse::lexer::tokentrees::<impl rustc_parse::lexer::StringReader>::into_token_trees
             at src/librustc_parse/lexer/tokentrees.rs:26
  22: rustc_parse::maybe_file_to_stream
             at src/librustc_parse/lib.rs:202
  23: rustc_parse::maybe_source_file_to_parser
             at src/librustc_parse/lib.rs:137
  24: rustc_parse::source_file_to_parser
             at src/librustc_parse/lib.rs:127
  25: rustc_parse::new_parser_from_file
             at src/librustc_parse/lib.rs:112
