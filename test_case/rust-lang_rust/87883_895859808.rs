rust 
// Cargo.toml smallvec dependency:
// smallvec = { version = "*", features = ["union", "may_dangle"] }

use std::{mem, num::NonZeroUsize};

use smallvec::SmallVec;

pub struct Size {
    raw: u64,
}

pub enum FieldsShapeVec {
    Primitive,
    Union(NonZeroUsize),
    Array {
        stride: Size,
        count: u64,
    },
    Arbitrary {
        offsets: Vec<Size>,
        memory_index: Vec<u32>,
    },
}

pub enum FieldsShapeSmallVec {
    Primitive,
    Union(NonZeroUsize),
    Array {
        stride: Size,
        count: u64,
    },
    Arbitrary {
        offsets: SmallVec<[Size; 2]>,
        memory_index: SmallVec<[u32; 4]>,
    },
}

fn main() {
    assert_eq!(
        mem::size_of::<FieldsShapeVec>(),
        mem::size_of::<FieldsShapeSmallVec>()
    );
}
