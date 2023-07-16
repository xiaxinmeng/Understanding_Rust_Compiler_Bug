
error: could not evaluate constant
 --> some_query.rs:3:15
  |
3 | sqlx::query!("SELECT 5,");
  | ------^^^^^^^^^^^^^^^^^^^-
  |       |
  |       within the query, syntax error at or near `,`
  |
