rust
error[E0605]: non-primitive cast: `Struct` as `&dyn Trait`
 --> src/main.rs:9:13
  |
9 |     let _ = s as &dyn Trait; // or `let _ = s as &mut dyn Trait;`
  |             ^^^^^^^^^^^^^^^
  |
  = help: try casting `&Struct` instead: `&s as &dyn Trait`
