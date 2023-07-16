
error: expected `,`, or `}`, found `b`
 --> src/main.rs:2:11
  |
2 |     a: i32
  |           ^ help: try adding a comma: `,`

error: expected identifier, found `,`
 --> src/main.rs:7:12
  |
7 |     a: i32,,
  |            ^
  |            |
  |            expected identifier
  |            help: remove this comma

error: expected one of `,`, `.`, `?`, `}`, or an operator, found `b`
  --> src/main.rs:14:9
   |
13 |         a: 42
   |              - 
   |              |
   |              expected one of `,`, `.`, `?`, `}`, or an operator here
   |              help: add a comma here: `,`
14 |         b: 3,
   |         ^ unexpected token

error: expected identifier, found `,`
  --> src/main.rs:17:15
   |
16 |     let y = Bar {
   |             --- while parsing this struct
17 |         a: 42,,
   |               ^
   |               |
   |               expected identifier
   |               help: remove this comma
