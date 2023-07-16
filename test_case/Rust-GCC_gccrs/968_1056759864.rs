
MacroMatch :
      [Token](https://doc.rust-lang.org/reference/tokens.html)except $ and delimiters
   | MacroMatcher
   | $ [IDENTIFIER](https://doc.rust-lang.org/reference/identifiers.html) : MacroFragSpec
   | $ ( MacroMatch+ ) MacroRepSep? MacroRepOp
