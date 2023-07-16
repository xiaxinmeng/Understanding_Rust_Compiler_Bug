rust
#[macro_use]
extern crate serde_derive;

extern crate serde;
use serde::{Deserialize, Deserializer};

use ShapeName::deserialize_shape_name;

#[derive(Debug, Deserialize)]
pub struct MyStruct {
    #[serde(deserialize_with="deserialize_shape_name")]
    pub shape: String,
}

pub trait ShapeName: Sized {
    fn deserialize_shape_name<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer;
}

impl ShapeName for String {
    fn deserialize_shape_name<D>(deserializer: D) -> Result<String, D::Error>
        where D: Deserializer
    {
        String::deserialize(deserializer)
    }
}
