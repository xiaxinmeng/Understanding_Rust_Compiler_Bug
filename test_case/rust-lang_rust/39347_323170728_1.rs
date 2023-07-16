rust
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

table!(
    sessions(id) {
        id -> Integer,
        user_id -> Integer,
    }
);
table!(
    users(id) {
        id -> Integer,
    }
);

#[derive(Debug, Clone, Eq, PartialEq, Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct Session {
    id: i64,
    pub user_id: i64,
}


#[derive(Debug, Clone, Eq, PartialEq, Identifiable, Queryable)]
pub struct User {
    pub id: i64,
}

fn main() {}
