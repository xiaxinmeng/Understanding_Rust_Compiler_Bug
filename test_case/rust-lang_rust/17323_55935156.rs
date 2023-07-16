 rust
pub enum Json {
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Boolean(bool),
    List(List),
    Object(Object),
    Null,
}
pub type List = Vec<Json>;
pub type Object = TreeMap<String, Json>;
