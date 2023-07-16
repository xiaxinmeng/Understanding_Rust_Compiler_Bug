rust
use serde::{Serialize, Serializer};

struct Type;

impl Serialize for Type {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        #[derive(Serialize)]
        struct S;
    }
}
