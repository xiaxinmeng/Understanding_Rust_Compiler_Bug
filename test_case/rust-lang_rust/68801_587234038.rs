rust
#[macro_use]
extern crate diesel;

mod schema {
    table! {
        producers (id) {
            id -> Integer,
        }
    }

    table! {
        purchases (id) {
            id -> Integer,
            wine_id -> Integer,
        }
    }

    table! {
        wines (id) {
            id -> Integer,
            producer_id -> Integer,
        }
    }

    joinable!(purchases -> wines (wine_id));

    allow_tables_to_appear_in_same_query!(
        producers,
        purchases,
        wines,
    );
}

mod model {
    use crate::schema::{purchases, wines};
    use diesel::prelude::*;

    pub fn top<Table: diesel::Table +
                      diesel::query_dsl::InternalJoinDsl<
                        _, diesel::query_source::joins::Inner, _
                      >>(
        table: Table
    ) {
        table.inner_join(wines::table.inner_join(purchases::table))
    }
}
