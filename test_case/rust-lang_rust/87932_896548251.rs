rust
pub struct Up;

impl<'de> serde::Deserialize<'de> for Up {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const _A: () = {
            extern crate serde as _serde; // maybe this one causing the overflow?
        };
        Ok(Up)
    }
}

fn main() {}
