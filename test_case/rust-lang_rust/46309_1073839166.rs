rust
#[macro_use]
extern crate diesel;


use diesel::*;
use diesel::pg::{PgConnection, Pg};
use diesel::associations::HasTable;
use diesel::query_dsl::LoadQuery;
use diesel::insertable::Insertable;
use diesel::query_builder::InsertStatement;
use diesel::query_builder::UndecoratedInsertRecord;
use diesel::query_source::Table;
use diesel::result::QueryResult;



pub trait GstInsertable<'insert, T, Res, Ret = Res, E = Self> {
    fn insert(&'insert self, conn: &PgConnection) -> QueryResult<Ret>;
}

table!{
    users{
        id -> Integer,
        name-> Text,
    }
}

#[derive(Queryable)]
struct User {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
    name: &'a str,
}

impl<'a, Res> GstInsertable<'a, users::table, Res> for NewUser<'a>
    where Res: Queryable<(diesel::types::Integer, diesel::types::Text), Pg>
{
    fn insert(&'a self, conn: &PgConnection) -> QueryResult<Res> {
        ::diesel::insert_into(users::table)
            .values(self)
            .get_result(conn)
    }
}

impl<'a, C, E, T, Res> GstInsertable<'a, T, Res, Vec<Res>, E> for C
where
    C: AsRef<[E]>,
    E: GstInsertable<'a, T, Res> + 'a + UndecoratedInsertRecord<T>,
    T: Table + HasTable<Table = T>,
    for<'b> InsertStatement<T, <&'b [E] as Insertable<T>>::Values>: LoadQuery<PgConnection, Res>,
    for<'b> &'b [E]: Insertable<T> + UndecoratedInsertRecord<T>,
{
    fn insert(&'a self, conn: &PgConnection) -> QueryResult<Vec<Res>> {
        ::diesel::insert_into(T::table())
            .values(self.as_ref())
            .get_results( conn)
    }
}


fn main() {
    let conn: PgConnection = unimplemented!();
    let new_user = NewUser { name: "jon" };

    let _: User = new_user.insert(&conn).unwrap();
    let users = &vec![new_user];

    // previously wasn't working, but now seems OK
    let _: Vec<User> = users.insert(&conn).unwrap();

    // but this is
    let _: Vec<User> = GstInsertable::insert(users, &conn).unwrap();
}
