
error: to use a constant of type `Opaque` in a pattern, `Opaque` must be annotated with `#[derive(PartialEq, Eq)]`
  --> src/lib.rs:10:9
   |
10 |         FOO => {},
   |         ^^^

warning: unreachable pattern
  --> src/lib.rs:11:9
   |
10 |         FOO => {},
   |         --- matches any value
11 |         _ => {}
   |         ^ unreachable pattern
   |
   = note: `#[warn(unreachable_patterns)]` on by default
