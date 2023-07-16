
warning[E0170]: pattern binding `Eof` is named the same as one of the variants of the type `Type`
  --> src/main.rs:30:6
   |
30 |      Eof => {println!("hi");},
   |      ^^^ help: to match on the variant, qualify the path: `Type::Eof`
   |
   = note: `#[warn(bindings_with_variant_name)]` on by default

warning: unused variable: `Eof`
  --> src/main.rs:30:6
   |
30 |      Eof => {println!("hi");},
   |      ^^^ help: if this is intentional, prefix it with an underscore: `_Eof`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variable `Eof` should have a snake case name
  --> src/main.rs:30:6
   |
30 |      Eof => {println!("hi");},
   |      ^^^ help: convert the identifier to snake case: `eof`
   |
   = note: `#[warn(non_snake_case)]` on by default
