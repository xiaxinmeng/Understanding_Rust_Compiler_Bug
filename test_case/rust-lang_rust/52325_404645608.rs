
fn get_something() -> Result<f64, Error> {
    #[derive(QueryableByName)]
    struct Foo { #[sql_type = "Double"] f: f64 }
    let Foo { f } = sql_query("<my query>").get_result(conn)?;
    ...
}
