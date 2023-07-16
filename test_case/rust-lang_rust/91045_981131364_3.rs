
error[E0463]: can't find crate for `bar`
 --> .\src\main.rs:1:5
  |
1 | use bar::foo;
  |     ^^^ can't find crate

error[E0463]: can't find crate for `foo`
 --> .\src\main.rs:2:5
  |
2 | use foo::bar;
  |     ^^^ can't find crate

error[E0432]: unresolved imports `bar::foo`, `foo::bar`
 --> .\src\main.rs:1:5
  |
1 | use bar::foo;
  |     ^^^^^^^^
2 | use foo::bar;
  |     ^^^^^^^^

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0432, E0463.
For more information about an error, try `rustc --explain E0432`.
