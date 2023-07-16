 rust
struct User {
    id: i32,
    name: String,
}

impl<ST> FromSqlRow<ST> for User where
    ST: NativeSqlType,
    (i32, String): FromSqlRow<ST>
{
    fn build_from_row(row: &mut Row) -> Result<Self, Box<Error>> {
        let (id, name) = try!(<(i32, String) as FromSqlRow<ST>>::build_from_row(row));
        Ok(User {
            id: id,
            name: name,
        })
    }
}
