
error: cannot find derive macro `Serialize` in this scope
 --> src/main.rs:3:10
  |
3 | #[derive(Serialize)]
  |          ^^^^^^^^^
  |
note: `Serialize` is imported here, but it is not a derive macro
 --> src/main.rs:1:5
  |
1 | use serde::Serialize;
  |     ^^^^^^^^^^^^^^^^
