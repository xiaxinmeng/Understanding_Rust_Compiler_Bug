
error[E0710]: an unknown tool name found in scoped lint: `foo::warnings`
 --> src/lib.rs:2:8
  |
2 | #[deny(foo::warnings)] pub fn baz() { None.expect(&format!("")) }
  |        ^^^

warning: unused import: `clippy as foo`
 --> src/lib.rs:1:5
  |
1 | use clippy as foo;
  |     ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error: aborting due to previous error; 1 warning emitted
