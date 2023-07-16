rust
#![warn(explicit_outlives_requirements)]

use serde::Serialize;

#[derive(Serialize)]
pub struct Foo<'a> {
    #[serde(with = "stringify")]
    bar: &'a String,
}

mod stringify {
    pub fn serialize<S>(s: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&s)
    }
}
