 Rust
struct Row;

pub trait FromSql<A> {}

pub trait FromSqlRow<A> {
    fn foo(&self);
}

impl<ST, T> FromSqlRow<ST> for T where
    T: FromSql<ST>,
{
    fn foo(&self) {}
}

struct User;

impl<ST> FromSqlRow<ST> for User where
    (i32, String): FromSqlRow<ST>
{
    fn foo(&self) {}
}

fn main() {}
