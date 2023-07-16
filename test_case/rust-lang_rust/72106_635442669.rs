
error: `impl` item signature doesn't match `trait` item signature
  --> file8.rs:45:5
   |
41 |     fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>;
   |     ------------------------------------------------------------- expected `fn(ValueRef<'_>) -> std::result::Result<&str, FromSqlError>`
...
45 |     fn column_result(value: ValueRef<'_>) -> FromSqlResult<&str> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(ValueRef<'_>) -> std::result::Result<&str, FromSqlError>`
   |
   = note: expected `fn(ValueRef<'_>) -> std::result::Result<&str, _>`
              found `fn(ValueRef<'_>) -> std::result::Result<&str, _>`
   = help: the lifetime requirements from the `impl` do not correspond to the requirements in the `trait`
   = help: verify the lifetime relationships in the `trait` and `impl` between the `self` argument, the other inputs and its output
