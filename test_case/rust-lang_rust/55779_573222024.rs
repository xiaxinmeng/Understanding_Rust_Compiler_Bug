rust
// [dependencies]
// serde = "1.0"
// serde_derive = "1.0"

use serde::{Serialize, Serializer};
use serde_derive::Serialize;

struct _Local {
    range: std::ops::Range<usize>,
}

impl Serialize for _Local {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Helper {
            min: usize,
            max: usize,
        }

        let helper = Helper {
            min: self.range.start,
            max: self.range.end,
        };
        helper.serialize(serializer)
    }
}

fn main() {}
