
error: to use a constant of type `Baonzo` in a pattern, `Baonzo` must be annotated with `#[derive(PartialEq, Eq)]`
  --> src/main.rs:14:9
   |
14 |         PUM => { println!("Here we are"); },
   |         ^^^

warning: unreachable pattern
  --> src/main.rs:15:9
   |
15 |         _ => {},
   |         ^
   |
   = note: #[warn(unreachable_patterns)] on by default

error: aborting due to previous error
