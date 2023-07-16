
error: unknown prefix on string literal: bar
 --> file.rs:x:y
  |
1 | foo!(bar"qux");
  |      ^^^ help: try using whitespace here: `bar "qux"`
  |
  = note: prefixed string literals are reserved for future use
