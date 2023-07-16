
error: expected one of `,`, `:`, or `}`, found `.`
 --> src/lib.rs:8:14
  |
7 |         Some(a) if a.value == b {
  |                               - while parsing this struct
8 |             a.value = 1;
  |             -^ expected one of `,`, `:`, or `}`
  |             |
  |             help: try naming a field: `a:`

error: expected one of `.`, `=>`, `?`, or an operator, found `,`
 --> src/lib.rs:9:10
  |
9 |         },
  |          ^ expected one of `.`, `=>`, `?`, or an operator

error: could not compile `playground` due to 2 previous errors
