
error: expected identifier, found keyword `return`
  --> src/main.rs:19:13
   |
18 |         } else m == n { // should be else if here
   |                     - while parsing this struct
19 |             return Some(x);
   |             ^^^^^^ expected identifier, found keyword

error: expected `{`, found `m`
  --> src/main.rs:18:16
   |
18 |         } else m == n { // should be else if here
   |                ^ expected `{`
   |
help: try placing this code inside a block
   |
18 ~         } else { m == n { // should be else if here
19 |             return Some(x);
20 ~         } }
   |
