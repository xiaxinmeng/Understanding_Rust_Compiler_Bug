
error: generic parameters may not be used in const operations
  --> src/main.rs:13:27
   |
13 |     data: [T; U::SIZE]
   |                           ^^^^^^^ cannot perform const operation using `U`
   |
   = note: type parameters may not be used in const expressions
