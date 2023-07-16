
error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found keyword `if`
 --> src/main.rs:4:5
  |
3 |     and
  |        - expected one of 8 possible tokens
4 |     if let Some(y) = a { true } else { false }
  |     ^^ unexpected token

error[E0308]: mismatched types
 --> src/main.rs:2:26
  |
2 |     if let Some(x) = a { true } else { false }
  |     ---------------------^^^^------------------ help: consider using a semicolon here
  |     |                    |
  |     |                    expected `()`, found `bool`
  |     expected this to be `()`

error[E0308]: mismatched types
 --> src/main.rs:2:40
  |
2 |     if let Some(x) = a { true } else { false }
  |     -----------------------------------^^^^^--- help: consider using a semicolon here
  |     |                                  |
  |     |                                  expected `()`, found `bool`
  |     expected this to be `()`
