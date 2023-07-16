
$ cargo +nightly doc --all

    |
112 |                 let error_message = format!("Chrono can only represent dates up to {:?}", MAX_DATE);
    |                                                                                           ^^^^^^^^

error[E0107]: this struct takes 0 lifetime arguments but 1 lifetime argument was supplied
   --> diesel/src/query_builder/insert_statement/mod.rs:231:72
    |
231 | impl<'a, T, U, Op> ExecuteDsl<SqliteConnection> for InsertStatement<T, BatchInsert<'a, U, T>, Op>
    |                                                                        ^^^^^^^^^^^ -- help: remove this lifetime argument
    |                                                                        |
    |                                                                        expected 0 lifetime arguments
    |
note: struct defined here, with 0 lifetime parameters
   --> diesel/src/insertable.rs:222:12
    |
222 | pub struct BatchInsert {
    |            ^^^^^^^^^^^

error[E0107]: this struct takes 0 generic arguments but 2 generic arguments were supplied
   --> diesel/src/query_builder/insert_statement/mod.rs:231:72
    |
231 | impl<'a, T, U, Op> ExecuteDsl<SqliteConnection> for InsertStatement<T, BatchInsert<'a, U, T>, Op>
    |                                                                        ^^^^^^^^^^^     ---- help: remove these generic arguments
    |                                                                        |
    |                                                                        expected 0 generic arguments
    |
note: struct defined here, with 0 generic parameters
   --> diesel/src/insertable.rs:222:12
    |
222 | pub struct BatchInsert {
    |            ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0107`.
warning: `diesel` (lib) generated 5 warnings
error: could not compile `diesel` due to 2 previous errors; 5 warnings emitted
warning: build failed, waiting for other jobs to finish...
warning: `diesel` (lib) generated 5 warnings (5 duplicates)
error: could not compile `diesel` due to 2 previous errors; 5 warnings emitted
error: could not document `diesel`
