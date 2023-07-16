 rust
use std::error::Error
trait NativeSqlType;
struct Row;

pub trait FromSql<A: NativeSqlType>: Sized {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>>;
}

pub trait FromSqlRow<A: NativeSqlType>: Sized {
    fn build_from_row(row: &mut Row) -> Result<Self, Box<Error>>;
}

impl<ST, T> FromSqlRow<ST> for T where
    ST: NativeSqlType,
    T: FromSql<ST>,
{
    fn build_from_row(row: &mut Row) -> Result<Self, Box<Error>> {
        // ...
    }
}
