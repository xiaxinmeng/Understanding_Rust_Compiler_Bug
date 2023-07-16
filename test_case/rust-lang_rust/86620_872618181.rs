rust
// lib.rs
use ppv_lite86::*;

pub struct S;

impl MultiLane<usize> for S {
    fn from_lanes(lanes: usize) -> Self {
        todo!();
    }
    fn to_lanes(self) -> usize {
        todo!()
    }
}
