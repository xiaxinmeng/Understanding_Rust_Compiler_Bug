rust
extern crate diesel;
pub fn top<Table: diesel::Table +
                  diesel::query_dsl::InternalJoinDsl<
                    _, diesel::query_source::joins::Inner, _
                  >>(
    table: Table
) {}
