rust
use std::borrow::ToOwned;

#[derive(Clone)]
pub enum Test {
    Owned(<[Test] as ToOwned>::Owned),
}
