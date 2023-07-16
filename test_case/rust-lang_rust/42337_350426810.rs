
error[E0433]: failed to resolve. Could not find `postgres` in `{{root}}`
 --> src/lib.rs:9:17
  |
9 | #[derive(Debug, SqlTable)]
  |                 ^^^^^^^^ Could not find `postgres` in `{{root}}`

error[E0433]: failed to resolve. Could not find `std` in `{{root}}`
 --> src/lib.rs:9:17
  |
9 | #[derive(Debug, SqlTable)]
  |                 ^^^^^^^^ Could not find `std` in `{{root}}`

error[E0412]: cannot find type `Table` in this scope
 --> src/lib.rs:9:17
  |
9 | #[derive(Debug, SqlTable)]
  |                 ^^^^^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
  |
3 | use Table;
  |
