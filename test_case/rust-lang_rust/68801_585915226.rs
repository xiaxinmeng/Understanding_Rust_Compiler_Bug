
error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> src/producers.rs:43:70
   |
43 | pub fn top<Table: diesel::Table + diesel::query_dsl::InternalJoinDsl<_, diesel::query_source::joins::Inner, _>>(table: Table, limit: usize, connection: DbConn) -> RestResult<Vec<TopWineType>> {
   |                                                                      ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> src/producers.rs:43:109
   |
43 | pub fn top<Table: diesel::Table + diesel::query_dsl::InternalJoinDsl<_, diesel::query_source::joins::Inner, _>>(table: Table, limit: usize, connection: DbConn) -> RestResult<Vec<TopWineType>> {
   |                                                                                                             ^ not allowed in type signatures

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0121`.
error: could not compile `vinoteca`.

