rust
use serde::{Deserialize, Deserializer};

struct Type;

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Serialize)]
        struct S;
    }
}
