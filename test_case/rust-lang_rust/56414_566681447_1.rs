
error[E0659]: `users` is ambiguous (name vs any other name during import resolution)
 --> src/main.rs:9:9
  |
9 |     use users::dsl::*;
  |         ^^^^^ ambiguous name
  |
note: `users` could refer to the struct imported here
 --> src/main.rs:9:9
  |
9 |     use users::dsl::*;
  |         ^^^^^^^^^^^^^
note: `users` could also refer to the module defined here
 --> src/main.rs:1:1
  |
1 | / mod users {
2 | |     pub struct table;
3 | |     pub mod dsl {
4 | |         pub use super::table as users;
5 | |     }
6 | | }
  | |_^
