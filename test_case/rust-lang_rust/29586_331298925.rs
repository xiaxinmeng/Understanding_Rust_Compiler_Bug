
error: expected one of `!`, `,`, `.`, `::`, `?`, `{`, `}`, or an operator, found `=>`
 --> src/main.rs:6:17
  |
5 |         &Foo::A => "art"
  |                    - in the match arm starting here...
6 |         &Foo::B => "bart"
  |                 ^^ expected one of 8 possible tokens here
  = note: is a comma missing here?
  |
5 |         &Foo::A => "art",
  |                         ^
