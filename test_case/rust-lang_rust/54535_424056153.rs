diff
  enum TokenTree {
      Token(Span, Token),
-     Delimited(DelimSpan, Delimited),
+     Delimited(DelimSpan, DelimToken, ThinTokenStream),
  }
-
- struct Delimited {
-     delim: DelimToken,
-     tts: ThinTokenStream,
- }
