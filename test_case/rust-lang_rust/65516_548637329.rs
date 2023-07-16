rust
#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(u64),
}

impl<'de> de::Deserialize<'de> for StringOrNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = StringOrNumber;

            /** .... further Visitor impl here **/
        }

        deserializer.deserialize_any(Visitor)
    }
}
