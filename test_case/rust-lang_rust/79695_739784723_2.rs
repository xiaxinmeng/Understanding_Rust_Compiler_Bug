
error: mismatched types; Rust type `String` (as SQL type `TEXT`) is not compatible with SQL type `INTEGER`
 --> some_query.rs:3:15
  |
3 | sqlx::query_as!(Foo, "SELECT 10 as foo");
  | ------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
  |
