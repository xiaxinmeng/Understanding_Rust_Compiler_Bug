
error[E0107]: missing generics for associated type `X::Y`
 --> src/test/ui/generic-associated-types/gat-trait-path-missing-lifetime.rs:5:8
  |
5 |   type Y<'a>;
  |        ^ expected 1 lifetime argument
  |
note: associated type defined here, with 1 lifetime parameter: `'a`
 --> src/test/ui/generic-associated-types/gat-trait-path-missing-lifetime.rs:5:8
  |
5 |   type Y<'a>;
  |        ^ --
help: use angle brackets to add missing lifetime argument
  |
5 |   type Y<'a><'a>;
  |         ^^^^

error: aborting due to previous error; 1 warning emitted
