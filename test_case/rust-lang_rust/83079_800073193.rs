rust
enum EscapeChars {
    EscapeSingleQuote,
    EscapeDoubleQuote,
    EscapeBothQuotes,
}

impl EscapeChars {
    fn escape_single_quote(&self) -> bool {
        match self {
            ...
        }
    }

    fn escape_double_quote(&self) -> bool {
        match self {
            ...
        }
    }
}
