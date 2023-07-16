
error[E0432]: unresolved import `Foo`
 --> file7.rs:8:5
  |
8 | use Foo::Foo;
  |     ^^^ `Foo` is a variant, not a module

warning: unused import: `foo::*`
 --> file7.rs:7:5
  |
7 | use foo::*;
  |     ^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0432, E0601.
For more information about an error, try `rustc --explain E0432`.
