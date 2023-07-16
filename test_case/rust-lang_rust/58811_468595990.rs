none
error: expected one of `>`, identifier, lifetime, or type, found `"aaa"`
 --> src/main.rs:2:22
  |
2 |     let a: Vec<&str, "aaa"> = Vec::new();
  |         --           ^^^^^ expected one of `>`, identifier, lifetime, or type here
  |         ||
  |         |help: use `=` if you meant to assign
  |         while parsing the type for `a`

error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `,`
 --> src/main.rs:2:20
  |
2 |     let a: Vec<&str, "aaa"> = Vec::new();
  |                    ^ expected one of 7 possible tokens here
