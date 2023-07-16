rust
error: expected one of `->`, `;`, `where`, or `{`, found `}`
 --> <anon>:5:1
  |
4 |     fn foo()
  |        ---  - expected one of `->`, `;`, `where`, or `{`
  |        |
  |        while parsing this `fn`
5 | }
  | ^ unexpected token

error: associated function in `impl` without body
 --> <anon>:4:5
  |
4 |     fn foo()
  |     ^^^^^^^^- help: provide a definition for the function: `{ <body> }`

error: aborting due to 2 previous errors
