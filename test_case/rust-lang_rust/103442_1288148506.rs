
warning[[E0170]](https://doc.rust-lang.org/stable/error-index.html#E0170): pattern binding `A` is named the same as one of the variants of the type `Enum`
  --> src/main.rs:11:9
   |
11 |         A => println!("A"),
   |         ^ help: to match on the variant, qualify the path: `Enum::A`
   |
   = note: `#[warn(bindings_with_variant_name)]` on by default

warning[[E0170]](https://doc.rust-lang.org/stable/error-index.html#E0170): pattern binding `B` is named the same as one of the variants of the type `Enum`
  --> src/main.rs:12:9
   |
12 |         B => println!("B"),
   |         ^ help: to match on the variant, qualify the path: `Enum::B`

warning: unreachable pattern
  --> src/main.rs:12:9
   |
11 |         A => println!("A"),
   |         - matches any value
12 |         B => println!("B"),
   |         ^ unreachable pattern
   |
   = note: `#[warn(unreachable_patterns)]` on by default

warning: unreachable pattern
  --> src/main.rs:13:9
   |
11 |         A => println!("A"),
   |         - matches any value
12 |         B => println!("B"),
13 |         Foo => {},
   |         ^^^ unreachable pattern

warning: unused variable: `A`
  --> src/main.rs:11:9
   |
11 |         A => println!("A"),
   |         ^ help: if this is intentional, prefix it with an underscore: `_A`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `B`
  --> src/main.rs:12:9
   |
12 |         B => println!("B"),
   |         ^ help: if this is intentional, prefix it with an underscore: `_B`

warning: unused variable: `Foo`
  --> src/main.rs:13:9
   |
13 |         Foo => {},
   |         ^^^ help: if this is intentional, prefix it with an underscore: `_Foo`

warning: variants `A` and `C` are never constructed
 --> src/main.rs:2:5
  |
1 | enum Enum {
  |      ---- variants in this enum
2 |     A,
  |     ^
3 |     B,
4 |     C,
  |     ^
  |
  = note: `#[warn(dead_code)]` on by default

warning: variable `A` should have a snake case name
  --> src/main.rs:11:9
   |
11 |         A => println!("A"),
   |         ^ help: convert the identifier to snake case: `a`
   |
   = note: `#[warn(non_snake_case)]` on by default

warning: variable `B` should have a snake case name
  --> src/main.rs:12:9
   |
12 |         B => println!("B"),
   |         ^ help: convert the identifier to snake case: `b`

warning: variable `Foo` should have a snake case name
  --> src/main.rs:13:9
   |
13 |         Foo => {},
   |         ^^^ help: convert the identifier to snake case (notice the capitalization): `foo`

For more information about this error, try `rustc --explain E0170`.
warning: `playground` (bin "playground") generated 11 warnings
