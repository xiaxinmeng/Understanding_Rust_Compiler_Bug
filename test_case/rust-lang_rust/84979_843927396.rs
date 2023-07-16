rust
// rustc_lexer
pub struct LexerConfig {
    allow_string_prefixes: bool,
}

/// Parses the first token from the provided input string.
pub fn first_token(config: &LexerConfig, input: &str) -> Token {
    debug_assert!(!input.is_empty());
    Cursor::new(input).advance_token()
}

// Call-site in rustc/rust-analyzer
let lexer_config = if edition >= 2021 {
    LexerConfig { allow_string_prefixes: true }
} else {
    LexerConfig { allow_string_prefixes: false }
}
first_token(&lexer_config, token_text);
