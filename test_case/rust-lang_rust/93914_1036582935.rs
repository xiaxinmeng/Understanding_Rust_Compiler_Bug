
error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `,`
 --> <source>:2:30
  |
2 |     let counts = HashMap<char, i32> = HashMap.new()
  |                              ^ expected one of 8 possible tokens
  |
help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
  |
2 |     let counts = HashMap::<char, i32> = HashMap.new()
  |                         ++

error: aborting due to previous error
