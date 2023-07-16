 rust
#[db_automap]
struct Person {
    #[primary_key]
    id: i64,
    first_name: String,
    #[db(name="last")]
    last_name: String,
}
