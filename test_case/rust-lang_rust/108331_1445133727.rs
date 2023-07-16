rust
error[E0599]: the method `fetch_all` exists for struct `QueryAs<'_, MySql, T, MySqlArguments>`, but its trait bounds were not satisfied
  --> src/db/treeable.rs:70:38
   |
70 |         let mut rows: Vec<T> = query.fetch_all(conn).await?;
   |                                      ^^^^^^^^^ method cannot be called on `QueryAs<'_, MySql, T, MySqlArguments>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `T: FromRow<'r, MySqlRow>`

error[E0599]: the method `fetch_all` exists for struct `QueryAs<'_, MySql, T, MySqlArguments>`, but its trait bounds were not satisfied
   --> src/db/treeable.rs:103:38
    |
103 |         let mut rows: Vec<T> = query.fetch_all(conn).await?;
    |                                      ^^^^^^^^^ method cannot be called on `QueryAs<'_, MySql, T, MySqlArguments>` due to unsatisfied trait bounds
    |
    = note: the following trait bounds were not satisfied:
            `T: FromRow<'r, MySqlRow>`

error[E0599]: the method `fetch_all` exists for struct `QueryAs<'_, MySql, T, MySqlArguments>`, but its trait bounds were not satisfied
   --> src/db/treeable.rs:141:38
    |
141 |         let mut rows: Vec<T> = query.fetch_all(conn).await?;
    |                                      ^^^^^^^^^ method cannot be called on `QueryAs<'_, MySql, T, MySqlArguments>` due to unsatisfied trait bounds
    |
    = note: the following trait bounds were not satisfied:
            `T: FromRow<'r, MySqlRow>`

error[E0599]: the method `fetch_all` exists for struct `QueryAs<'_, MySql, T, MySqlArguments>`, but its trait bounds were not satisfied
   --> src/db/treeable.rs:177:38
    |
177 |         let mut rows: Vec<T> = query.fetch_all(conn).await?;
    |                                      ^^^^^^^^^ method cannot be called on `QueryAs<'_, MySql, T, MySqlArguments>` due to unsatisfied trait bounds
    |
    = note: the following trait bounds were not satisfied:
            `T: FromRow<'r, MySqlRow>`

error[E0599]: the method `fetch_one` exists for struct `QueryAs<'_, MySql, T, MySqlArguments>`, but its trait bounds were not satisfied
   --> src/db/treeable.rs:206:31
    |
206 |         let target: T = query.fetch_one(conn).await?;
    |                               ^^^^^^^^^ method cannot be called on `QueryAs<'_, MySql, T, MySqlArguments>` due to unsatisfied trait bounds
    |
    = note: the following trait bounds were not satisfied:
            `T: FromRow<'r, MySqlRow>`

error[E0599]: the method `fetch_one` exists for struct `QueryAs<'_, MySql, T, MySqlArguments>`, but its trait bounds were not satisfied
   --> src/db/treeable.rs:233:32
    |
233 |         let new_target = query.fetch_one(conn).await?;
    |                                ^^^^^^^^^ method cannot be called on `QueryAs<'_, MySql, T, MySqlArguments>` due to unsatisfied trait bounds
    |
    = note: the following trait bounds were not satisfied:
            `T: FromRow<'r, MySqlRow>`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `openapi_client` due to 6 previous errors
