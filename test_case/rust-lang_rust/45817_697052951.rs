
error[E0404]: expected trait, found struct `A`
 --> fil7.rs:2:11
  |
2 | fn f() -> A + 'static {
  |           ^ not a trait
  |
help: `+` can be used to constrain a "trait object" type with lifetimes or auto-traits, structs and enums can't be bound in that way
 --> fil7.rs:2:11
  |
2 | fn f() -> A + 'static {
  |           ^^^^^^^^^^^
