rust
union DummyUnion {
    field1: u32,
    field2: u32,
}

pub fn init() -> u32 {
    const UNION: DummyUnion = DummyUnion { field1: 0 };
    const FIELD: u32 = unsafe { UNION.field1 };
    FIELD
}

fn main() {
    init();
}
