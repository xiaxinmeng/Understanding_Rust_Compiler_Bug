 rust
pub enum Json {
    Number(f64),
    String(String),
    Boolean(bool),
    List(List),
    Object(Object),
    Null,
}

pub type List = Vec<Json>;
pub type Object = TreeMap<String, Json>;
