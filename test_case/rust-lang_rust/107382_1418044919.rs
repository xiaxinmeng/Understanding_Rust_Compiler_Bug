rust
use rkyv::{Archive, Serialize};

#[derive(Archive, Serialize, Debug)]
pub struct Record<'a> {
    payload: &'a [u8],
}
