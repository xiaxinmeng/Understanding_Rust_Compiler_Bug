rust
pub struct UnicodePropertyV1 {}

impl<'de> serde::Deserialize<'de> for UnicodePropertyV1 {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        enum Field {}

        todo!()
    }
}
