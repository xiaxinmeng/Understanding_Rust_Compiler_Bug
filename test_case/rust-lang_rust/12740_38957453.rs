
enum ParserError {
    SyntaxError(SyntaxError),
    IoError(IoError),
}

enum JsonEvent {
    ListStart,
    ...
    Error(JsonError),
}

impl Parser {
    fn parse(&mut self, ...) -> Result<JsonEvent, JsonError> { ... }
}

impl Builder {
    fn build(&mut self, ...) -> Result<Json, JsonError> { ... }
}

enum DecoderError {
    ParserError(JsonError),
    ExpectedError(~str, ~str),
    ...
}
