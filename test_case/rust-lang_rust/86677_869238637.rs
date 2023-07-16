rust
warning[E0170]: pattern binding `A` is named the same as one of the variants of the type `Foo`
  --> src/main.rs:10:9
   |
10 |         A => println!("A"),
   |         ^ help: to match on the variant, qualify the path: `Foo::A`
   |
   = note: `#[warn(bindings_with_variant_name)]` on by default

warning[E0170]: pattern binding `B` is named the same as one of the variants of the type `Foo`
  --> src/main.rs:11:9
   |
11 |         B => println!("B"),
   |         ^ help: to match on the variant, qualify the path: `Foo::B`

warning: unreachable pattern
  --> src/main.rs:11:9
   |
10 |         A => println!("A"),
   |         - matches any value
11 |         B => println!("B"),
   |         ^ unreachable pattern
   |
   = note: `#[warn(unreachable_patterns)]` on by default

warning: unused variable: `A`
  --> src/main.rs:10:9
   |
10 |         A => println!("A"),
   |         ^ help: if this is intentional, prefix it with an underscore: `_A`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `B`
  --> src/main.rs:11:9
   |
11 |         B => println!("B"),
   |         ^ help: if this is intentional, prefix it with an underscore: `_B`

warning: variant is never constructed: `A`
 --> src/main.rs:2:5
  |
2 |     A,
  |     ^
  |
  = note: `#[warn(dead_code)]` on by default

warning: variable `A` should have a snake case name
  --> src/main.rs:10:9
   |
10 |         A => println!("A"),
   |         ^ help: convert the identifier to snake case: `a`
   |
   = note: `#[warn(non_snake_case)]` on by default

warning: variable `B` should have a snake case name
  --> src/main.rs:11:9
   |
11 |         B => println!("B"),
   |         ^ help: convert the identifier to snake case: `b`

warning: 8 warnings emitted
