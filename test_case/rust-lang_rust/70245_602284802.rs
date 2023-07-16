rust
enum ErrorReported {
  /// Emitted something with level < error
  Emitted,
  /// Emitted something with that will fail compilation
  EmittedWithProof(ErrorProof),
}

impl DiagnosticBuilder {
  pub fn emit(...) -> ErrorReported { ... }
}
