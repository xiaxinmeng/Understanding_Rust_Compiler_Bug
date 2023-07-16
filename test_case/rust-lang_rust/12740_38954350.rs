
// An error that can be generated from wrong json syntax during parsing
struct SyntaxError {
  line: u32, col: u32, reason: ErrorCode
}

// The output of the parser
enum JsonEvent {
  ListStart,
  // ...
  ParseError(SyntaxError),
  IoError(io::IoError),
}

// the Builder is implemented on top of the parser.
BuilderError {
  ParserError2(SyntaxError), // :(
  IoError2(JsonError),       // :(
}

// The decoder is implemented on top of the Builder
enum DecoderError {
  ParseError3(SyntaxError),     // :(
  IoError3(io::IoError),        // :(
  ExpectedError(~str, ~str),
  MissingFieldError(~str),
  UnknownVariantError(~str),
}
