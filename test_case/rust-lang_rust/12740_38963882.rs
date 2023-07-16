 rust
pub enum ParserError {
    SyntaxError(SyntaxError),
    IoError(IoError),
}

pub enum BuilderError {
    ParseError(ParserError),
    // decoder errors
    ExpectedError(~str, ~str),
    MissingFieldError(~str),
    UnknownVariantError(~str)
}

pub enum JsonEvent {
    ListStart,
    ... // note: no Error event, that's handled by the Result
}

impl Parser {
    pub fn parse(&mut self, ...) -> Result<JsonEvent, ParserError> { ... }
}

impl Builder {
    pub fn build(&mut self, ...) -> Result<Json, BuilderError> { ... }
}
