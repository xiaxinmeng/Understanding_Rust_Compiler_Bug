
error: expected `{` or `if`, found `m`
  --> src/main.rs:18:16
   |
18 |         } else m == n { // should be else if here
   |                ^ expected `{`
   |
help: you might be missing an `if` keyword here
   |
18 |         } else if m == n { // should be else if here
   |                ++
