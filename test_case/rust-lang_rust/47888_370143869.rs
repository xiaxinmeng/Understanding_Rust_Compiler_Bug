rust
warning: unused imports: `Bar`, `Foo`
 --> src/main.rs:3:12
  |
3 | use test::{Bar, Foo, Trait1, Trait2, C};
  |            ^^^  ^^^
  |
  = note: #[warn(unused_imports)] on by default

warning: unused import: `Trait2`
 --> src/main.rs:3:30
  |
3 | use test::{Bar, Foo, Trait1, Trait2, C};
  |                              ^^^^^^
